#!/usr/bin/env python3
"""Analyze AES contract protocol files against the one-trait-one-method rule.

Rule (from crates/skills/aes-contract/references/HOW-TO-MAKE-RUST-CONTRACT.md):
  A `_protocol` file must declare exactly ONE pub trait with exactly ONE method.
  Extra methods belong in their own protocol file.

Usage:
  python3 tools/analyze_protocols.py            # report all violations
  python3 tools/analyze_protocols.py --json     # machine-readable
"""

import argparse
import json
import pathlib
import re
import subprocess

ROOT = pathlib.Path(__file__).resolve().parent.parent
SHARED = ROOT / "crates" / "shared" / "src"

TRAIT_RE = re.compile(r"^pub trait (\w+)")
FN_RE = re.compile(r"^\s{4}(?:pub )?fn (\w+)\s*\(")


def method_names(path: pathlib.Path) -> list[str]:
    """Extract method names from the single pub trait in a contract file."""
    names: list[str] = []
    in_trait = False
    for line in path.read_text().splitlines():
        if TRAIT_RE.match(line):
            in_trait = True
            continue
        if in_trait and line.startswith("}"):
            break
        if in_trait:
            m = FN_RE.match(line)
            if m:
                names.append(m.group(1))
    return names


def usage_count(trait: str) -> int:
    """Count files referencing a trait outside its own contract file."""
    out = subprocess.run(
        ["grep", "-rl", trait, "--include=*.rs", str(ROOT / "crates")],
        capture_output=True,
        text=True,
    )
    files = [f for f in out.stdout.split() if "contract_" not in f]
    return len(files)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--min-methods", type=int, default=2)
    args = ap.parse_args()

    violations = []
    for path in sorted(SHARED.rglob("contract_*_protocol.rs")):
        methods = method_names(path)
        if len(methods) < args.min_methods:
            continue
        rel = path.relative_to(ROOT)
        trait = TRAIT_RE.search(path.read_text())
        violations.append(
            {
                "file": str(rel),
                "trait": trait.group(1) if trait else "?",
                "methods": methods,
                "method_count": len(methods),
                "new_files_needed": len(methods) - 1,
                "usage_files": usage_count(trait.group(1)) if trait else 0,
            }
        )

    total_methods = sum(v["method_count"] for v in violations)
    total_new = sum(v["new_files_needed"] for v in violations)

    if args.json:
        print(
            json.dumps(
                {
                    "violating_files": len(violations),
                    "total_methods": total_methods,
                    "new_files_needed": total_new,
                    "violations": violations,
                },
                indent=2,
            )
        )
        return

    print(f"Protocol files violating one-trait-one-method: {len(violations)}")
    print(f"Total methods in those files: {total_methods}")
    print(f"New protocol files required: {total_new}\n")
    print(f"{'uses':>5} {'meth':>4}  file")
    for v in sorted(violations, key=lambda x: -x["usage_files"]):
        print(f"{v['usage_files']:5} {v['method_count']:4}  {v['file']}")


if __name__ == "__main__":
    main()
