# S1.2 producer implementation and proposed admission

Main baseline: `1a8388b03017e4044cad01ac10e7e89f8fb5e38f`, merged PR 3.
Corrected procedure/configuration commit: `d88fad2bb7b339c56bd9964167d3e0acddc8ee4e`.
The source-locator correction and its receipts are separately reviewable in
[`../correction/`](../correction/README.md). All original PR 3 evidence remains unchanged.

The implementation is [`engine/pair/`](../../../engine/pair/README.md), a narrow
archive/canonical producer under ADR-0007. It binds acceptance to the reviewed configs,
actual source lines, source bytes and templates, checks both files and the additional
parent/child invariants, preflights dry runs, and verifies its compiled input closure
against the recorded Git commit and checkout. No manifest-schema, ingest-contract,
TeX-producer or existing dependency-version change is required. Cargo.lock gains only
the local `syndeom-pair` package.

This producer is tested and gated before its first implementation commit. Real
generation occurs only after that commit exists; the production evidence record added
after generation names the exact producer SHA and actual artifact IDs. Any later
production-affecting correction requires another producer commit before regeneration.

The first test run found and fixed an actual Windows extended-drive-prefix handling
bug. The second run found a fixture that had normalized `..` before passing the path
to the checker; the fixture now preserves its literal path spelling. Their failed logs
are retained externally and will accompany the final verification record. The third
run passed all 14 new tests; workspace clippy with `-D warnings` passed.

At this implementation-commit boundary no positive pair has yet been generated or
admitted. Existing inventory remains 31 raw parents and one canonical representation.
The next step is actual-source generation from the committed producer, actual emitted-file
Lean/type/axiom checks, deterministic fresh replay and CLI refusals, followed by the final
gate and an open draft implementation/admission PR. Gate S1 and the separate first-parent
promotion hold stay open; harvest resumes candidate 28, `1403.6317`.
