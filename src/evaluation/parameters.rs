use crate::position::num_square_symmetries;

pub const NUM_VALUE_FEATURES_4S: usize = 73;
pub const NUM_POLICY_FEATURES_4S: usize = 130;

pub const NUM_VALUE_FEATURES_5S: usize = 91;
pub const NUM_POLICY_FEATURES_5S: usize = 149;

pub const NUM_VALUE_FEATURES_6S: usize = 94;
pub const NUM_POLICY_FEATURES_6S: usize = 159;

#[derive(Debug)]
pub struct ValueFeatures<'a> {
    pub flat_psqt: &'a mut [f32],
    pub wall_psqt: &'a mut [f32],
    pub cap_psqt: &'a mut [f32],
    pub supports_psqt: &'a mut [f32],
    pub captives_psqt: &'a mut [f32],
    pub shallow_supports_per_piece: &'a mut [f32],
    pub deep_supports_per_piece: &'a mut [f32],
    pub shallow_captives_per_piece: &'a mut [f32],
    pub deep_captives_per_piece: &'a mut [f32],
    pub side_to_move: &'a mut [f32],
    pub flatstone_lead: &'a mut [f32],
    pub i_number_of_groups: &'a mut [f32],
    pub critical_squares: &'a mut [f32],
    pub flat_next_to_our_stack: &'a mut [f32],
    pub wall_next_to_our_stack: &'a mut [f32],
    pub cap_next_to_our_stack: &'a mut [f32],
    pub num_lines_occupied: &'a mut [f32],
    pub line_control: &'a mut [f32],
    pub block_their_line: &'a mut [f32],
    pub sidelined_cap: &'a mut [f32],
    pub fully_isolated_cap: &'a mut [f32],
    pub semi_isolated_cap: &'a mut [f32],
}

impl<'a> ValueFeatures<'a> {
    pub fn new<const S: usize>(coefficients: &'a mut [f32]) -> Self {
        assert_eq!(coefficients.len(), num_value_features::<S>());

        let (flat_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (wall_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (cap_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (supports_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (captives_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (shallow_supports_per_piece, coefficients) = coefficients.split_at_mut(4);
        let (deep_supports_per_piece, coefficients) = coefficients.split_at_mut(4);
        let (shallow_captives_per_piece, coefficients) = coefficients.split_at_mut(4);
        let (deep_captives_per_piece, coefficients) = coefficients.split_at_mut(4);
        let (side_to_move, coefficients) = coefficients.split_at_mut(3);
        let (flatstone_lead, coefficients) = coefficients.split_at_mut(3);
        let (i_number_of_groups, coefficients) = coefficients.split_at_mut(3);
        let (critical_squares, coefficients) = coefficients.split_at_mut(6);
        let (flat_next_to_our_stack, coefficients) = coefficients.split_at_mut(1);
        let (wall_next_to_our_stack, coefficients) = coefficients.split_at_mut(1);
        let (cap_next_to_our_stack, coefficients) = coefficients.split_at_mut(1);
        let (num_lines_occupied, coefficients) = coefficients.split_at_mut(S + 1);
        let (line_control, coefficients) = coefficients.split_at_mut(S + 1);
        let (block_their_line, coefficients) = coefficients.split_at_mut(S + 1);
        let (sidelined_cap, coefficients) = coefficients.split_at_mut(3);
        let (fully_isolated_cap, coefficients) = coefficients.split_at_mut(3);
        let (semi_isolated_cap, coefficients) = coefficients.split_at_mut(3);

        assert!(coefficients.is_empty());

        ValueFeatures {
            flat_psqt,
            wall_psqt,
            cap_psqt,
            supports_psqt,
            captives_psqt,
            shallow_supports_per_piece,
            deep_supports_per_piece,
            shallow_captives_per_piece,
            deep_captives_per_piece,
            side_to_move,
            flatstone_lead,
            i_number_of_groups,
            critical_squares,
            flat_next_to_our_stack,
            wall_next_to_our_stack,
            cap_next_to_our_stack,
            num_lines_occupied,
            line_control,
            block_their_line,
            sidelined_cap,
            fully_isolated_cap,
            semi_isolated_cap,
        }
    }
}

#[derive(Debug)]
pub struct PolicyFeatures<'a> {
    pub decline_win: &'a mut [f32],
    pub place_to_win: &'a mut [f32],
    pub place_to_draw: &'a mut [f32],
    pub place_to_loss: &'a mut [f32],
    pub place_to_allow_opponent_to_end: &'a mut [f32],
    pub two_flats_left: &'a mut [f32],
    pub three_flats_left: &'a mut [f32],
    pub flat_psqt: &'a mut [f32],
    pub wall_psqt: &'a mut [f32],
    pub cap_psqt: &'a mut [f32],
    pub our_road_stones_in_line: &'a mut [f32],
    pub their_road_stones_in_line: &'a mut [f32],
    pub extend_single_group_base: &'a mut [f32],
    pub extend_single_group_linear: &'a mut [f32],
    pub extend_single_group_to_new_line_base: &'a mut [f32],
    pub extend_single_group_to_new_line_linear: &'a mut [f32],
    pub merge_two_groups_base: &'a mut [f32],
    pub merge_two_groups_linear: &'a mut [f32],
    pub block_merger_base: &'a mut [f32],
    pub block_merger_linear: &'a mut [f32],
    pub place_our_critical_square: &'a mut [f32],
    pub place_their_critical_square: &'a mut [f32],
    pub ignore_their_critical_square: &'a mut [f32],
    pub next_to_our_last_stone: &'a mut [f32],
    pub next_to_their_last_stone: &'a mut [f32],
    pub diagonal_to_our_last_stone: &'a mut [f32],
    pub diagonal_to_their_last_stone: &'a mut [f32],
    pub attack_strong_flats: &'a mut [f32],
    pub blocking_stone_blocks_extensions_of_two_flats: &'a mut [f32],
    pub attack_strong_stack_with_wall: &'a mut [f32],
    pub attack_strong_stack_with_cap: &'a mut [f32],
    pub attack_last_movement: &'a mut [f32],
    pub place_last_movement: &'a mut [f32],
    pub move_role_bonus: &'a mut [f32],
    pub stack_movement_that_gives_us_top_pieces: &'a mut [f32],
    pub stack_captured_by_movement: &'a mut [f32],
    pub stack_capture_in_strong_line: &'a mut [f32],
    pub stack_capture_in_strong_line_cap: &'a mut [f32],
    pub move_cap_onto_strong_line: &'a mut [f32],
    pub move_cap_onto_strong_line_with_critical_square: &'a mut [f32],
    pub recapture_stack_pure: &'a mut [f32],
    pub recapture_stack_impure: &'a mut [f32],
    pub move_last_placement: &'a mut [f32],
    pub continue_spread: &'a mut [f32],
    pub move_onto_critical_square: &'a mut [f32],
    pub spread_that_connects_groups_to_win: &'a mut [f32],
}

impl<'a> PolicyFeatures<'a> {
    #[inline(never)]
    pub fn new<const S: usize>(coefficients: &'a mut [f32]) -> PolicyFeatures<'a> {
        assert_eq!(coefficients.len(), num_policy_features::<S>());

        let (decline_win, coefficients) = coefficients.split_at_mut(1);
        let (place_to_win, coefficients) = coefficients.split_at_mut(1);
        let (place_to_draw, coefficients) = coefficients.split_at_mut(1);
        let (place_to_loss, coefficients) = coefficients.split_at_mut(1);
        let (place_to_allow_opponent_to_end, coefficients) = coefficients.split_at_mut(3);
        let (two_flats_left, coefficients) = coefficients.split_at_mut(2);
        let (three_flats_left, coefficients) = coefficients.split_at_mut(2);
        let (flat_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (wall_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (cap_psqt, coefficients) = coefficients.split_at_mut(num_square_symmetries::<S>());
        let (our_road_stones_in_line, coefficients) = coefficients.split_at_mut(S * 3);
        let (their_road_stones_in_line, coefficients) = coefficients.split_at_mut(S * 3);
        let (extend_single_group_to_new_line_base, coefficients) = coefficients.split_at_mut(3);
        let (extend_single_group_to_new_line_linear, coefficients) = coefficients.split_at_mut(3);
        let (extend_single_group_base, coefficients) = coefficients.split_at_mut(3);
        let (extend_single_group_linear, coefficients) = coefficients.split_at_mut(3);
        let (merge_two_groups_base, coefficients) = coefficients.split_at_mut(3);
        let (merge_two_groups_linear, coefficients) = coefficients.split_at_mut(3);
        let (block_merger_base, coefficients) = coefficients.split_at_mut(3);
        let (block_merger_linear, coefficients) = coefficients.split_at_mut(3);
        let (place_our_critical_square, coefficients) = coefficients.split_at_mut(1);
        let (place_their_critical_square, coefficients) = coefficients.split_at_mut(4);
        let (ignore_their_critical_square, coefficients) = coefficients.split_at_mut(2);
        let (next_to_our_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (next_to_their_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (diagonal_to_our_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (diagonal_to_their_last_stone, coefficients) = coefficients.split_at_mut(1);
        let (attack_strong_flats, coefficients) = coefficients.split_at_mut(1);
        let (blocking_stone_blocks_extensions_of_two_flats, coefficients) =
            coefficients.split_at_mut(1);
        let (attack_strong_stack_with_wall, coefficients) = coefficients.split_at_mut(6);
        let (attack_strong_stack_with_cap, coefficients) = coefficients.split_at_mut(6);
        let (attack_last_movement, coefficients) = coefficients.split_at_mut(4);
        let (place_last_movement, coefficients) = coefficients.split_at_mut(3);
        let (move_role_bonus, coefficients) = coefficients.split_at_mut(3);
        let (stack_movement_that_gives_us_top_pieces, coefficients) = coefficients.split_at_mut(6);
        let (stack_captured_by_movement, coefficients) = coefficients.split_at_mut(1);
        let (stack_capture_in_strong_line, coefficients) = coefficients.split_at_mut(S - 3);
        let (stack_capture_in_strong_line_cap, coefficients) = coefficients.split_at_mut(S - 3);
        let (move_cap_onto_strong_line, coefficients) = coefficients.split_at_mut(S - 3);
        let (move_cap_onto_strong_line_with_critical_square, coefficients) =
            coefficients.split_at_mut(S - 3);
        let (recapture_stack_pure, coefficients) = coefficients.split_at_mut(3);
        let (recapture_stack_impure, coefficients) = coefficients.split_at_mut(3);
        let (move_last_placement, coefficients) = coefficients.split_at_mut(3);
        let (continue_spread, coefficients) = coefficients.split_at_mut(3);
        let (move_onto_critical_square, coefficients) = coefficients.split_at_mut(3);
        let (spread_that_connects_groups_to_win, coefficients) = coefficients.split_at_mut(1);

        assert!(coefficients.is_empty());

        PolicyFeatures {
            decline_win,
            place_to_win,
            place_to_draw,
            place_to_loss,
            place_to_allow_opponent_to_end,
            two_flats_left,
            three_flats_left,
            flat_psqt,
            wall_psqt,
            cap_psqt,
            our_road_stones_in_line,
            their_road_stones_in_line,
            extend_single_group_base,
            extend_single_group_linear,
            extend_single_group_to_new_line_base,
            extend_single_group_to_new_line_linear,
            merge_two_groups_base,
            merge_two_groups_linear,
            block_merger_base,
            block_merger_linear,
            place_our_critical_square,
            place_their_critical_square,
            ignore_their_critical_square,
            next_to_our_last_stone,
            next_to_their_last_stone,
            diagonal_to_our_last_stone,
            diagonal_to_their_last_stone,
            attack_strong_flats,
            blocking_stone_blocks_extensions_of_two_flats,
            attack_strong_stack_with_wall,
            attack_strong_stack_with_cap,
            attack_last_movement,
            place_last_movement,
            move_role_bonus,
            stack_movement_that_gives_us_top_pieces,
            stack_captured_by_movement,
            stack_capture_in_strong_line,
            stack_capture_in_strong_line_cap,
            move_cap_onto_strong_line,
            move_cap_onto_strong_line_with_critical_square,
            recapture_stack_pure,
            recapture_stack_impure,
            move_last_placement,
            continue_spread,
            move_onto_critical_square,
            spread_that_connects_groups_to_win,
        }
    }
}

pub fn num_value_features<const S: usize>() -> usize {
    match S {
        4 => NUM_VALUE_FEATURES_4S,
        5 => NUM_VALUE_FEATURES_5S,
        6 => NUM_VALUE_FEATURES_6S,
        _ => unimplemented!(),
    }
}

pub fn num_policy_features<const S: usize>() -> usize {
    match S {
        4 => NUM_POLICY_FEATURES_4S,
        5 => NUM_POLICY_FEATURES_5S,
        6 => NUM_POLICY_FEATURES_6S,
        _ => unimplemented!(),
    }
}

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_4S: [f32; NUM_VALUE_FEATURES_4S] = [
    0.54923415,
    0.7282322,
    1.0255456,
    1.583941,
    2.0131814,
    2.5839906,
    0.002932434,
    -0.0025715088,
    -0.00047107227,
    0.94568884,
    1.0339974,
    1.2408891,
    0.59685516,
    0.7428497,
    0.9859054,
    1.027361,
    1.5435753,
    0.009879224,
    -0.0059808395,
    0.43253043,
    0.21758828,
    0.005999675,
    -0.0073812627,
    -0.67089164,
    -0.26996073,
    -0.008598236,
    0.0018530292,
    -0.678351,
    -0.71593153,
    0.005024814,
    0.0038007405,
    1.7668045,
    1.7301219,
    2.287093,
    0.6275563,
    0.26913106,
    1.4216075,
    -0.17621191,
    0.09113373,
    0.07649631,
    0.34329876,
    0.02262616,
    0.082917236,
    -0.058202233,
    -0.0047925757,
    -0.005560641,
    0.01647186,
    -0.25089246,
    -0.0082427785,
    1.1404806,
    -1.4436735,
    -0.7150899,
    0.06830657,
    0.93882126,
    -1.1160055,
    -0.808328,
    0.33283132,
    1.5984336,
    -0.0060374904,
    -0.017376488,
    0.0060048904,
    0.07591783,
    0.0022945618,
    0.0074333027,
    0.0074355192,
    0.0020511579,
    -0.0041063665,
    -0.00496279,
    0.007571606,
    0.009635687,
    -0.0014058612,
    -0.0038658213,
    -0.00062850676,
];

#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_4S: [f32; NUM_POLICY_FEATURES_4S] = [
    -3.9657896,
    1.4110678,
    1.0998982,
    -3.1612859,
    -3.3797283,
    0.22887644,
    0.46740365,
    -0.25038686,
    0.19303164,
    -0.16947459,
    0.15422852,
    0.06293144,
    0.16235097,
    0.43227813,
    -0.38303038,
    -0.504268,
    0.21411678,
    0.007571606,
    0.009635687,
    -0.0014058612,
    0.0085868295,
    -0.27475312,
    0.46439883,
    1.078042,
    0.020157898,
    0.14288536,
    0.055918362,
    -1.5474207,
    0.0061870757,
    -0.0077467104,
    -0.007937893,
    0.0076041985,
    0.14159878,
    -0.38050216,
    0.3239574,
    1.2015599,
    -0.7297661,
    -0.6360258,
    -0.5853736,
    0.6284847,
    -0.0057374667,
    0.00015021767,
    0.008557079,
    -0.0009557083,
    0.6104631,
    -0.5925414,
    -0.009191279,
    -1.3853506,
    0.5065972,
    -0.00008111913,
    0.45035902,
    0.8281232,
    -0.008808966,
    0.14468262,
    -0.81799924,
    -0.0049592447,
    0.8167471,
    1.2574946,
    0.002150652,
    0.1963947,
    -0.7948367,
    -0.004483435,
    0.17575556,
    0.4369178,
    0.008150065,
    -0.84050864,
    -0.34566236,
    0.0040705632,
    2.4563792,
    0.23108627,
    0.9188401,
    0.008161059,
    0.79233116,
    -3.536681,
    -1.7433494,
    0.24274868,
    1.396689,
    0.43048444,
    0.28698713,
    0.27532858,
    0.58915645,
    0.18701936,
    0.33074096,
    -0.3476396,
    0.54176253,
    0.008410923,
    0.0043121455,
    -0.008000903,
    0.0010137418,
    -0.006815267,
    0.0031001903,
    -0.002478423,
    -0.002353027,
    0.35292476,
    0.07396798,
    -0.007463541,
    -0.009312019,
    0.06629895,
    -0.21437466,
    0.0035254955,
    -1.3564991,
    -1.2744789,
    0.008508706,
    -0.04956134,
    0.8928778,
    -1.5557747,
    0.62927735,
    -1.0346689,
    0.24117747,
    0.5185278,
    -0.20043422,
    -0.005868571,
    -0.0075792954,
    0.00952702,
    1.4246035,
    0.761231,
    -0.0077517964,
    -1.1132826,
    0.28384545,
    -0.0027733874,
    0.427133,
    1.0458082,
    0.001523626,
    0.120106295,
    -0.09654278,
    -0.0018659113,
    0.7107393,
    2.2507515,
    0.70542514,
    2.967115,
];

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_5S: [f32; NUM_VALUE_FEATURES_5S] = [
    -0.01766367,
    0.13190418,
    0.15378818,
    0.31723237,
    0.28847328,
    0.22101857,
    0.9498503,
    1.2208939,
    1.280625,
    1.5973332,
    1.6176373,
    1.599286,
    -0.3143458,
    0.17446846,
    0.24319822,
    1.0477629,
    1.2258011,
    1.362184,
    0.72082955,
    0.8353656,
    0.88150555,
    0.8901383,
    0.9415868,
    1.0723851,
    0.36758053,
    0.48239633,
    0.49854112,
    0.5758226,
    0.5928107,
    0.6420837,
    0.67831975,
    0.99770945,
    1.9606228,
    1.2180399,
    0.28024656,
    0.051383656,
    0.023585834,
    0.12440024,
    -0.57056105,
    -0.29262114,
    -0.3302291,
    -0.19528829,
    -0.5421305,
    -0.46610847,
    -0.28468513,
    -0.48777387,
    1.4224459,
    1.0963479,
    1.3554882,
    0.77129334,
    -0.07038798,
    0.40416652,
    -0.22574988,
    -0.13048096,
    -0.030161168,
    0.3232189,
    0.04954554,
    0.15955968,
    0.0054203826,
    0.09706385,
    0.0062534767,
    0.0077612293,
    -0.18049242,
    -0.15707847,
    1.0325853,
    -1.139021,
    -0.6868432,
    -0.23241615,
    0.25649944,
    0.78682137,
    -1.6941394,
    -1.1845641,
    -0.19539261,
    0.99373823,
    2.0744228,
    -0.00188174,
    0.045624822,
    0.05865494,
    0.17137231,
    0.2693768,
    0.1374934,
    0.15292902,
    -0.29544687,
    -0.079517275,
    -0.1894004,
    -0.76731634,
    -0.25005832,
    -0.11255828,
    -0.42605087,
    0.06926874,
    -0.22079101,
];

#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_5S: [f32; NUM_POLICY_FEATURES_5S] = [
    -4.755364,
    1.1311738,
    0.58556867,
    -3.4021764,
    -3.1373155,
    0.04412974,
    0.458972,
    -0.30885246,
    0.14668843,
    -0.14313453,
    0.0872659,
    -0.1082516,
    0.16851811,
    0.065404534,
    0.3631787,
    0.06401016,
    -0.104257725,
    -0.21088494,
    -0.31087786,
    -0.27435446,
    0.08521605,
    0.14504233,
    0.24366389,
    -1.4423789,
    -1.3047569,
    -0.94594556,
    0.4572711,
    1.0609539,
    2.7472804,
    0.013866621,
    -0.27202812,
    -0.0010028178,
    0.4315712,
    0.7059404,
    -0.20276293,
    -0.14279109,
    -0.023965461,
    -0.047358107,
    -0.24944656,
    0.013453558,
    0.04208832,
    0.5729235,
    1.3989291,
    -0.8662238,
    0.28540576,
    -0.17574224,
    0.02379508,
    0.37689117,
    0.40937334,
    -0.33514708,
    -0.4670903,
    -0.4427345,
    0.05848252,
    0.5580883,
    -0.5805978,
    -0.70629185,
    -0.30271703,
    0.4686727,
    2.294098,
    0.60297436,
    0.11505742,
    1.4712111,
    -0.02313266,
    -0.7007273,
    -2.5861368,
    0.36617997,
    -0.029973844,
    -0.6797968,
    0.25856647,
    0.19340551,
    0.5791781,
    1.587876,
    0.3253362,
    1.4125295,
    -0.15290302,
    -0.17460127,
    -2.217749,
    0.4083747,
    0.71800256,
    1.4114511,
    -0.22816692,
    -0.58337367,
    -0.45859587,
    2.230897,
    0.21277095,
    2.1068072,
    2.6636884,
    0.7823231,
    -4.1001625,
    -1.795641,
    0.39795363,
    1.4063269,
    0.32883975,
    0.41488165,
    0.15165795,
    0.30999684,
    0.27579188,
    0.3322012,
    -0.046668332,
    0.3830913,
    -0.015490389,
    0.39347458,
    0.2111288,
    -0.9909295,
    0.12670685,
    0.4658484,
    -0.28010443,
    -0.16104476,
    0.27260667,
    -0.04851852,
    0.004360006,
    0.009378757,
    0.33778733,
    0.092597574,
    1.1445029,
    -0.93961936,
    -0.5925652,
    -0.45683023,
    0.20834558,
    0.2884825,
    -0.9228111,
    -0.14170785,
    -1.2900378,
    -0.25622728,
    0.44415691,
    -0.028698953,
    -0.14867443,
    -0.0018671652,
    -0.004052444,
    -0.027956044,
    0.35590026,
    -1.2162863,
    -0.5925793,
    1.9376279,
    1.0999765,
    1.5152308,
    -0.39294106,
    1.1180122,
    1.2197584,
    0.5836516,
    1.1542743,
    1.2598281,
    0.25476572,
    0.25285935,
    -0.3309673,
    0.12298836,
    2.8491154,
    1.186176,
    3.5280263,
];

#[allow(clippy::unreadable_literal)]
pub const VALUE_PARAMS_6S: [f32; NUM_VALUE_FEATURES_6S] = [
    0.15032494,
    0.2004099,
    0.23468137,
    0.36364177,
    0.42615497,
    0.41651413,
    0.74279684,
    1.0102295,
    0.9851218,
    1.2786154,
    1.3374159,
    1.3933432,
    -0.38403457,
    0.034885645,
    0.118072756,
    0.7591345,
    1.0394831,
    1.250817,
    0.52444607,
    0.63054544,
    0.696335,
    0.69933826,
    0.7730416,
    0.75132674,
    0.36209446,
    0.4357826,
    0.44520032,
    0.53792,
    0.570159,
    0.5559521,
    0.609749,
    0.8553306,
    1.4764518,
    1.0722666,
    0.29201478,
    -0.08819922,
    -0.0846832,
    -0.0653986,
    -0.55806386,
    -0.19375761,
    -0.17476112,
    -0.106859066,
    -0.4016885,
    -0.47403577,
    -0.64445275,
    -0.36365113,
    1.0030613,
    0.83665866,
    1.005228,
    0.8072807,
    0.33252513,
    0.6622387,
    -0.25884378,
    -0.19568622,
    -0.012775312,
    0.25674498,
    0.088972315,
    0.15774657,
    -0.016921382,
    0.11569098,
    -0.019284582,
    0.022707444,
    -0.12779771,
    -0.12921244,
    0.85275567,
    -0.684027,
    -0.50397587,
    -0.3003094,
    -0.050307985,
    0.2258372,
    0.47624534,
    -1.1474047,
    -0.88395405,
    -0.40220466,
    0.17469327,
    0.8449353,
    1.4083447,
    -0.0026799915,
    0.016039887,
    0.034118842,
    0.033347696,
    0.14921796,
    0.055567764,
    0.18217656,
    0.22071688,
    -0.20521808,
    -0.10074911,
    -0.21885154,
    -0.4905714,
    -0.463265,
    -0.11369107,
    -0.33212864,
    0.0018254452,
    -0.20465,
];

#[allow(clippy::unreadable_literal)]
pub const POLICY_PARAMS_6S: [f32; NUM_POLICY_FEATURES_6S] = [
    -3.5961018,
    1.1715668,
    0.17949656,
    -3.1173847,
    -2.7146156,
    -0.29636982,
    0.37646836,
    -0.40992042,
    0.13564974,
    -0.080214545,
    0.073455766,
    -0.11587167,
    0.07807559,
    -0.070633404,
    0.33107528,
    0.29100162,
    0.12504591,
    -0.31672958,
    -0.28122553,
    -0.3821588,
    0.18652995,
    0.2494285,
    0.2706151,
    -0.7336709,
    -0.6134914,
    -1.3403125,
    0.15585347,
    0.7600765,
    2.3306339,
    0.0066573303,
    -0.28623724,
    -0.097407274,
    0.23667637,
    0.62867105,
    0.7386253,
    -0.21688832,
    -0.16700281,
    -0.08503936,
    -0.013302018,
    -0.056717705,
    -0.024381556,
    -0.054422356,
    -0.3119709,
    0.24450749,
    0.4980525,
    0.8796308,
    -0.052035157,
    0.29533902,
    -0.09485476,
    -0.022807823,
    0.20023687,
    0.46446759,
    0.4231539,
    -0.31820518,
    -0.37169176,
    -0.3981396,
    -0.23861855,
    0.20347169,
    0.58293164,
    -0.87294495,
    -0.961616,
    -0.7239369,
    0.2536004,
    1.0887674,
    2.4055388,
    0.6042376,
    -0.039670847,
    0.5383345,
    0.13897365,
    -0.22086447,
    -0.85672134,
    0.4506342,
    0.15414315,
    0.5155593,
    0.20372114,
    0.1959227,
    -0.5279979,
    1.7749112,
    0.5194491,
    1.7603195,
    -0.11568471,
    -0.10367626,
    -1.2129226,
    0.49326625,
    0.6533442,
    1.3472204,
    -0.08694865,
    -0.2619005,
    -0.3611921,
    2.4824772,
    -0.03424727,
    2.1948102,
    1.5011711,
    0.6755501,
    -4.0308566,
    -2.4326375,
    0.70404565,
    1.534783,
    0.4642022,
    0.6228668,
    0.1005706,
    0.398483,
    0.30044267,
    0.19416665,
    -0.030533506,
    0.3133174,
    -0.05576763,
    0.3029141,
    -0.10947571,
    -1.0040523,
    0.16132209,
    0.27788267,
    -0.4264962,
    -0.0074874256,
    0.2758454,
    -0.020382686,
    0.0054334905,
    -0.006010456,
    0.3515595,
    0.022196317,
    2.1574738,
    -0.9567373,
    -0.5230534,
    -0.6011168,
    0.51744986,
    0.1323464,
    -0.8579046,
    -0.15650782,
    -1.7326092,
    -0.9643042,
    0.4065113,
    -0.029295646,
    -0.040752478,
    -0.1207008,
    -0.043940593,
    0.015043762,
    0.10670062,
    -0.24279961,
    0.0037804677,
    -0.21292888,
    -0.1140509,
    -0.45980576,
    -0.27085638,
    2.1825454,
    1.5512332,
    1.9375703,
    0.051246107,
    1.354466,
    1.8797683,
    0.7818063,
    1.7069739,
    1.6903217,
    0.38359755,
    0.328086,
    -0.4395057,
    0.24705306,
    3.3318887,
    1.185266,
    3.5835598,
];
