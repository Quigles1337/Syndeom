//! S1.4 reporting tool; analysis metadata, never a corpus producer (SD10).
#[path = "support/coverage.rs"]
mod coverage;

fn main() {
    if let Err(error) = coverage::cli(std::env::args().skip(1).collect()) {
        eprintln!("S1.4 refused: {error}");
        std::process::exit(2);
    }
}
