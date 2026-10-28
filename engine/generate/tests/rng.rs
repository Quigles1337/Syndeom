//! `syndeom-rng/1` conformance (ADR-0011 D4).
//!
//! - The D4 vectors (seeds 0, 1, 7 under the label `test`) were computed at implementation by an
//!   independent Python implementation written from D4's text (Session 40;
//!   `analysis/s2-generate-1/evidence/rng_d4.py`). They check this crate against the ADR.
//! - The attribution is verified against published reference vectors (Al's ruling 4 on PR 26):
//!   the tests of `rand_xoshiro` 0.3.0 (S-0081), which record them as produced with Vigna's
//!   reference C at xoshiro.di.unimi.it. They are `src/splitmix64.rs` (seed 1477776061723855037,
//!   50 outputs) and `src/xoshiro256starstar.rs` (state 1, 2, 3, 4, 10 outputs; that test's
//!   comment names `xoshiro128starstar.c`). The numbers below were extracted from the retrieved
//!   files by script and also match the independent Python implementation.

use syndeom_generate::rng::{Stream, label_hash, splitmix64};

/// `rand_xoshiro` 0.3.0, `src/splitmix64.rs`, test `reference` (S-0081).
const SPLITMIX64_REFERENCE: [u64; 50] = [
    1985237415132408290,
    2979275885539914483,
    13511426838097143398,
    8488337342461049707,
    15141737807933549159,
    17093170987380407015,
    16389528042912955399,
    13177319091862933652,
    10841969400225389492,
    17094824097954834098,
    3336622647361835228,
    9678412372263018368,
    11111587619974030187,
    7882215801036322410,
    5709234165213761869,
    7799681907651786826,
    4616320717312661886,
    4251077652075509767,
    7836757050122171900,
    5054003328188417616,
    12919285918354108358,
    16477564761813870717,
    5124667218451240549,
    18099554314556827626,
    7603784838804469118,
    6358551455431362471,
    3037176434532249502,
    3217550417701719149,
    9958699920490216947,
    5965803675992506258,
    12000828378049868312,
    12720568162811471118,
    245696019213873792,
    8351371993958923852,
    14378754021282935786,
    5655432093647472106,
    5508031680350692005,
    8515198786865082103,
    6287793597487164412,
    14963046237722101617,
    3630795823534910476,
    8422285279403485710,
    10554287778700714153,
    10871906555720704584,
    8659066966120258468,
    9420238805069527062,
    10338115333623340156,
    13514802760105037173,
    14635952304031724449,
    15419692541594102413,
];

/// `rand_xoshiro` 0.3.0, `src/xoshiro256starstar.rs`, test `reference` (S-0081).
const XOSHIRO256SS_REFERENCE: [u64; 10] = [
    11520,
    0,
    1509978240,
    1215971899390074240,
    1216172134540287360,
    607988272756665600,
    16172922978634559625,
    8476171486693032832,
    10595114339597558777,
    2904607092377533576,
];

#[test]
fn d4_splitmix64_matches_the_published_reference_vector() {
    // `SplitMix64::seed_from_u64` stores the seed itself as the state (little-endian round trip).
    let mut x: u64 = 1477776061723855037;
    for expected in SPLITMIX64_REFERENCE {
        assert_eq!(splitmix64(&mut x), expected);
    }
}

#[test]
fn d4_xoshiro256starstar_matches_the_published_reference_vector() {
    let mut s = Stream::from_state([1, 2, 3, 4]);
    for expected in XOSHIRO256SS_REFERENCE {
        assert_eq!(s.next_u64(), expected);
    }
}

fn first_four(seed: u64, label: &str) -> [u64; 4] {
    let mut s = Stream::new(seed, label);
    [s.next_u64(), s.next_u64(), s.next_u64(), s.next_u64()]
}

#[test]
fn d4_vectors_for_seeds_0_1_7_under_label_test() {
    assert_eq!(
        first_four(0, "test"),
        [
            0xd71e_8318_f960_66d9,
            0xea71_f31a_ce4c_6a79,
            0xab69_eb3b_3ecf_5404,
            0xa49d_4a29_6c10_7845
        ]
    );
    assert_eq!(
        first_four(1, "test"),
        [
            0x2b7e_4b9b_e99b_88ac,
            0xdeae_154d_4579_9264,
            0xba7a_1d13_6ed8_0f5e,
            0xb9a5_5191_ac98_c0de
        ]
    );
    assert_eq!(
        first_four(7, "test"),
        [
            0xb31a_f33b_becd_ab07,
            0x3910_b825_3346_a4d1,
            0xdd0d_d69e_9edc_e677,
            0x37e6_f108_ae0e_2078
        ]
    );
}

#[test]
fn d4_bounded_draw_is_the_high_word_of_the_product() {
    let mut s = Stream::new(7, "injector/FLAW-0001/1");
    assert_eq!(s.clone().next_u64(), 0x0d3c_5e98_6d53_6fce);
    assert_eq!(s.below(3), 0);
    let mut t = Stream::new(7, "injector/FLAW-0001/1");
    assert_eq!(t.below(1), 0);
}

#[test]
fn d4_labels_select_independent_streams() {
    assert_ne!(first_four(7, "test"), first_four(7, "padding"));
    assert_ne!(label_hash("test"), label_hash("padding"));
    let mut x = 0;
    assert_ne!(splitmix64(&mut x), 0);
}
