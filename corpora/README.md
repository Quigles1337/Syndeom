# corpora/ — manifested synthetic artifacts (S2)

Layout per ADR-0001 D2: `corpora/<artifact_id>/manifest.json` plus `proof.md` and/or `Proof.lean`.
Every artifact here has `origin: "synthetic"` and `ground_truth.derivation: "by-construction"`
(SD1, SD12); an artifact without a manifest does not exist to this program.

Storage default for the proof files is decided in ADR-0002 (`regenerable`: manifests are
committed, proof files are regenerated from the manifest and verified by `content_hash`).
Golden fixtures live in `engine/manifest/tests/fixtures/`, not here. Empty until Phase S2.
