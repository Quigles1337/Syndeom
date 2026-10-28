//! The first later prefix's inventory and coverage (ADR-0013); no admission or derivation producer.
#[allow(dead_code)]
#[path = "support/later_prefix.rs"]
mod later_prefix;

fn main() {
    if let Err(error) = later_prefix::cli() {
        eprintln!("FAIL: {error}");
        std::process::exit(1);
    }
}
