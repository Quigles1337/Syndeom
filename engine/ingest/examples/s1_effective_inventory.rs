//! One accepted raw metadata amendment; no admission or derivation producer.
#[path = "support/effective.rs"]
mod effective;

fn main() {
    if let Err(error) = effective::cli() {
        eprintln!("FAIL: {error}");
        std::process::exit(1);
    }
}
