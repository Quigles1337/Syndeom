//! Development-only rehearsal of existing pure APIs; creates no Manifest or artifact.
use std::{env, fs};
use syndeom_manifest::{artifact_id, config_hash, content_hash, parse_informal_md, parse_lean};

fn main() {
    let a: Vec<_> = env::args().collect();
    assert_eq!(
        a.len(),
        5,
        "informal formal archive-config canonical-config"
    );
    let informal = fs::read(&a[1]).unwrap();
    let formal = fs::read(&a[2]).unwrap();
    let i = parse_informal_md(&informal).expect("canonical informal bytes");
    let f = parse_lean(&formal).expect("canonical formal bytes");
    assert_eq!(i.n_steps, 0);
    assert_eq!(i.n_lemmas, 0);
    assert_eq!(i.steps, ["There exist an infinite number of primes."]);
    assert_eq!(f.imports, ["Mathlib.Data.Nat.PrimeFin"]);
    assert_eq!(f.declarations, [(3, "syndeom_s1_2_judson_primes".into())]);
    println!("DEVELOPMENT REHEARSAL ONLY: no manifest, producer record, or corpus admission");
    println!("informal: {i:?}");
    println!("formal: {f:?}");
    println!("informal_sha256: {}", content_hash(&informal));
    println!("formal_sha256: {}", content_hash(&formal));
    let mut archive_hash = String::new();
    for (path, generator) in [(&a[3], "s1_pair_archive"), (&a[4], "s1_pair")] {
        let config = fs::read_to_string(path).unwrap().parse().unwrap();
        let hash = config_hash(&config).unwrap();
        if generator == "s1_pair_archive" {
            archive_hash.clone_from(&hash);
        } else {
            assert_eq!(
                config["parent_config_sha256"].as_str(),
                Some(archive_hash.as_str())
            );
            assert_eq!(
                config["derived_from"].as_str().unwrap(),
                artifact_id("s1_pair_archive", "0.1.0", &archive_hash, None)
            );
        }
        println!("{generator} proposed_config_sha256: {hash}");
        println!(
            "{generator} prospective_id_not_admitted: {}",
            artifact_id(generator, "0.1.0", &hash, None)
        );
    }
    let mut bad_informal = informal.clone();
    bad_informal.insert(0, b'\r');
    assert!(parse_informal_md(&bad_informal).is_err());
    assert!(parse_lean(b"import Lean\n").is_err());
    println!("negative format controls: CR informal and non-Mathlib import rejected");
}
