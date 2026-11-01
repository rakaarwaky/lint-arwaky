#!/usr/bin/env python3
"""AES501-AES506 module-reachability audit (orphan family).

The static auditor in `audit_architecture_compliance.py` cannot answer the
orphan rules: they need the *wiring* graph, not the naming/import-surface
tables.  This script builds that graph and reports files nothing live consumes.

Model
-----
* nodes   : every production `.rs` file under `crates/` (tests/benches excluded,
            because `DEFAULT_IGNORED_PATHS` puts them outside the scan)
* edges   : consumer -> producer.  A file "uses" another file when it mentions
            the producer's module stem or one of the producer's top-level
            `pub` item names.
* liveness: BFS *down* from the entry layers.  A file is live when some
            entry-layer file reaches it through the edge direction above.
* barrels : `lib.rs` / `mod.rs` are path routers.  `pub mod x;` is a
            declaration, not a use, and a `pub use` re-export is not wiring
            (AES502 says so explicitly), so barrels are neither nodes nor
            consumers here.  This is the single most important calibration:
            counting barrel lines as consumers hides every dead module.

Rule mapping
------------
AES501 taxonomy   no live consumer at all (the doc's "no inbound contract
                  import" case is the narrow version of this)
AES502 contract   no live implementor / consumer
AES503 capability not reachable from a root container
AES504 utility    not consumed by any capability/agent/surface/root/contract
AES505 agent      orchestrator not reachable from a root container
AES506 surface    not reachable from a root container

Known blind spots (reported separately, not by this script)
-----------------------------------------------------------
* root containers are *roots*, so a container nobody constructs can never be
  flagged here -- see finding A9 in ARCHITECTURE_AUDIT.md.
* symbols produced by `include!(concat!(env!("OUT_DIR"), ...))` have no
  definition to find (e.g. `utility_release_version.rs` -> `RELEASE_VERSION`);
  such files must be verified by hand.
* a file whose only job is `impl Trait for LiveType` looks live; that is
  correct for the orphan rules but it is what AES404 is for.

Usage: python3 scripts/audit_module_reachability.py
"""
import importlib.util
import os
import re
from collections import defaultdict, deque

_HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location(
    "aud", os.path.join(_HERE, "audit_architecture_compliance.py")
)
m = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(m)

BARRELS = {"lib.rs", "mod.rs"}
# Method names that say nothing about ownership; excluding them prevents one
# `pub fn new` from marking every file in the workspace as a consumer.
GENERIC = {
    "new", "default", "fmt", "from", "into", "clone", "drop", "main", "error",
    "deref", "display", "eq", "hash", "next", "run", "build",
}

files = {}
for p in m.rust_sources():
    if m.is_test_or_bench(p):
        continue
    rel = m.rel(p).split("/")
    crate = "shared/" + rel[2] if rel[1] == "shared" else rel[1]
    files[p] = (crate, os.path.basename(p)[:-3])

code, rawcode, tokens, exports = {}, {}, {}, {}
for p in files:
    rawcode[p] = m.strip_cfg_test(m.no_strings(m.read(p)))
    code[p] = m.code_only(m.strip_macros(rawcode[p]))
    if os.path.basename(p) in BARRELS:
        reexport = set()
        for line in code[p].splitlines():
            if re.match(r"\s*pub use\s", line):
                reexport |= set(re.findall(r"[A-Za-z_]\w*", line))
        tokens[p] = reexport
    else:
        tokens[p] = set(re.findall(r"[A-Za-z_]\w*", code[p]))
    names = set(
        re.findall(
            r"^pub (?:struct|enum|trait|const|static|type|fn|async fn)\s+(\w+)",
            code[p],
            re.M,
        )
    )
    names |= set(re.findall(r"macro_rules!\s*(\w+)", rawcode[p]))
    exports[p] = names - GENERIC

index = defaultdict(set)
for p, toks in tokens.items():
    for tk in toks:
        index[tk].add(p)


def referrers(target):
    """Files that mention the target's stem or one of its exported names."""
    _crate, stem = files[target]
    out = set()
    for n in {stem} | exports[target]:
        out |= index.get(n, set())
    out.discard(target)
    return {p for p in out if os.path.basename(p) not in BARRELS}


edges = defaultdict(set)  # consumer -> producers it uses
for p in files:
    if os.path.basename(p) in BARRELS:
        continue
    for consumer in referrers(p):
        edges[consumer].add(p)


def layer(p):
    return m.layer_of(files[p][1])


def live_from(seed_layers):
    src = [p for p in files if layer(p) in seed_layers]
    seen, q = set(src), deque(src)
    while q:
        n = q.popleft()
        for t in edges[n]:
            if os.path.basename(t) in BARRELS or t in seen:
                continue
            seen.add(t)
            q.append(t)
    return seen


root_live = live_from({"root"})
high_live = live_from({"contract", "capabilities", "agent", "surface", "root"})
util_live = live_from({"capabilities", "agent", "surface", "root", "contract"})

out = defaultdict(list)
for p in sorted(files):
    if os.path.basename(p) in BARRELS:
        continue
    L = layer(p)
    if L is None:
        continue
    if L in {"capabilities", "agent", "surface"} and p not in root_live:
        out[{"capabilities": "AES503", "agent": "AES505", "surface": "AES506"}[L]].append(m.rel(p))
    elif L in {"taxonomy", "contract"} and p not in high_live:
        out["AES501" if L == "taxonomy" else "AES502"].append(m.rel(p))
    elif L == "utility" and p not in util_live:
        out["AES504"].append(m.rel(p))

for k in sorted(out):
    print(f"### {k}: {len(out[k])}")
    for r in sorted(out[k]):
        print("   ", r)
print(f"\nfiles: {len(files)}")
