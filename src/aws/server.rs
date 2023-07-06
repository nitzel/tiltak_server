use crate::aws::{AwsWrappedEvent, Event, Output, TimeControl, Action};
use crate::position::{Komi, Position};
use crate::search::MctsSetting;
use crate::search::{self, MonteCarloTree};
use board_game_traits::{GameResult, Position as EvalPosition};
use lambda_runtime::Context;
use pgn_traits::PgnPosition;
use std::convert::TryFrom;
use std::time::{Duration, Instant};

type Error = Box<dyn std::error::Error + Sync + Send>;


/// AWS serverside handler
pub async fn handle_aws_event(e: AwsWrappedEvent, c: Context) -> Result<Output, Error> {
    println!("input={}", e.body.replace("\r", " ").replace("\n", "")); // log query to AWS CloudWatch
    let e: Event = serde_json::from_str(&e.body)?;
    if let Some(tps) = &e.tps {
        println!("tps={};komi={};", tps, e.komi); // log query to AWS CloudWatch
    } else {
        println!("moves={};komi={};", e.moves.join(" "), e.komi); // log query to AWS CloudWatch
    }
    match e.size {
        4 => handle_aws_event_generic::<4>(e, c),
        5 => handle_aws_event_generic::<5>(e, c),
        6 => handle_aws_event_generic::<6>(e, c),
        s => panic!("Unsupported board size {}", s),
    }
}

pub fn handle_aws_event_generic<const S: usize>(e: Event, _c: Context) -> Result<Output, Error> {
    let komi = Komi::try_from(e.komi)?;
    let mut position = match e.tps {
        Some(tps) => <Position<S>>::from_fen_with_komi(&tps, komi)?,
        None => <Position<S>>::start_position_with_komi(komi),
    };
    for move_string in e.moves {
        let mv = position.move_from_san(&move_string)?;
        let mut legal_moves = vec![];
        position.generate_moves(&mut legal_moves);
        if !legal_moves.contains(&mv) {
            return Err(format!("Illegal {}s move {}", S, move_string).into());
        }
        position.do_move(mv);
    }

    match position.game_result() {
        Some(GameResult::Draw) => {
            return Ok(Output::SuggestMove {
                score: 0.5,
                // defaults
                pv: Vec::new(),
                nodes: 0,
                mem_usage: 0,
                time_taken: Duration::default(),
            })
        }
        Some(result) if result == GameResult::win_by(position.side_to_move()) => {
            return Ok(Output::SuggestMove {
                score: 0.0,
                // defaults
                pv: Vec::new(),
                nodes: 0,
                mem_usage: 0,
                time_taken: Duration::default(),
            })
        }
        Some(_) => {
            return Ok(Output::SuggestMove {
                score: 1.0,
                // defaults
                pv: Vec::new(),
                nodes: 0,
                mem_usage: 0,
                time_taken: Duration::default(),
            })
        }
        None => (),
    }

    let settings = if let Some(dirichlet) = e.dirichlet_noise {
        MctsSetting::default().add_dirichlet(dirichlet)
    } else {
        MctsSetting::default()
    }
    .add_rollout_depth(e.rollout_depth)
    .add_rollout_temperature(e.rollout_temperature)
    .mem_usage(2_usize.pow(30));

    let start_time = Instant::now();
    match e.action {
        Action::SuggestMoves => {
            match e.time_control {
                TimeControl::Time(time_left, _increment) => {
                    let max_time = time_left;
                    let mut tree = MonteCarloTree::with_settings(position, settings);
                    tree.search_for_time(max_time, |_| {});

                    let best_children = tree.best_children_info(10);

                    Ok(Output::SuggestMoves(best_children))
                }
                TimeControl::FixedNodes(nodes) => {
                    let mut tree = search::MonteCarloTree::with_settings(position, settings);
                    for _ in 0..nodes {
                        if tree.select().is_none() {
                            eprintln!("Warning: Search stopped early due to OOM");
                            break;
                        };
                    }
                    let best_children = tree.best_children_info(10);
                    Ok(Output::SuggestMoves(best_children))
                }
            }
        }
        Action::SuggestMove => {
            match e.time_control {
                TimeControl::Time(time_left, increment) => {
                    let max_time = if position.half_moves_played() < 4 {
                        Duration::min(time_left / 80 + increment / 6, Duration::from_secs(40))
                    } else {
                        Duration::min(time_left / 40 + increment / 3, Duration::from_secs(40))
                    };

                    let mut tree = MonteCarloTree::with_settings(position, settings);
                    tree.search_for_time(max_time, |_| {});

                    let score = 1.0 - tree.best_move().1;
                    let pv = tree.pv().map(|mv| mv.to_string::<S>()).collect();
                    Ok(Output::SuggestMove {
                        pv,
                        score,
                        nodes: tree.visits(),
                        mem_usage: tree.mem_usage() as u64,
                        time_taken: start_time.elapsed(),
                    })
                }
                TimeControl::FixedNodes(nodes) => {
                    let mut tree = search::MonteCarloTree::with_settings(position, settings);
                    for _ in 0..nodes {
                        if tree.select().is_none() {
                            eprintln!("Warning: Search stopped early due to OOM");
                            break;
                        };
                    }
                    let score = 1.0 - tree.best_move().1;
                    let pv = tree.pv().map(|mv| mv.to_string::<S>()).collect();
                    Ok(Output::SuggestMove {
                        pv,
                        score,
                        nodes: tree.visits(),
                        mem_usage: tree.mem_usage() as u64,
                        time_taken: start_time.elapsed(),
                    })
                }
            }
        }
    }
}
