use std::convert::TryFrom;
use std::time::Duration;
use clap::{App, Arg};

use board_game_traits::{Position as PositionTrait};

use tiltak::evaluation::{parameters, value_eval};
use tiltak::position::{Komi, Move};
use tiltak::position::Position;
use tiltak::search::{self, MctsSetting, MoveInfo};

#[cfg(test)]
mod tests;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PtnMoveResult {
    ptn: String,
    white: isize,
    black: isize,
}

impl From<&MoveInfo> for PtnMoveResult {
    fn from(value: &MoveInfo) -> Self {
        PtnMoveResult{
            ptn: value.mv.clone(),
            white: ((value.winning_probability) * 100.) as isize,
            black: ((1.-value.winning_probability) * 100.) as isize,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct OpeningResult {
    black: isize,
    white: isize,
    moves: Vec<PtnMoveResult>,

    _debug: Vec<MoveInfo>, // do not rely on this
}
impl From<&Vec<MoveInfo>> for OpeningResult {
    fn from(value: &Vec<MoveInfo>) -> Self {
        let moves: Vec<PtnMoveResult> = value.iter().map(PtnMoveResult::from).collect();
        let total_white = moves.iter().fold(0, |sum, mv|sum+mv.white);
        let total_black = moves.iter().fold(0, |sum, mv|sum+mv.black);

        OpeningResult{
            moves,
            white: total_white,
            black: total_black,
            _debug: value.clone(),
        }
    }
}
/**
 * Determines the size from a tps string.
 * [TPS](https://ustak.org/tak-positional-system-tps/) separtes rows with `/` and columns with `,`.
 * If `n` adjacent columns are empty, they're merged into `xn`, e.g. `x5` for a completely empty row on `5x5`.
 */
fn determine_tps_size(tps: &str) -> usize{
    let (first_row_tps, _) = tps.split_once("/").unwrap();
    first_row_tps.split(",").fold(0, |column_count: usize, tps_segment: &str| {
        if tps_segment.starts_with("x") {

            return column_count + tps_segment.trim_start_matches("x").parse::<usize>().unwrap_or(1);
        }
        return column_count + 1;
    })
}

fn cors(response: rouille::Response) -> rouille::Response{
    response
        .with_additional_header("Access-Control-Allow-Origin", "*")
        .with_additional_header("Access-Control-Allow-Methods", "*")
        .with_additional_header("Access-Control-Allow-Headers", "*")
}

fn response_text(text: &str) -> rouille::Response{
    cors(rouille::Response::text(text))
}

fn main() {
    let app = App::new("Tiltak API Server")
        .version("0.1")
        .author("Morten Lohne(Tiltak), nitzel(API server)")
        // .arg(
        //     Arg::with_name("logfile")
        //         .short("l")
        //         .long("logfile")
        //         .value_name("tiltak.log")
        //         .help("Name of debug logfile")
        //         .takes_value(true),
        // )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("port")
                .help("Port to listen on")
                .takes_value(true)
                .default_value("3000"),
        )
        .arg(
            Arg::with_name("host")
                .long("host")
                .value_name("host")
                .help("IP or hostname to listen on")
                .takes_value(true)
                .default_value("0.0.0.0"),
        )
        .arg(
            Arg::with_name("timeout")
                .long("timeout")
                .short("t")
                .value_name("timeout")
                .help("How many seconds of thinking time until results are returned")
                .takes_value(true)
                .default_value("2.0"),
        );

    let matches = app.get_matches();
    let timeout: f64 = matches.value_of("timeout").unwrap().parse().unwrap();
    let port: usize = matches.value_of("port").unwrap().parse().unwrap();
    let host: String = matches.value_of("host").unwrap().parse().unwrap();
    let default_move_count = 5; // number of evaluated moves returned for a request
    let max_move_count = 15;

    let host_address = format!("{}:{}", host, port);
    println!("Tiltak Web API starting on {} with timeout={}s", host_address, timeout);

    rouille::start_server(host_address, move |request| {
        rouille::router!(request,
            (GET) (/api/v1/ping) => {
                response_text("pong")
            },
            (GET) (/api/v1/best_move/{tps: String}) => {
                let komi: Option<f32> = request.get_param("komi").and_then(|s|s.parse::<f32>().ok());
                let komi = Komi::try_from(komi.unwrap_or_default());
                if let Err(err) = komi {
                    return rouille::Response::text(err).with_status_code(400)
                }
                let komi = komi.unwrap();
                
                let move_count;
                if let Some(move_count_str) = request.get_param("count") {
                    match move_count_str.parse() {
                        Ok(n) => move_count = n,
                        Err(err) => {
                            return response_text(&format!("Failed to parse count '{}': {:?}", move_count_str, err))
                                .with_status_code(400)
                        }
                    };
                } else {
                    move_count = default_move_count;
                }
                if move_count > max_move_count {
                    return response_text(&format!("Move count {} is too high, please choose <= {}", move_count, max_move_count))
                        .with_status_code(400);
                }

                println!("# best_move komi={} tps={}", komi, tps);

                let size = determine_tps_size(&tps);
                println!("derived board size={}", size);
                let best_children = analyze_position_from_tps_size(size, komi, timeout, &tps, move_count);

                // Builds a `Response` object that contains the "hello world" text.
                let best_children_info = best_children.iter().map(|x|x.to_string()).collect::<Vec<String>>().join("\n");
                println!("{}", best_children_info);

                cors(rouille::Response::json(&OpeningResult::from(&best_children)))
            },
            _ => response_text("no such thing, try /api/v1/best_move/{tps}?komi=2&count=5").with_status_code(404)
        )
    });
}

fn analyze_position_from_tps_size(size: usize, komi: Komi, timeout_sec: f64, tps: &String, move_count: usize) -> Vec<MoveInfo> {
    match size {
        4 => analyze_position_from_tps::<4>(komi, timeout_sec, tps, move_count),
        5 => analyze_position_from_tps::<5>(komi, timeout_sec, tps, move_count),
        6 => analyze_position_from_tps::<6>(komi, timeout_sec, tps, move_count),
        7 => analyze_position_from_tps::<7>(komi, timeout_sec, tps, move_count),
        8 => analyze_position_from_tps::<8>(komi, timeout_sec, tps, move_count),
        // TODO: Handle error with HTTP response/return Result
        s => panic!("Unsupported size {}", s),
    }
}

fn analyze_position_from_tps<const S: usize>(komi: Komi, timeout_sec: f64, tps: &String, move_count: usize) -> Vec<MoveInfo> {
    let position = <Position<S>>::from_fen_with_komi(tps, komi).unwrap();
    analyze_position(&position, Some(timeout_sec), move_count)
}

fn analyze_position<const S: usize>(position: &Position<S>, timeout_sec: Option<f64>, move_count: usize) -> Vec<MoveInfo> {
    if position.game_result() != None { // game over
        return vec![];
    }

    let group_data = position.group_data();

    let mut coefficients = vec![0.0; parameters::num_value_features::<S>()];
    {
        let mut value_features = parameters::ValueFeatures::new::<S>(&mut coefficients);
        value_eval::static_eval_game_phase::<S>(position, &group_data, &mut value_features);
    }
    for (feature, param) in coefficients.iter_mut().zip(<Position<S>>::value_params()) {
        *feature *= param;
    }

    let mut simple_moves = vec![];
    let mut moves = vec![];

    position.generate_moves_with_probabilities(
        &position.group_data(),
        &mut simple_moves,
        &mut moves,
        &mut vec![],
    );
    moves.sort_by(|(_mv, score1), (_, score2)| score1.partial_cmp(score2).unwrap().reverse());

    let mut feature_sets = vec![vec![0.0; parameters::num_policy_features::<S>()]; moves.len()];
    let mut policy_feature_sets: Vec<_> = feature_sets
        .iter_mut()
        .map(|feature_set| parameters::PolicyFeatures::new::<S>(feature_set))
        .collect();

    let simple_moves: Vec<Move> = moves.iter().map(|(mv, _)| mv.clone()).collect();

    position.features_for_moves(&mut policy_feature_sets, &simple_moves, &group_data);

    let settings: MctsSetting<S> = search::MctsSetting::default()
        .arena_size(2_u32.pow(26)) // 2**26 equals 1.5GB max according to Morten
        .exclude_moves(vec![]);

    let mut tree = search::MonteCarloTree::with_settings(position.clone(), settings);
    tree.search_for_time(Duration::from_secs_f64(timeout_sec.unwrap()), |_| {});

    let move_count = std::cmp::min(move_count, moves.len());
    let best_children = tree.best_children_info(move_count);
    return best_children;
}
