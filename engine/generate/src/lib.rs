//! SYNDEOM S2 first generator family — deterministic seeded templates and grammars (ADR-0011).
//!
//! - D2: the plan and its two renderers ([`plan`]); lemma steps from `s2_template` 0.3.0 and
//!   padding steps from 0.4.0.
//! - D4: `syndeom-rng/1` ([`rng`]).
//! - D5–D7: the `s2_template` producer, its injector, its padding module (0.4.0) and ground
//!   truth ([`template`]); the structure every producer prepares ([`template::Prepared`]) and
//!   the one `assemble` that measures it ([`template::assemble`]).
//! - D6: the `s2_grammar` producer ([`grammar`]) — 0.1.0: the grammar `ring_chain` with its
//!   control and FLAW-0001.
//! - D8: the Lean harness ([`harness`]).
//! - D9, as amended by Al's ruling 5 on PR 26: the running binary's provenance ([`provenance`]).
//!
//! The remaining grammars, the remaining template sub-variants and the fidelity sweep arrive in
//! later sessions (ADR-0011 D11).

#![forbid(unsafe_code)]

pub mod grammar;
pub mod harness;
pub mod plan;
pub mod provenance;
pub mod rng;
pub mod template;

use serde_json::Value;

use template::{Prepared, Producer};

/// `generate`'s dispatch (ADR-0011 D3): a config that names a `theorem` is an `s2_template`
/// config and one that names a `grammar` is an `s2_grammar` config; each producer emits its
/// current version. A config with both keys, or neither, is refused.
pub fn prepare_any(config: &Value, seed: u64) -> Result<Prepared, String> {
    let has = |k: &str| config.get(k).is_some();
    match (has("theorem"), has("grammar")) {
        (true, false) => template::prepare(config, seed),
        (false, true) => grammar::prepare(config, seed),
        _ => Err(
            "a config names either a theorem (an s2_template config) or a grammar (an s2_grammar config)"
                .into(),
        ),
    }
}

/// `regenerate` and `verify`'s dispatch: the producer and version a manifest names.
pub fn prepare_for(producer: Producer, config: &Value, seed: u64) -> Result<Prepared, String> {
    match producer {
        Producer::Template(v) => template::prepare_with(v, config, seed),
        Producer::Grammar(v) => grammar::prepare_with(v, config, seed),
    }
}
