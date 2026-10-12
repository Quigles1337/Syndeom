# SYNDEOM

**SYNDEOM** (συνδέω, *syndéō*, "to bind together") — calibration infrastructure for AI-generated
mathematical proofs: a generator of proof-shaped text (informal and Lean 4) with dialable, labeled
flaws and ground truth by construction; a versioned flaw taxonomy; a detector battery with a blinded
human-referee protocol; and ROC/AUC calibration per flaw class versus proof length and severity. The
deliverables are the **fooling length** at which each triage method falls to chance, and the
**cost in expert-minutes per true positive**.

SYNDEOM measures truth and triage, never understanding. It never issues verdicts on any real
contested proof; its outputs are calibrated triage scores with uncertainty. "Proven" is reserved for
sorry-free Lean proofs that have also passed statement-fidelity review.

- **Constitution (read first):** [`PROGRAM/SYNDEOM-PROGRAM.md`](PROGRAM/SYNDEOM-PROGRAM.md)
- **Current state:** [`PROGRAM/STATE.md`](PROGRAM/STATE.md)
- **Session log:** [`PROGRAM/HANDOFF.md`](PROGRAM/HANDOFF.md)
- **Flaw taxonomy:** [`PROGRAM/FLAWS.md`](PROGRAM/FLAWS.md)

Sibling programs: [KOBER](https://github.com/Quigles1337/KOBER) (Linear A falsification
infrastructure) and [FRIEDMAN](https://github.com/Quigles1337/FR13DMAN) (designedness-detection
instrument). SYNDEOM binds FRIEDMAN's generator-and-calibration discipline to KOBER's formal-methods
rigor; the structural map is [`PROGRAM/SIBLING-MAP-2026-10.md`](PROGRAM/SIBLING-MAP-2026-10.md).
