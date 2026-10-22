# Owner decisions of 2026-10-21 — orientation repair and the held harvest

**Owner:** Al (Quigles1337). **Received:** 2026-10-21 UTC, as the task text of the session
recorded in HANDOFF Session 23, after HANDOFF Session 22 and `main` `d8c0d9cbcef0af5249cbf439808b74d8828bd69d`
were on the default branch. **Transcription:** the "OWNER DECISIONS" section of that task text,
verbatim; the task's instructions are summarized in [README.md](README.md) and the decisions are
recorded in [ADR-0008](../../PROGRAM/DECISIONS/ADR-0008.md), STATE and HANDOFF. The owner asked that
these actual decisions be recorded and not be requested again.

```
OWNER DECISIONS

1. Preserve branch claude/s1-harvest-candidates-28-30 at its original
   recorded commit. Defer admission review and integration until after
   Gate S1 closes.

   This preserves the work; it does not approve the five proposed labels,
   reserve future source identifiers, advance accepted main's harvest
   position, or authorize merging/cherry-picking the stale branch.

2. I approve a narrowly scoped orientation amendment, recorded as
   ADR-0008 if that number is still available.

   The governing requirement is:
   Before reconstructing program state or executing NEXT, establish a
   successfully fetched default-branch snapshot and identify the exact
   checkout being examined.

   Implement this as an explicitly accepted addendum governing §4.1.
   Preserve the frozen PROGRAM/SYNDEOM-PROGRAM.md bytes. Put the complete
   amended procedure in the ADR and link it prominently from current
   STATE. The unchanged §6 launch prompt and SD15 requirements remain
   in force.

3. The older truth, severity and unnamed-result questions retain their
   existing dispositions. Questions introduced by the held branch remain
   pending its later admission review.

Record these actual decisions. Do not ask me to make the same choices again.
```

## What each decision binds

- **Decision 1.** Branch `claude/s1-harvest-candidates-28-30` stays at commit
  `c3a3d369689d56bab3678f519904087b313daf4e` (base `8e222a49a64273c9cefc35bca5ae15a0a05fb9ee`),
  pushed and untouched: its five manifests, their `engine_commit` and producer provenance, its
  seven source entries, records and harvest-log rows are preserved as recorded. Nothing of it is
  merged or cherry-picked; its five labels are not approved; no future source identifiers are
  reserved for it; accepted `main`'s harvest position stays at candidate 27 / `1403.4485` with
  candidate 28 / `1403.6317` saved. Admission review and integration come after Gate S1 closes.
- **Decision 2.** ADR-0008 is the accepted addendum governing §4.1; the constitution's bytes are
  unchanged; SD15, ADR-0005 and the §6 launch prompt are unchanged and in force.
- **Decision 3.** The seven truth questions, seven severity questions and the unnamed-result
  question keep the dispositions recorded in `analysis/s1-4/config/dispositions.json` and
  `analysis/s1-4/OBLIGATIONS.md`. The questions the held branch introduced (an eighth truth
  reading, two further severity readings, further unnamed results, and the "overcome in a later
  paper" wording of its item 36) are pending its later admission review and are decided nowhere
  else.
