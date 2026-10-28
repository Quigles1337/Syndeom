// Produce the v0.1 golden cases with `syndeom-generate generate --dry-run`, one at a time.
import { mkdirSync, writeFileSync, appendFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';

const base = 'C:/Users/LEET/Codex/Temp/syndeom-generate-20261028T1706Z/goldens';
const bin = 'C:/Users/LEET/Syndeom/engine/target/debug/syndeom-generate.exe';
const commit = process.argv[2];
const seed = '7';
const spec = (cls, sub, sev, dials) => [{ class: cls, sub_variant: sub, severity: sev, site: 'auto', dials }];
const sign = { perturbation: 'sign' };
const cases = [
  ['nat_le_sq', 'control', 'two_cases', []],
  ['sq_plus_one_ge_two_x', 'control', 'expand_square', []],
  ['sq_plus_one_ge_two_x', 'flaw-0001-propagated', 'expand_square', spec('FLAW-0001', 'propagated', 'fatal', sign)],
  ['sq_plus_one_ge_two_x', 'flaw-0001-local', 'expand_square', spec('FLAW-0001', 'local', 'repairable', sign)],
  ['n_sq_mod_four', 'control', 'parity', []],
  ['n_sq_mod_four', 'flaw-0014-gap', 'parity', spec('FLAW-0014', null, 'gap', { omitted_case: 'odd', holds: true })],
];
mkdirSync(base);
for (const [theorem, name, style, flaws] of cases) {
  for (const target of ['informal', 'formal']) {
    const id = `${theorem}/${name}-${target}`;
    const dir = `${base}/${theorem}/${name}-${target}`;
    mkdirSync(dir, { recursive: true });
    const config = { theorem, style, target, flaws, padding_ratio: 0 };
    writeFileSync(`${dir}/config.json`, JSON.stringify(config) + '\n');
    const started = new Date().toISOString();
    const r = spawnSync(bin, ['generate', '--config', `${dir}/config.json`, '--seed', seed,
      '--root', 'C:/Users/LEET/Syndeom', '--engine-commit', commit, '--receipt-dir', `${dir}/receipt`, '--dry-run'],
      { encoding: 'utf8' });
    appendFileSync(`${base}/summary.txt`,
      `${id} exit=${r.status} start=${started} end=${new Date().toISOString()}\n${r.stdout}${r.stderr}\n`);
  }
}
appendFileSync(`${base}/summary.txt`, '=== GOLDENS end\n');
