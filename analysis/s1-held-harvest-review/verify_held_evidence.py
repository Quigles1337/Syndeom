"""Re-verify Session 22's preserved held evidence against held-evidence-manifest.json.

Usage: python -I -X utf8 verify_held_evidence.py <held-evidence-dir> <committed-manifest> <out.json>
Reads only; writes one JSON result (refuses to overwrite).
"""
import hashlib
import json
import os
import sys
from datetime import datetime, timezone


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def main():
    root, committed, out = sys.argv[1], sys.argv[2], sys.argv[3]
    started = datetime.now(timezone.utc).isoformat()
    local_manifest = os.path.join(root, "manifest.json")
    m_local_sha = sha256(local_manifest)
    m_committed_sha = sha256(committed)
    with open(committed, encoding="utf-8") as f:
        m = json.load(f)
    dest = m["destination"]
    base = os.path.join(root, os.path.basename(dest.rstrip("\\/")))
    results, bad = {}, []
    total = 0
    for rel, rec in sorted(m["files"].items()):
        p = os.path.join(base, *rel.replace("\\", "/").split("/"))
        if not os.path.isfile(p):
            bad.append({"file": rel, "problem": "missing"})
            continue
        got = sha256(p)
        n = os.path.getsize(p)
        total += n
        want = rec.get("sha256")
        want_n = rec.get("bytes")
        ok = got == want and (want_n is None or n == want_n)
        results[rel] = {"sha256": got, "bytes": n, "ok": ok}
        if not ok:
            bad.append({"file": rel, "want": want, "got": got, "want_bytes": want_n, "got_bytes": n})
    on_disk = []
    for dp, _, fns in os.walk(base):
        for fn in fns:
            on_disk.append(os.path.relpath(os.path.join(dp, fn), base).replace("\\", "/"))
    listed = {k.replace("\\", "/") for k in m["files"]}
    extra = sorted(set(on_disk) - listed)
    res = {
        "schema": "syndeom-held-evidence-reverify/1",
        "started_utc": started,
        "finished_utc": datetime.now(timezone.utc).isoformat(),
        "evidence_root": root,
        "manifest_in_root_sha256": m_local_sha,
        "manifest_committed_sha256": m_committed_sha,
        "manifest_copies_identical": m_local_sha == m_committed_sha,
        "manifest_file_count": m["file_count"],
        "manifest_total_bytes": m["total_bytes"],
        "checked": len(results),
        "ok": sum(1 for r in results.values() if r["ok"]),
        "total_bytes_on_disk": total,
        "mismatches_or_missing": bad,
        "files_on_disk_not_in_manifest": extra,
        "unavailable_bytes_entries": len(m.get("unavailable_bytes", [])),
        "result": "ALL MATCH" if not bad and not extra and len(results) == m["file_count"] else "PROBLEM",
    }
    with open(out, "x", encoding="utf-8") as f:
        json.dump(res, f, indent=2, ensure_ascii=False)
        f.write("\n")
    print(json.dumps({k: res[k] for k in res if k not in ("mismatches_or_missing",)}, indent=1)[:2500])
    print("mismatches:", len(bad))
    return 0 if res["result"] == "ALL MATCH" else 1


if __name__ == "__main__":
    sys.exit(main())
