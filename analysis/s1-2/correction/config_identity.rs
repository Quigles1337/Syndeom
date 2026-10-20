//! Phase A identity calculation using the unchanged canonical Rust authority.
use std::{env, fs};
use syndeom_manifest::{artifact_id, config_hash};

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(args.len(), 3, "config.json generator");
    let value = fs::read_to_string(&args[1]).unwrap().parse().unwrap();
    let hash = config_hash(&value).unwrap();
    println!("{hash}");
    println!("{}", artifact_id(&args[2], "0.1.0", &hash, None));
}
