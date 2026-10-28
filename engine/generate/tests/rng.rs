//! `syndeom-rng/1` conformance (ADR-0011 D4). The vectors were computed at implementation by an
//! independent Python implementation written from D4's text (Session 40; script and output under
//! the session's external evidence root), so they check this crate against the ADR's definition.
//! Agreement with the published SplitMix64 and xoshiro256** reference code is not checked here;
//! the attribution is unverified (D4).

use syndeom_generate::rng::{Stream, label_hash, splitmix64};

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
