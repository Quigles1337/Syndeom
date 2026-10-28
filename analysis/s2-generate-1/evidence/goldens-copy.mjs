// Copy the dry-run outputs into engine/generate/tests/goldens/ with a case.json each, and compare
// the formal outputs with the hand-authored Lean probes written before the crate existed.
import { readFileSync, writeFileSync, mkdirSync, readdirSync, existsSync } from 'node:fs';
import { createHash } from 'node:crypto';

const base = 'C:/Users/LEET/Codex/Temp/syndeom-generate-20261028T1706Z';
const dest = 'C:/Users/LEET/Syndeom/engine/generate/tests/goldens';
const sha = (b) => createHash('sha256').update(b).digest('hex');
const probes = {
  'sq_plus_one_ge_two_x/control-formal': 'sq_faithful',
  'sq_plus_one_ge_two_x/flaw-0001-propagated-formal': 'sq_propagated',
  'sq_plus_one_ge_two_x/flaw-0001-local-formal': 'sq_local',
  'n_sq_mod_four/control-formal': 'nsq_faithful',
  'n_sq_mod_four/flaw-0014-gap-formal': 'nsq_gap',
};
const report = [];
for (const theorem of readdirSync(`${base}/goldens`).filter((d) => !d.includes('.')).sort()) {
  for (const name of readdirSync(`${base}/goldens/${theorem}`).sort()) {
    const src = `${base}/goldens/${theorem}/${name}`;
    const receipt = JSON.parse(readFileSync(`${src}/receipt/receipt.json`, 'utf8'));
    const out = `${dest}/${theorem}/${name}`;
    mkdirSync(out, { recursive: true });
    const files = readdirSync(`${src}/receipt/outputs`).sort();
    for (const f of files) writeFileSync(`${out}/${f}`, readFileSync(`${src}/receipt/outputs/${f}`));
    const config = JSON.parse(readFileSync(`${src}/config.json`, 'utf8'));
    const caseJson = {
      config,
      seed: 7,
      engine_commit: receipt.engine_commit,
      toolchain: receipt.toolchain,
      observation: receipt.observation,
    };
    writeFileSync(`${out}/case.json`, JSON.stringify(caseJson, null, 2) + '\n');
    const row = { case: `${theorem}/${name}`, files, verdict: receipt.verdict, observation: receipt.observation };
    const probe = probes[`${theorem}/${name}`];
    if (probe) {
      // the probes of the controls end with "\n#print axioms <main>\n"; strip that tail
      let p = readFileSync(`${base}/lean-probe/${probe}.lean`, 'utf8');
      p = p.replace(/\n#print axioms [^\n]+\n$/, '');
      const g = readFileSync(`${out}/Proof.lean`, 'utf8');
      row.hand_authored_probe = probe;
      row.equals_probe = p === g;
      row.proof_sha256 = sha(Buffer.from(g, 'utf8'));
    }
    report.push(row);
  }
}
writeFileSync(`${base}/goldens-copy-report.json`, JSON.stringify(report, null, 1) + '\n');
console.log(JSON.stringify(report, null, 1));
