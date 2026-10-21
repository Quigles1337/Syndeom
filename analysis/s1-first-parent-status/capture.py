import datetime, hashlib, json, pathlib, subprocess, sys
folder, name, cwd, *cmd = sys.argv[1:]
folder = pathlib.Path(folder)
log, receipt = folder/(name+'.log'), folder/(name+'.json')
assert not log.exists() and not receipt.exists()
def git(*args):
    return subprocess.check_output(['git',*args],cwd=cwd).decode().strip()
r = dict(command=cmd,cwd=cwd,head=git('rev-parse','HEAD'),tree=git('rev-parse','HEAD^{tree}'),status_before=git('status','--porcelain=v1','--untracked-files=all'),started=datetime.datetime.now(datetime.timezone.utc).isoformat())
with log.open('xb') as f:
    p = subprocess.run(cmd,cwd=cwd,stdout=f,stderr=subprocess.STDOUT)
r.update(exit_code=p.returncode,finished=datetime.datetime.now(datetime.timezone.utc).isoformat(),log_sha256=hashlib.sha256(log.read_bytes()).hexdigest(),head_after=git('rev-parse','HEAD'),tree_after=git('rev-parse','HEAD^{tree}'),status_after=git('status','--porcelain=v1','--untracked-files=all'))
with receipt.open('x',encoding='utf-8',newline='\n') as f:
    f.write(json.dumps(r,indent=2)+'\n')
sys.exit(p.returncode)
