import re
import subprocess
import sys
from pathlib import Path

root = Path(sys.argv[1])
files = subprocess.check_output(["git", "-C", str(root), "ls-files"], text=True).split()
problems = []

print("== bytes ==")
BINARY = (".pdf", ".png", ".jpg", ".zip", ".gz", ".tgz")  # marked binary in .gitattributes; not line-ending checked
for f in files:
    if f.lower().endswith(BINARY):
        print(f"binary (skipped): {f}")
        continue
    b = (root / f).read_bytes()
    cr = b.count(b"\r")
    bom = b.startswith(b"\xef\xbb\xbf")
    nl = b.endswith(b"\n")
    if cr or bom or not nl:
        problems.append(f"bytes: {f}")
        print(f"PROBLEM {f}: {len(b)} bytes, CR={cr}, BOM={bom}, trailing_LF={nl}")
print(f"{len(files)} tracked files checked for CR/BOM/trailing LF")

prog = {f: (root / f).read_text(encoding="utf-8") for f in files if f.endswith(".md")}
const = prog["PROGRAM/SYNDEOM-PROGRAM.md"]
state = prog["PROGRAM/STATE.md"]
flaws = prog["PROGRAM/FLAWS.md"]
claims = prog["PROGRAM/CLAIMS.md"]
sources = prog["PROGRAM/SOURCES.md"]
adr0 = prog["PROGRAM/DECISIONS/ADR-0000.md"]
alltext = "\n".join(prog.values())

def xref(name, defined, referenced):
    missing = sorted(referenced - defined, key=lambda s: (len(s), s))
    print(f"== {name}: defined {len(defined)}, referenced {len(referenced)}, undefined {missing}")
    if missing:
        problems.append(f"{name}: undefined {missing}")

# Q definitions: the Session-1 STATE list (**Qn —) or the ADR-0000 addendum table (| Qn |)
q_defined = set(re.findall(r"\*\*Q(\d+) —", state)) | set(re.findall(r"^\| Q(\d+) \|", adr0, re.M))
xref("Q", q_defined, set(re.findall(r"\bQ(\d+)\b", alltext)))
xref("SD", set(re.findall(r"\*\*SD(\d+) —", const)), set(re.findall(r"\bSD(\d+)\b", alltext)))
xref("FLAW", set(re.findall(r"^## FLAW-(\d{4})", flaws, re.M)), set(re.findall(r"FLAW-(\d{4})", alltext)))
xref("CLAIM", set(re.findall(r"^## CLAIM-(\d{4})", claims, re.M)), set(re.findall(r"CLAIM-(\d{4})\b", alltext)))
xref("S-", set(re.findall(r"^## S-(\d{4})", sources, re.M)), set(re.findall(r"\bS-(\d{4})\b", alltext)))

print("== phase items ==", re.findall(r"^- (S\d\.\d) ", const, re.M))
print("== gates ==", re.findall(r"\*\*Gate (S\d):\*\*", const))
print("== CLAIM statuses ==", re.findall(r"^- \*\*Status:\*\* (\S+)", claims, re.M))
print("== SOURCE states ==", re.findall(r"^- \*\*State:\*\* (\S+)", sources, re.M))
print("== DRAFT word count by file ==", {f: t.count("DRAFT") for f, t in prog.items() if "DRAFT" in t})

print()
print("RESULT:", "ALL PASS" if not problems else f"{len(problems)} PROBLEM(S): {problems}")
sys.exit(1 if problems else 0)
