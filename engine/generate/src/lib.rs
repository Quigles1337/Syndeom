//! SYNDEOM S2 first generator family — deterministic seeded templates and grammars (ADR-0011).
//!
//! - D2: the plan and its two renderers ([`plan`]).
//! - D4: `syndeom-rng/1` ([`rng`]).
//! - D5–D7: the `s2_template` producer, its injector and ground truth ([`template`]).
//! - D8: the Lean harness ([`harness`]).
//! - D9, as amended by Al's ruling 5 on PR 26: the running binary's provenance ([`provenance`]).
//!
//! `s2_grammar`, padding (FLAW-0013), lemma steps and the fidelity sweep arrive in later sessions
//! (ADR-0011 D11).

#![forbid(unsafe_code)]

pub mod harness;
pub mod plan;
pub mod provenance;
pub mod rng;
pub mod template;
