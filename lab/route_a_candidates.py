"""Route A candidate finder (ADR-0003 D1, D8) — exploratory, zero result authority (SD10).

Queries the arXiv API for math.* papers whose revision comment matches the two S-0015 stage-1
regexes and writes one TSV line per candidate, sorted by ascending arXiv identifier. It finds
candidates; it decides nothing. Admission is the executor's reading, logged in
groundtruth/HARVEST-LOG.md (ADR-0003 D2, D7).

Usage:
    python -I -X utf8 lab/route_a_candidates.py <window-start> <window-end> <out-dir>
        window dates as YYYY-MM-DD (ADR-0003 D8: 2023-01-01 2026-09-30)
        writes <out-dir>/candidates.tsv, <out-dir>/finder-state.json, <out-dir>/pages/*.xml

How it asks: the API refuses deep page offsets (HTTP 429/503 at start >= ~1500 for a large
result set — observed 2026-10-14), so the query is split into original-submission-date ranges
(yearly 1991–2019, quarterly 2020 onward) and each range is paged shallowly. Every page is cached
on disk and reused, so a re-run resumes where the last one stopped. Ranges are fetched in date
order; the output is the union of all cached ranges.

What the window means here: ADR-0003 D8 bounds the *correcting version's* submission date. The
API returns only the latest version's metadata, so this script keeps every paper whose latest
update is on or after the window start (a correcting version cannot be later than the latest
update) and marks papers whose latest update is after the window end as "latest update after
window" — the examiner reads the submission history and decides. Standard library only.
"""

import hashlib
import json
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET
from pathlib import Path

API = "https://export.arxiv.org/api/query"
NS = {"a": "http://www.w3.org/2005/Atom", "x": "http://arxiv.org/schemas/atom",
      "os": "http://a9.com/-/spec/opensearch/1.1/"}
# S-0015 Appendix D, stage 1 (case-insensitive), applied to the revision comment
RE_FIX = re.compile(r"correct|errat|error|fix|mistake|bug|flaw|wrong|revised|amendment", re.I)
RE_RESULT = re.compile(r"lemma|theorem|proposition", re.I)
# the API-side pre-filter: word forms the regexes cover, OR-ed over the comment field
FIX_WORDS = ["corrected", "correction", "corrections", "erratum", "errata", "error", "errors",
             "fix", "fixed", "fixes", "mistake", "mistakes", "bug", "flaw", "flawed", "wrong",
             "revised", "amendment"]
RESULT_WORDS = ["lemma", "theorem", "proposition"]
PAGE = 200
MAX_START = 1000           # never ask for a deeper offset than this within one range
PAUSE_SECONDS = 6.0        # arXiv asks for >= 3 s between requests; be gentler
RETRY_DELAYS = [10, 20, 40, 80, 160, 300, 300, 300]
USER_AGENT = "SYNDEOM-route-a-candidates/0.2 (lab; standard library)"


def base_query():
    fix = " OR ".join(f"co:{w}" for w in FIX_WORDS)
    res = " OR ".join(f"co:{w}" for w in RESULT_WORDS)
    return f"({fix}) AND ({res}) AND cat:math.*"


def date_ranges():
    """Original-submission-date ranges: yearly 1991-2019, quarterly 2020 until this year."""
    out = []
    for y in range(1991, 2020):
        out.append((f"{y}01010000", f"{y}12312359", f"{y}"))
    this_year = int(time.strftime("%Y", time.gmtime()))
    for y in range(2020, this_year + 1):
        for q, (m1, m2, d2) in enumerate([("01", "03", "31"), ("04", "06", "30"),
                                          ("07", "09", "30"), ("10", "12", "31")], 1):
            out.append((f"{y}{m1}010000", f"{y}{m2}{d2}2359", f"{y}Q{q}"))
    return out


def fetch(url, cache_path):
    if cache_path.exists():
        return cache_path.read_bytes(), True
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    for attempt, delay in enumerate(RETRY_DELAYS + [None]):
        try:
            with urllib.request.urlopen(req, timeout=120) as r:
                data = r.read()
            # the API sometimes answers 200 with an empty feed under load: treat as retryable
            if b"<entry>" not in data and b"totalResults>0<" not in data:
                raise urllib.error.HTTPError(url, 503, "empty feed", None, None)
            cache_path.write_bytes(data)
            return data, False
        except urllib.error.HTTPError as e:
            if e.code not in (429, 500, 502, 503, 504) or delay is None:
                raise
            print(f"HTTP {e.code}; retry {attempt + 1} in {delay}s", file=sys.stderr)
            time.sleep(delay)
    raise RuntimeError("unreachable")


def entries(xml_bytes):
    root = ET.fromstring(xml_bytes)
    total = int(root.find("os:totalResults", NS).text)
    out = []
    for e in root.findall("a:entry", NS):
        full = e.find("a:id", NS).text.strip()          # http://arxiv.org/abs/<id>v<N>
        ident = full.rsplit("/abs/", 1)[1]
        m = re.match(r"^(.*)v(\d+)$", ident)
        base, version = (m.group(1), int(m.group(2))) if m else (ident, 0)
        comment = e.find("x:comment", NS)
        comment = comment.text if comment is not None and comment.text else ""
        primary = e.find("x:primary_category", NS)
        out.append({
            "id": base, "version": version,
            "published": e.find("a:published", NS).text[:10],
            "updated": e.find("a:updated", NS).text[:10],
            "primary": primary.get("term") if primary is not None else "",
            "title": " ".join(e.find("a:title", NS).text.split()),
            "comment": " ".join(comment.split()),
        })
    return total, out


def main():
    if len(sys.argv) != 4:
        sys.exit(__doc__)
    start_day, end_day, out_dir = sys.argv[1], sys.argv[2], Path(sys.argv[3])
    pages_dir = out_dir / "pages"
    pages_dir.mkdir(parents=True, exist_ok=True)
    rows, ranges_done, requests_made, cached_hits = {}, [], 0, 0
    for lo, hi, label in date_ranges():
        q = f"{base_query()} AND submittedDate:[{lo} TO {hi}]"
        start, total = 0, None
        while True:
            params = {"search_query": q, "start": start, "max_results": PAGE,
                      "sortBy": "submittedDate", "sortOrder": "ascending"}
            url = API + "?" + urllib.parse.urlencode(params)
            cache = pages_dir / f"{label}-start{start}.xml"
            data, was_cached = fetch(url, cache)
            cached_hits += was_cached
            if not was_cached:
                requests_made += 1
                time.sleep(PAUSE_SECONDS)
            got_total, got = entries(data)
            total = got_total if total is None else total
            for r in got:
                rows[r["id"]] = r
            if len(got) < PAGE or start + PAGE >= total:
                break
            start += PAGE
            if start > MAX_START:
                print(f"range {label}: total {total} exceeds the shallow-offset limit; "
                      f"split this range further before relying on its completeness",
                      file=sys.stderr)
                break
        ranges_done.append({"range": label, "total": total})
        print(f"range {label}: total={total} rows_so_far={len(rows)}", file=sys.stderr)
    kept = []
    for r in rows.values():
        if r["version"] < 2:
            continue
        if not (RE_FIX.search(r["comment"]) and RE_RESULT.search(r["comment"])):
            continue
        if r["updated"] < start_day:
            continue
        r["window_note"] = "latest update after window" if r["updated"] > end_day else ""
        kept.append(r)
    kept.sort(key=lambda r: r["id"])
    lines = ["id\tlatest_version\tpublished\tlatest_updated\tprimary\twindow_note\ttitle\tcomment"]
    for r in kept:
        lines.append("\t".join([r["id"], f"v{r['version']}", r["published"], r["updated"],
                                r["primary"], r["window_note"], r["title"], r["comment"]]))
    data = ("\n".join(lines) + "\n").encode("utf-8")
    (out_dir / "candidates.tsv").write_bytes(data)
    state = {"base_query": base_query(), "page_size": PAGE, "ranges": ranges_done,
             "api_hits_fetched": len(rows), "candidates": len(kept),
             "requests_made_this_run": requests_made, "pages_from_cache": cached_hits,
             "window": [start_day, end_day],
             "candidates_sha256": hashlib.sha256(data).hexdigest(),
             "generated_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
    (out_dir / "finder-state.json").write_text(json.dumps(state, indent=1) + "\n", encoding="utf-8")
    print(json.dumps(state, indent=1), file=sys.stderr)


if __name__ == "__main__":
    main()
