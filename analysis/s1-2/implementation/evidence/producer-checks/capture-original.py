"""Exclusive per-command execution record; raw logs remain external."""
from datetime import datetime, timezone
from pathlib import Path
import hashlib
import json
import subprocess
import sys

work = Path(__file__).parent
root = Path(r"C:\Users\LEET\Codex\Projects\Syndeom-S1.3-Canonical")
label, relative_cwd, *command = sys.argv[1:]
log, receipt = work / f"{label}.log", work / f"{label}.json"
assert not log.exists() and not receipt.exists()
cwd = root / relative_cwd


def git(*args):
    return subprocess.check_output(["git", *args], cwd=root).decode().strip()


record = {"argv": command, "cwd": str(cwd), "started_at": datetime.now(timezone.utc).isoformat(),
          "checkout_revision": git("rev-parse", "HEAD"), "index_before": git("write-tree"),
          "status_before": git("status", "--porcelain=v1"),
          "capture_helper_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}
print(json.dumps(record), flush=True)
with log.open("xb") as f:
    result = subprocess.run(command, cwd=cwd, stdout=f, stderr=subprocess.STDOUT)
record.update({"exit_code": result.returncode, "completed_at": datetime.now(timezone.utc).isoformat(),
               "index_after": git("write-tree"), "status_after": git("status", "--porcelain=v1"),
               "raw_log_sha256": hashlib.sha256(log.read_bytes()).hexdigest()})
with receipt.open("x", encoding="utf-8", newline="\n") as f:
    json.dump(record, f, indent=2)
    f.write("\n")
print(json.dumps(record), flush=True)
print("\n".join(log.read_text(encoding="utf-8", errors="replace").splitlines()[-65:]), flush=True)
sys.exit(result.returncode)
