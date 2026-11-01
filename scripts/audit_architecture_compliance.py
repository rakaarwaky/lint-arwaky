#!/usr/bin/env python3
"""Static audit of `crates/` against ARCHITECTURE.md (AES rule set).

This is an *external* checker: it re-derives the AES101-AES706 invariants from
ARCHITECTURE.md instead of calling the linter binary, so it can be run in
environments without a Rust toolchain. It is a review aid, not a CI gate.

Scope mirrors `DEFAULT_IGNORED_PATHS`: `tests/` and `benches/` are excluded from
every rule except AES103 (test-file naming) and AES704 (test-suite coverage).

Usage: python3 scripts/audit_architecture_compliance.py [--json]
"""
from __future__ import annotations

import json
import os
import re
import sys
from collections import defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRATES = os.path.join(ROOT, "crates")

BARREL = {"lib.rs", "mod.rs", "build.rs", "__init__.py", "index.js", "index.ts", "index.tsx"}
ENTRY = BARREL | {"py.typed", "main.rs", "main.py"}
DEFAULT_IGNORED = {"tests", "benches"}

LAYER_PREFIXES = [
    ("taxonomy_", "taxonomy"),
    ("contract_", "contract"),
    ("capabilities_", "capabilities"),
    ("utility_", "utility"),
    ("agent_", "agent"),
    ("surface_", "surface"),
    ("root_", "root"),
]

# AES102 Layer Suffix Policy (ARCHITECTURE.md §AES102)
SUFFIX_POLICY = {
    "root": ("strict", ["entry", "container"], []),
    "taxonomy": ("strict", ["vo", "entity", "error", "event", "constant", "request", "response"], []),
    "contract": ("strict", ["protocol", "aggregate"], []),
    "utility": (
        "flexible",
        [],
        ["vo", "entity", "error", "event", "constant", "protocol", "aggregate", "request", "response"],
    ),
    "capabilities": (
        "flexible",
        [],
        ["vo", "entity", "error", "event", "constant", "constants", "protocol", "aggregate", "utility",
         "request", "response"],
    ),
    "agent": ("strict", ["orchestrator"], []),
    "surface": (
        "strict",
        ["command", "controller", "page", "view", "component", "router", "layout", "hook", "store",
         "action", "screen"],
        [],
    ),
}

# AES201 Import Matrix (ARCHITECTURE.md §AES201)
IMPORT_MATRIX = {
    "taxonomy(vo)": ({"taxonomy"}, set(),
                     {"agent", "surface", "contract", "utility", "capabilities", "root"}),
    "taxonomy(request,response)": ({"taxonomy"}, set(),
                                   {"agent", "surface", "contract", "utility", "capabilities", "root"}),
    "taxonomy(entity,error,event)": ({"taxonomy"}, {"taxonomy"},
                                     {"agent", "surface", "contract", "utility", "capabilities", "root"}),
    "taxonomy(constant)": ({"taxonomy"}, set(),
                           {"agent", "surface", "contract", "utility", "capabilities", "root"}),
    "utility": ({"taxonomy"}, set(),
                {"agent", "surface", "contract", "capabilities", "root", "utility"}),
    "contract(protocol)": ({"taxonomy", "contract"}, {"taxonomy"},
                           {"agent", "surface", "capabilities", "contract(aggregate)", "root"}),
    "contract(aggregate)": ({"taxonomy", "contract"}, {"taxonomy"},
                            {"agent", "surface", "capabilities", "root"}),
    "capabilities": ({"taxonomy", "contract", "utility"}, {"taxonomy", "contract(protocol)"},
                     {"surface", "agent", "capabilities", "root"}),
    "agent(orchestrator)": ({"taxonomy", "contract(aggregate)", "contract(protocol)", "utility"},
                            {"taxonomy", "contract(aggregate)"},
                            {"surface", "capabilities", "root"}),
    "root": ({"taxonomy", "contract", "capabilities", "agent", "surface"}, set(), set()),
    "surface(command|controller|page)": ({"taxonomy", "contract(aggregate)", "utility"}, set(),
                                         {"agent", "capabilities", "contract(protocol)", "root"}),
    "surface(hook|store|action|screen|router)": (
        {"taxonomy", "contract(aggregate)", "utility"}, set(),
        {"agent", "capabilities", "contract(protocol)",
         "surface(command|controller|page|entry)", "root"}),
    "surface(component|view|layout)": (
        {"taxonomy", "contract(aggregate)", "utility"}, set(),
        {"agent", "contract", "capabilities",
         "surface(command|controller|page|entry|hook|store|action|screen|router)", "root"}),
}

MAX_LINES = 1000
MIN_LINES = 5

TEST_TYPE_PREFIXES = ["contract_", "unit_", "integration_", "dogfood_", "smoke_", "e2e_", "acceptance_"]
TEST_SUPPORT_PREFIXES = ["regression_", "behavioral_", "mock_", "fixture_"]
BENCH_PREFIX = "bench_"
TEST_SUPPORT_DIRS = {"common"}

COMMENT_PATTERNS = [
    (r"#\[allow\(", "BYPASS_COMMENT", "#[allow(...)]"),
    (r"#\s*noqa", "BYPASS_COMMENT", "noqa"),
    (r"type:\s*ignore", "BYPASS_COMMENT", "type: ignore"),
    (r"eslint-disable", "BYPASS_COMMENT", "eslint-disable"),
    (r"@ts-ignore", "BYPASS_COMMENT", "@ts-ignore"),
    (r"@ts-expect-error", "BYPASS_COMMENT", "@ts-expect-error"),
    (r"\bFIXME\b", "BYPASS_COMMENT", "FIXME"),
    (r"\bHACK\b", "BYPASS_COMMENT", "HACK"),
    (r"\bXXX\b", "BYPASS_COMMENT", "XXX"),
    (r"raise\s+NotImplementedError", "UNIMPLEMENTED", "raise NotImplementedError"),
    (r"\bassert\s+False\b", "PANIC", "assert False"),
    (r"throw\s+new\s+(Error|TypeError|RangeError|ReferenceError|SyntaxError)", "PANIC", "throw new Error"),
]

def _strip_line(line: str, drop_comments: bool) -> str:
    """Blank string/char literals (and optionally comments) on a single line.

    Line-wise by design: a mis-detected quote must never swallow the rest of
    the file, which is what a whole-file regex does.
    """
    out = []
    i, n = 0, len(line)
    while i < n:
        ch = line[i]
        if ch == "/" and i + 1 < n and line[i + 1] == "/" and drop_comments:
            break
        if ch == "r" and i + 1 < n and line[i + 1] in '#"':
            j = i + 1
            hashes = 0
            while j < n and line[j] == "#":
                hashes += 1
                j += 1
            if j < n and line[j] == '"':
                close = '"' + "#" * hashes
                k = line.find(close, j + 1)
                if k == -1:
                    out.append(" " * (n - i))
                    i = n
                else:
                    out.append(" " * (k + len(close) - i))
                    i = k + len(close)
                continue
        if ch == '"':
            j = i + 1
            while j < n:
                if line[j] == "\\":
                    j += 2
                    continue
                if line[j] == '"':
                    j += 1
                    break
                j += 1
            out.append(" " * (j - i))
            i = j
            continue
        if ch == "'" and i + 2 < n and (line[i + 2] == "'" or (line[i + 1] == "\\" and i + 3 < n)):
            j = i + 1
            while j < n:
                if line[j] == "\\":
                    j += 2
                    continue
                if line[j] == "'":
                    j += 1
                    break
                j += 1
            out.append(" " * (j - i))
            i = j
            continue
        out.append(ch)
        i += 1
    return "".join(out)


def strip_macros(text: str) -> str:
    """Remove `macro_rules!` bodies — a macro is token soup, not a declaration."""
    out, i = [], 0
    while True:
        idx = text.find("macro_rules!", i)
        if idx == -1:
            out.append(text[i:])
            return "".join(out)
        out.append(text[i:idx])
        brace = text.find("{", idx)
        if brace == -1:
            out.append(text[idx:])
            return "".join(out)
        depth, k, closed = 0, brace, -1
        while k < len(text):
            if text[k] == "{":
                depth += 1
            elif text[k] == "}":
                depth -= 1
                if depth == 0:
                    closed = k + 1
                    break
            k += 1
        if closed == -1:
            return "".join(out)
        i = closed


def code_only(text: str) -> str:
    """Code with string literals and comments removed (word-token checks)."""
    return "\n".join(_strip_line(l, True) for l in text.splitlines())


def no_strings(text: str) -> str:
    """Full text with string literals removed, comments kept (comment-pattern checks)."""
    return "\n".join(_strip_line(l, False) for l in text.splitlines())


def strip_cfg_test(text: str) -> str:
    """Drop `#[cfg(test)]` items (brace-matched): the shipped checkers skip them."""
    out, i, n = [], 0, len(text)
    attr = re.compile(r"^[ \t]*#\[cfg\(test\)\]", re.M)
    while True:
        found = attr.search(text, i)
        if found is None:
            out.append(text[i:])
            return "".join(out)
        idx = found.start() + (len(found.group(0)) - len(found.group(0).lstrip()))
        out.append(text[i:idx])
        brace = text.find("{", idx)
        if brace == -1:
            out.append(text[idx:])
            return "".join(out)
        depth, k, closed = 0, brace, -1
        while k < n:
            if text[k] == "{":
                depth += 1
            elif text[k] == "}":
                depth -= 1
                if depth == 0:
                    closed = k + 1
                    break
            k += 1
        if closed == -1:
            return "".join(out)
        i = closed


WORD_TOKENS = [
    (r"(?<![\w.])(?:unwrap|expect)\s*\(", "UNWRAP_EXPECT", "unwrap/expect"),
    (r"(?<![\w.])panic!\s*\(", "PANIC", "panic!"),
    (r"(?<![\w.])todo!\s*\(", "TODO", "todo!"),
    (r"(?<![\w.])(?:unimplemented|unreachable)!\s*\(", "UNIMPLEMENTED", "unimplemented!/unreachable!"),
]

def layer_of(stem: str):
    for prefix, layer in LAYER_PREFIXES:
        if stem.startswith(prefix):
            return layer
    return None


def suffix_of(stem: str) -> str:
    return stem.rsplit("_", 1)[-1] if "_" in stem else ""


def scope_of(stem: str):
    layer = layer_of(stem)
    if layer is None:
        return None
    suffix = suffix_of(stem)
    if layer == "taxonomy":
        if suffix == "vo":
            return "taxonomy(vo)"
        if suffix in ("request", "response"):
            return "taxonomy(request,response)"
        if suffix in ("entity", "error", "event"):
            return "taxonomy(entity,error,event)"
        if suffix == "constant":
            return "taxonomy(constant)"
        return "taxonomy(vo)"
    if layer == "contract":
        return f"contract({suffix})" if suffix in ("protocol", "aggregate") else "contract(protocol)"
    if layer == "agent":
        return "agent(orchestrator)"
    if layer == "surface":
        if suffix in ("command", "controller", "page"):
            return "surface(command|controller|page)"
        if suffix in ("hook", "store", "action", "screen", "router"):
            return "surface(hook|store|action|screen|router)"
        if suffix in ("component", "view", "layout"):
            return "surface(component|view|layout)"
        return "surface(command|controller|page)"
    return layer


def rust_sources():
    for base, _dirs, files in os.walk(CRATES):
        for f in files:
            if f.endswith(".rs"):
                yield os.path.join(base, f)


def rel(path: str) -> str:
    return os.path.relpath(path, ROOT)


def read(path: str) -> str:
    with open(path, encoding="utf-8", errors="replace") as fh:
        return fh.read()


def is_test_or_bench(path: str) -> bool:
    return bool(DEFAULT_IGNORED & set(path.split(os.sep)))


# ─── main ─────────────────────────────────────────────────────────────────
def main() -> int:
    findings = defaultdict(list)
    files = list(rust_sources())
    raw, code, textv = {}, {}, {}
    for p in files:
        try:
            raw[p] = read(p)
        except OSError:
            raw[p] = ""
        # order matters: blank strings first (so a literal "#[cfg(test)]" in a
        # string cannot be brace-matched), then drop cfg(test) items, then strip
        # comments for the code view.
        base = strip_macros(strip_cfg_test(no_strings(raw[p])))
        code[p] = code_only(base)
        textv[p] = base

    # AES101 / AES102 — production source naming, suffix policy
    for p in files:
        name = os.path.basename(p)
        stem = name[:-3]
        if is_test_or_bench(p) or name in ENTRY:
            continue
        if not re.fullmatch(r"[a-z0-9]+(_[a-z0-9]+){2,}", stem):
            findings["AES101"].append(
                f"{rel(p)} — stem '{stem}' is not lowercase prefix_concept_role (>=3 words)")
            continue
        layer = layer_of(stem)
        if layer is None:
            findings["AES101-unprefixed"].append(f"{rel(p)} — '{stem}' carries no AES layer prefix")
            continue
        policy, allowed, forbidden = SUFFIX_POLICY[layer]
        suf = suffix_of(stem)
        if suf in forbidden:
            findings["AES102"].append(f"{rel(p)} — suffix '{suf}' forbidden in layer '{layer}'")
        elif policy == "strict" and allowed and suf not in allowed:
            findings["AES102"].append(
                f"{rel(p)} — suffix '{suf}' not allowed for strict layer '{layer}' "
                f"(allowed: {', '.join(allowed)})")

    # AES103 — test/bench file prefix discipline
    for p in files:
        if not is_test_or_bench(p):
            continue
        name = os.path.basename(p)
        stem = name[:-3]
        if name in ("mod.rs", "lib.rs"):
            continue
        parts = rel(p).split("/")
        kind = "tests" if "tests" in parts else "benches"
        tail = parts[parts.index(kind) + 1:-1]
        if tail and not set(tail) <= TEST_SUPPORT_DIRS:
            findings["AES103"].append(f"{rel(p)} — nested in {kind}/{'/'.join(tail)} (must stay flat)")
            continue
        legal = TEST_TYPE_PREFIXES + TEST_SUPPORT_PREFIXES if kind == "tests" else [BENCH_PREFIX]
        if not any(stem.startswith(pre) for pre in legal):
            findings["AES103"].append(f"{rel(p)} — '{stem}' lacks a legal {kind} prefix")

    # AES301 / AES302 / AES303 — size + definition floor
    for p, text in raw.items():
        name = os.path.basename(p)
        stem = name[:-3]
        if name in ENTRY or stem.endswith("_constant") or is_test_or_bench(p):
            continue
        lines = text.count("\n") + 1
        if lines > MAX_LINES:
            findings["AES301"].append(f"{rel(p)} — {lines} lines (> {MAX_LINES})")
        if lines < MIN_LINES:
            findings["AES302"].append(f"{rel(p)} — {lines} lines (< {MIN_LINES})")
        if not re.search(r"^\s*(pub(\([^)]*\))?\s+)?(struct|enum|trait|union)\s", code[p], re.M):
            findings["AES303"].append(f"{rel(p)} — no struct/enum/trait definition")
        dead = []
        lines_l = code[p].splitlines()
        for idx, line in enumerate(lines_l):
            stripped = re.sub(r"^\s*(?:pub(?:\([^)]*\))?\s+)?", "", line.strip())
            if stripped.startswith("struct ") and stripped.endswith(";") and "(" not in stripped:
                if idx > 0 and lines_l[idx - 1].strip().startswith("#[derive("):
                    continue
                nxt = idx + 1
                while nxt < len(lines_l) and (
                    not lines_l[nxt].strip() or lines_l[nxt].strip().startswith(("#", "//"))
                ):
                    nxt += 1
                if nxt >= len(lines_l) or not lines_l[nxt].strip().startswith("impl "):
                    dead.append(f"unit struct without impl (line {idx + 1})")
        if dead:
            findings["AES303-dead"].append(f"{rel(p)} — {', '.join(dead)}")

    # AES304 — bypass patterns (word tokens read code only; comment markers read full text)
    for p in files:
        if is_test_or_bench(p):
            continue
        body = strip_cfg_test(no_strings(raw[p]))
        code_view = code_only(body)
        text_view = body
        for pattern, kind, label in WORD_TOKENS:
            hits = len(re.findall(pattern, code_view))
            if hits:
                findings["AES304"].append(f"{rel(p)} — {label} x{hits} ({kind})")
        for pattern, kind, label in COMMENT_PATTERNS:
            hits = len(re.findall(pattern, text_view))
            if hits:
                findings["AES304"].append(f"{rel(p)} — {label} x{hits} ({kind})")

    # AES403 — capabilities role
    for p in files:
        stem = os.path.basename(p)[:-3]
        if layer_of(stem) != "capabilities" or is_test_or_bench(p):
            continue
        text = code[p]
        types = re.findall(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|trait|union)\s+\w+", text, re.M)
        if len(types) > 3:
            findings["AES403"].append(f"{rel(p)} — {len(types)} type declarations (max 3)")
        protocols = set(re.findall(r"^\s*impl\s+(I\w*Protocol)\s+for\s+", text, re.M))
        if not protocols:
            findings["AES403"].append(f"{rel(p)} — no `impl I*Protocol for` (CapabilityNoImplementor)")
        elif len(protocols) > 1:
            findings["AES403"].append(f"{rel(p)} — implements {len(protocols)} protocols {sorted(protocols)}")
        markers = [int(n) for n in re.findall(r"Block\s+(\d+)\s*:", textv[p])]
        if not markers:
            findings["AES403"].append(f"{rel(p)} — no `Block N:` banners (CapabilityBlockMarkers)")
        else:
            missing = [n for n in (1, 2, 3) if n not in markers]
            if missing:
                findings["AES403"].append(f"{rel(p)} — missing Block marker(s) {missing}")
            if markers != sorted(markers):
                findings["AES403"].append(f"{rel(p)} — block markers out of order {markers}")
            if any(n > 3 for n in markers):
                findings["AES403"].append(f"{rel(p)} — block markers beyond Block 3 {sorted(set(markers))}")
        if re.search(r"^const\s+\w+", text, re.M):
            findings["AES403"].append(f"{rel(p)} — file-level `const` inline (CapabilityLocalConstant)")
        if re.search(r"^\s*#\[cfg\(test\)\]|^\s*(?:pub\s+)?mod\s+tests\b", text, re.M):
            findings["AES403"].append(f"{rel(p)} — inline #[cfg(test)] / mod tests (CapabilityEmbeddedTest)")

    # AES404 — utility role purity
    for p in files:
        stem = os.path.basename(p)[:-3]
        if layer_of(stem) != "utility" or is_test_or_bench(p):
            continue
        text = code[p]
        bad = []
        if re.search(r"^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+\w+", text, re.M):
            bad.append("struct")
        if re.search(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:trait|enum|union)\s+\w+", text, re.M):
            bad.append("trait/enum")
        if re.search(r"^\s*impl\s+[\w:<>, ]+\s*(?:for\s+[\w:<>, ]+)?\{", text, re.M):
            bad.append("impl block")
        if re.search(r"^\s*type\s+\w+\s*=", text, re.M):
            bad.append("type alias")
        if bad:
            findings["AES404"].append(f"{rel(p)} — utility file declares {', '.join(sorted(set(bad)))}")

    # AES405 — agent orchestrator role
    for p in files:
        stem = os.path.basename(p)[:-3]
        if layer_of(stem) != "agent" or is_test_or_bench(p):
            continue
        text = code[p]
        if re.search(r"^\s*impl\s+I\w*Protocol\s+for\s+", text, re.M):
            findings["AES405"].append(f"{rel(p)} — orchestrator implements a contract protocol")
        if not re.search(r"^\s*impl\s+(?:<[^>]*>\s*)?\w*Aggregate\w*(?:<[^>]*>)?\s+for\s+", text, re.M):
            findings["AES405"].append(f"{rel(p)} — no aggregate-trait implementor (AgentRouting)")
        types = re.findall(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|trait|union)\s+\w+", text, re.M)
        if len(types) > 3:
            findings["AES405"].append(f"{rel(p)} — {len(types)} type declarations (max 3)")
        if re.search(r"(?<![\w:])Any(?![\w])", text):
            findings["AES405"].append(f"{rel(p)} — `Any` type used (AgentRouting)")

    # AES201 — import-boundary matrix
    use_re = re.compile(r"^\s*use\s+([A-Za-z_][\w:{} ,*]*);", re.M)
    for p in files:
        stem = os.path.basename(p)[:-3]
        if is_test_or_bench(p):
            continue
        scope = scope_of(stem)
        if scope is None or scope not in IMPORT_MATRIX:
            continue
        _allowed, _mandatory, forbidden = IMPORT_MATRIX[scope]
        for raw_use in use_re.findall(code[p]):
            for path in raw_use.split(","):
                path = path.strip().strip("{}").strip()
                if not path:
                    continue
                target = None
                for seg in path.split("::"):
                    seg = seg.strip()
                    for prefix, lay in LAYER_PREFIXES:
                        if seg.startswith(prefix):
                            target = lay
                            break
                    if target:
                        break
                if target is None:
                    continue
                if target in forbidden:
                    findings["AES201"].append(
                        f"{rel(p)} [{scope}] — imports forbidden layer '{target}': `use {path};`")

    # AES701 shared-folder purity
    shared_src = os.path.join(CRATES, "shared", "src")
    for entry in sorted(os.listdir(shared_src)):
        folder = os.path.join(shared_src, entry)
        if not os.path.isdir(folder):
            continue
        for f in sorted(os.listdir(folder)):
            stem = f[:-3] if f.endswith(".rs") else f
            lay = layer_of(stem)
            if lay in ("capabilities", "agent", "surface"):
                findings["AES701"].append(f"{rel(folder)}/{f} — shared folder holds a '{lay}' file")
    for doc in ("DATA.md", "BACKLOG.md"):
        if not os.path.isfile(os.path.join(CRATES, "shared", doc)):
            findings["AES701"].append(f"crates/shared — missing {doc}")

    # AES702 / AES703 / AES705
    feature_crates, surface_crates = set(), set()
    for entry in sorted(os.listdir(CRATES)):
        folder = os.path.join(CRATES, entry)
        src = os.path.join(folder, "src")
        if not os.path.isdir(src) or entry == "shared":
            continue
        layers = defaultdict(list)
        for f in sorted(os.listdir(src)):
            if f.endswith(".rs"):
                layers[layer_of(f[:-3])].append(f)
        if layers.get("surface"):
            surface_crates.add(entry)
        if layers.get("capabilities") or layers.get("agent"):
            feature_crates.add(entry)
        for f in layers.get(None, []):
            if f not in ENTRY:
                findings["AES702"].append(f"crates/{entry}/src/{f} — no AES layer prefix")
        if layers.get("capabilities") or layers.get("agent"):
            if not layers.get("agent"):
                findings["AES702"].append(f"crates/{entry} — feature_missing_agent")
            if not layers.get("capabilities"):
                findings["AES702"].append(f"crates/{entry} — feature_missing_capability")
            if not layers.get("root"):
                findings["AES702"].append(f"crates/{entry} — feature_missing_root_container")
            for doc in ("FRD.md", "BACKLOG.md"):
                if not os.path.isfile(os.path.join(folder, doc)):
                    findings["AES702"].append(f"crates/{entry} — feature doc pair missing {doc}")
            mirror = os.path.join(shared_src, entry.replace("-", "_"))
            if not os.path.isdir(mirror):
                findings["AES702"].append(
                    f"crates/{entry} — feature_missing_shared_subfolder (shared/src/{entry.replace('-', '_')})")
        if layers.get("surface"):
            for doc in ("DESIGN.md", "BACKLOG.md"):
                if not os.path.isfile(os.path.join(folder, doc)):
                    findings["AES703"].append(f"crates/{entry} — surface doc pair missing {doc}")
            for lay in ("capabilities", "agent", "utility"):
                for f in layers.get(lay, []):
                    findings["AES703"].append(f"crates/{entry}/src/{f} — surface folder holds a '{lay}' file")

    for entry in sorted(os.listdir(CRATES)):
        if os.path.isfile(os.path.join(CRATES, entry)):
            stem = entry[:-3] if entry.endswith(".rs") else entry
            lay = layer_of(stem)
            if lay in ("capabilities", "agent", "surface", "utility"):
                findings["AES705"].append(f"crates/{entry} — '{lay}' file at member root")

    # AES704 test-suite coverage
    for entry in sorted(feature_crates):
        folder = os.path.join(CRATES, entry)
        tests, benches = os.path.join(folder, "tests"), os.path.join(folder, "benches")
        if not os.path.isdir(tests):
            findings["AES704"].append(f"crates/{entry} — test_suite_missing_tests_dir")
            continue
        if not os.path.isdir(benches) or not any(f.startswith(BENCH_PREFIX) for f in os.listdir(benches)):
            findings["AES704"].append(f"crates/{entry} — test_suite_missing_bench_dir")
        names = [f[:-3] for f in os.listdir(tests) if f.endswith(".rs")]
        for prefix in TEST_TYPE_PREFIXES:
            if not any(n.startswith(prefix) for n in names):
                findings["AES704"].append(f"crates/{entry} — test_suite_missing_category '{prefix}'")

    # AES401 — taxonomy role (constant purity + entity/error/event primitives)
    prims = ["String", "str", "char", "u8", "u16", "u32", "u64", "usize",
             "i8", "i16", "i32", "i64", "isize", "f32", "f64"]
    non_const = re.compile(
        r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|trait|fn|impl|mod|type|union)\b", re.M)
    for p in files:
        stem = os.path.basename(p)[:-3]
        if is_test_or_bench(p) or layer_of(stem) != "taxonomy":
            continue
        if stem.endswith("_constant"):
            hits = non_const.findall(code[p])
            if hits:
                findings["AES401"].append(
                    f"{rel(p)} — _constant file carries {len(hits)} non-const declaration(s)")
        if re.search(r"_(entity|error|event)$", stem):
            for lineno, line in enumerate(code[p].splitlines(), 1):
                t = line.strip()
                if not re.search(r"\bpub\s+\w+\s*:", t):
                    continue
                ty = t.split(":", 1)[1]
                for pr in prims:
                    if re.search(rf"(?<![\w:]){pr}(?![\w])", ty):
                        findings["AES401"].append(f"{rel(p)}:{lineno} — primitive field type '{pr}'")
                        break

    # AES202 — mandatory import presence
    for p in files:
        stem = os.path.basename(p)[:-3]
        if is_test_or_bench(p):
            continue
        scope = scope_of(stem)
        if scope is None or scope not in IMPORT_MATRIX:
            continue
        # compare on base layer names: `contract(protocol)` is satisfied by any
        # contract import (the sub-scope refinement is a same-layer detail)
        mandatory = {m.split("(")[0] for m in IMPORT_MATRIX[scope][1]}
        if not mandatory:
            continue
        present = set()
        for seg in re.findall(r"[A-Za-z_][\w]*", code[p]):
            for prefix, lay in LAYER_PREFIXES:
                if seg.startswith(prefix):
                    present.add(lay)
        missing = sorted(mandatory - present)
        if missing:
            findings["AES202"].append(
                f"{rel(p)} [{scope}] — no import from mandatory layer(s) {missing}")

    # DOC-DRIFT — rule inventory parity (ARCHITECTURE.md vs shipped rule table vs repo config)
    arch = read(os.path.join(ROOT, "ARCHITECTURE.md"))
    index_rows = [l for l in arch.splitlines() if l.startswith("| AES")]
    doc_codes = sorted({c for c in re.findall(r"AES\d{3}", "\n".join(index_rows))})
    rules_src = read(os.path.join(CRATES, "shared", "src", "config_system",
                                 "config_architecture_rules.rs"))
    table = rules_src[rules_src.index("static RULES"):rules_src.index("// ─── Public API")]
    table_codes = sorted(set(re.findall(r'"(AES\d{3})"', table)))
    cfg_path = os.path.join(ROOT, "lint_arwaky.config.yaml")
    cfg_codes = []
    if os.path.isfile(cfg_path):
        cfg_codes = ["AES" + c for c in sorted(set(re.findall(r"^\s+AES(\d{3}):", read(cfg_path), re.M)))]
    for code in sorted(set(doc_codes) - set(table_codes)):
        findings["DOC-DRIFT-inventory"].append(
            f"{code} — documented but absent from the hardcoded AES rule table "
            f"(cannot be toggled or given exceptions via config)")
    for code in sorted(set(doc_codes) - set(cfg_codes)):
        findings["DOC-DRIFT-inventory"].append(
            f"{code} — documented but absent from the repository's lint_arwaky.config.yaml rule list")
    for code in sorted(set(table_codes) - set(doc_codes)):
        findings["DOC-DRIFT-inventory"].append(f"{code} — in the rule table but not in ARCHITECTURE.md")
    findings["_meta"] = [
        f"documented codes: {len(doc_codes)}",
        f"hardcoded rule-table codes: {len(table_codes)}",
        f"repo-config codes: {len(cfg_codes)}",
    ]

    total = sum(len(v) for v in findings.values())
    print(f"Scanned {len(files)} Rust files under crates/ — {total} findings\n")
    for key in sorted(findings):
        rows = findings[key]
        print(f"### {key} ({len(rows)})")
        for row in rows:
            print(f"  - {row}")
        print()
    if "--json" in sys.argv:
        out = os.path.join(ROOT, "scripts", "architecture_audit.json")
        with open(out, "w", encoding="utf-8") as fh:
            json.dump(findings, fh, indent=2)
        print(f"wrote {rel(out)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
