use crate::position::num_square_symmetries;

pub const NUM_VALUE_FEATURES_4S: usize = 73;
pub const NUM_POLICY_FEATURES_4S: usize = 138;

pub const NUM_VALUE_FEATURES_5S: usize = 91;
pub const NUM_POLICY_FEATURES_5S: usize = 157;

pub const NUM_VALUE_FEATURES_6S: usize = 94;
pub const NUM_POLICY_FEATURES_6S: usize = 167;

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
    pub fcd: &'a mut [f32],
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
        let (fcd, coefficients) = coefficients.split_at_mut(8);
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
            fcd,
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
    -3.517305,
    1.4156812,
    1.1054099,
    -2.459851,
    -2.7257361,
    0.18018618,
    0.45840785,
    -0.23085578,
    0.19408847,
    -0.15243755,
    0.15461679,
    0.06578773,
    0.16587007,
    0.4307622,
    -0.27006903,
    -0.38956186,
    0.32724562,
    -0.0011021066,
    -0.0057374667,
    0.00015021767,
    0.00848979,
    -0.28406668,
    0.45781937,
    1.1286839,
    -0.20303512,
    -0.087381475,
    -0.18955293,
    -0.2263027,
    0.00022801384,
    -0.007706399,
    -0.008808966,
    -0.0034274147,
    0.1449845,
    -0.38534492,
    0.31906918,
    1.2246182,
    -0.5652572,
    -0.4637201,
    -0.42273992,
    0.76269364,
    0.008160291,
    -0.0016911505,
    0.008150065,
    0.0016175173,
    0.56777817,
    -0.18274973,
    0.0010270234,
    -1.3510655,
    -0.103079855,
    0.008161059,
    0.5034794,
    0.4153583,
    0.0021942016,
    0.066186346,
    -0.2966135,
    0.0051122922,
    0.8450302,
    1.3199521,
    0.0074368455,
    0.16734365,
    -0.9220521,
    -0.0077824043,
    0.21182755,
    0.4174671,
    0.0043121455,
    -0.9328329,
    -0.35034212,
    -0.006815267,
    2.4376605,
    0.22532937,
    0.9738848,
    0.008927224,
    0.5812266,
    -5.124265,
    -1.197279,
    0.24319229,
    1.4058412,
    0.42590767,
    0.28459212,
    0.28617054,
    0.58211595,
    0.18159384,
    0.34988704,
    -0.34513056,
    0.53766346,
    0.0060925093,
    0.0012618108,
    -0.0039176345,
    0.0039970186,
    -0.005868571,
    -0.0075792954,
    0.00952702,
    0.0032783058,
    0.35625905,
    0.06020934,
    -0.0077192923,
    -0.009022991,
    0.066513464,
    -0.20717226,
    -0.006355319,
    -0.96708935,
    -0.92719966,
    0.0012055086,
    -0.29887822,
    -2.18496,
    -1.8565207,
    -0.20602232,
    0.57668835,
    0.9695834,
    0.88601005,
    0.24935538,
    0.8295803,
    0.29545456,
    -2.5044622,
    -2.9680982,
    -0.22084734,
    -0.2375742,
    0.41423905,
    -0.15235187,
    -0.005897157,
    0.0069459323,
    -0.0069796084,
    1.3515189,
    0.81014615,
    0.0042860033,
    -0.45004252,
    -0.3100908,
    0.0054509398,
    0.31383207,
    1.0227821,
    0.005903894,
    0.09770534,
    -0.16001864,
    0.007400766,
    0.6717732,
    1.9348854,
    0.47109804,
    2.84209,
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
    -3.2426937,
    1.1334487,
    0.58837485,
    -2.5397959,
    -2.4528909,
    0.04625216,
    0.46276712,
    -0.3057371,
    0.1460385,
    -0.14103916,
    0.08693042,
    -0.11259244,
    0.16425481,
    0.061159525,
    0.35898963,
    0.05984606,
    -0.10804091,
    -0.20158556,
    -0.30121508,
    -0.26566204,
    0.091090985,
    0.14794934,
    0.24316224,
    -1.3963078,
    -1.293727,
    -0.92418027,
    0.4726246,
    1.0687442,
    2.7428446,
    0.016712716,
    -0.26934072,
    0.0015435147,
    0.43398425,
    0.73539674,
    -0.21506691,
    -0.1598312,
    -0.046794016,
    -0.077762656,
    -0.06978081,
    -0.020790154,
    0.0025045224,
    0.5060926,
    1.2638628,
    -0.45235795,
    0.28478718,
    -0.17642508,
    0.022916885,
    0.37561288,
    0.4070511,
    -0.32209694,
    -0.4552536,
    -0.43210647,
    0.067981824,
    0.56294334,
    -0.5426094,
    -0.66863406,
    -0.27507365,
    0.49595112,
    2.291272,
    0.60231113,
    -0.044524603,
    0.55989534,
    -0.023457732,
    -0.4281683,
    -1.0040728,
    0.3670006,
    0.10536757,
    0.1595078,
    0.25907844,
    0.048182953,
    -0.5209433,
    1.5879875,
    0.35067734,
    0.87858313,
    -0.15285206,
    -0.19722758,
    -0.3583819,
    0.4088183,
    0.7273416,
    1.4065808,
    -0.22887233,
    -0.5916815,
    -0.45701656,
    2.213128,
    0.21403779,
    2.1060362,
    2.644996,
    0.5916947,
    -3.7746875,
    -0.90954715,
    0.39803496,
    1.4063982,
    0.3277085,
    0.4149052,
    0.1516824,
    0.31029,
    0.27217838,
    0.33564222,
    -0.04965345,
    0.38840216,
    -0.018611567,
    0.39763406,
    -0.21554156,
    -0.31147504,
    0.031966582,
    0.15731743,
    -0.32171065,
    -0.08077651,
    0.27664593,
    -0.054279074,
    -0.0066909315,
    0.0056339707,
    0.33773515,
    0.09082926,
    1.1466126,
    -0.23553102,
    0.060630873,
    0.19176123,
    -1.2429785,
    -1.5893734,
    -1.2305157,
    0.11752341,
    0.81815904,
    1.2538441,
    0.8908952,
    1.0074295,
    0.2986588,
    -0.1800095,
    0.11518992,
    -0.95492566,
    -0.39097744,
    -0.780994,
    0.36050105,
    -0.01312785,
    -0.11744087,
    0.039280325,
    0.13988578,
    0.03194437,
    0.34802625,
    -0.5129511,
    -0.14653763,
    1.8025805,
    1.0939313,
    1.4113535,
    -0.0641356,
    1.2460495,
    1.7527406,
    0.4919175,
    1.1079466,
    1.1366708,
    0.25681293,
    0.17321478,
    -0.37564853,
    0.052003432,
    2.5768254,
    0.9543182,
    3.2341588,
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
    -2.576094,
    1.0901988,
    0.40482187,
    -2.249616,
    -2.0377758,
    -0.1199488,
    0.38244337,
    -0.36328197,
    0.0986858,
    -0.06176899,
    0.07917169,
    -0.15301108,
    -0.060707442,
    -0.1431243,
    0.3931565,
    0.4705812,
    0.21784095,
    -0.3033915,
    -0.39093173,
    -0.32393137,
    0.17145872,
    0.31151432,
    0.37210828,
    -0.6952023,
    -0.99417466,
    -1.0170014,
    -0.038113795,
    0.82976955,
    2.4679122,
    0.012820447,
    -0.24606143,
    -0.037175406,
    0.31386897,
    0.7081139,
    0.7276141,
    -0.19149128,
    -0.15203637,
    -0.024720285,
    0.016591007,
    0.031419255,
    -0.031693824,
    0.062400267,
    -0.23391746,
    0.2549008,
    0.50426745,
    0.5235303,
    -0.04994831,
    0.28797227,
    -0.061300483,
    0.03369779,
    0.26938114,
    0.52004564,
    0.42637384,
    -0.37453955,
    -0.41510874,
    -0.3919666,
    -0.15971617,
    0.3210285,
    0.6871645,
    -1.0579641,
    -0.8955643,
    -0.50316817,
    0.40125892,
    1.2803806,
    1.8441615,
    0.5497976,
    -0.053382955,
    0.29024997,
    0.280401,
    -0.21167523,
    0.046422895,
    0.39269835,
    0.08420523,
    0.22663872,
    0.1890304,
    0.22255647,
    -0.15183347,
    1.6415182,
    0.4047405,
    0.9214274,
    -0.106253356,
    -0.06718043,
    -0.024545034,
    0.48902562,
    0.74258286,
    1.2981079,
    -0.11066783,
    -0.23972672,
    -0.3460837,
    2.459993,
    -0.105026476,
    2.1209347,
    1.8717123,
    0.5385578,
    -3.4842384,
    -0.75706834,
    0.72509277,
    1.3911781,
    0.4566518,
    0.55191964,
    0.08974422,
    0.3234107,
    0.29166165,
    0.15095913,
    -0.014927925,
    0.32524824,
    -0.023047494,
    0.32823482,
    -0.36550298,
    -0.55943626,
    0.27659503,
    0.22075957,
    -0.20004377,
    -0.036281757,
    0.21599342,
    0.031126957,
    0.0030306866,
    -0.00916178,
    0.25094518,
    0.07858347,
    1.8231647,
    -0.241417,
    0.21002829,
    0.23274316,
    -0.786103,
    -2.0032475,
    -1.453027,
    0.023712017,
    0.83053404,
    1.3654993,
    1.324841,
    0.90062535,
    0.7251709,
    -0.41524222,
    0.099661775,
    -0.989749,
    -0.6250051,
    -0.92347234,
    0.3124071,
    -0.016236674,
    -0.029199615,
    -0.110223524,
    0.0013813265,
    0.09938834,
    0.24322031,
    -0.21268374,
    0.17007695,
    -0.2935862,
    -0.14467445,
    -0.14977032,
    -0.0036724228,
    1.964354,
    1.5061558,
    1.7615906,
    -0.029581644,
    0.51637024,
    1.6985507,
    0.65151435,
    1.5513816,
    1.1090717,
    0.45983377,
    0.26956674,
    -0.39630514,
    0.23652564,
    3.017353,
    0.9725984,
    3.157878,
];
