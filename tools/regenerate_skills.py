#!/usr/bin/env python3
"""Regenerate taxonomy_skills_constant.rs from crates/shared/skills on disk.

Each skill ships one language-agnostic SKILL.md (always installed) plus
optional reference(s)/<HOW-TO-MAKE-*.md> files. Reference files whose
filename contains PYTHON, RUST, or TYPESCRIPT are installed only when that
language is detected; all others are language-agnostic.

The skills markdown lives in `crates/shared/skills/` (inside the shared crate
so it ships in the published tarball). `include_str!` resolves the path via
the OUT_DIR staged by `build.rs`.

Run:  python3 tools/regenerate_skills.py [repo-root]
"""

import pathlib
import re
import sys

REPO = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
SKILLS = REPO / "crates" / "shared" / "skills"
OUT = REPO / "crates" / "shared" / "src" / "project_setup" / "taxonomy_skills_constant.rs"

# Detect language from filename: PYTHON-TAXONOMY, RUST-AGENT, TYPESCRIPT-SURFACE, etc.
LANG_PATTERNS = [
    ("python", re.compile(r"PYTHON|python", re.IGNORECASE)),
    ("rust", re.compile(r"\bRUST\b|\brust\b")),
    ("typescript", re.compile(r"TYPESCRIPT|typescript", re.IGNORECASE)),
]


def detect_lang(filename: str) -> str | None:
    for lang, pat in LANG_PATTERNS:
        if pat.search(filename):
            return lang
    return None


files = sorted(p.relative_to(SKILLS).as_posix() for p in SKILLS.rglob("*.md") if p.is_file())

entries = []
for rel in files:
    parts = rel.split("/")
    if len(parts) == 1:
        # Top-level files (README.md)
        name = pathlib.Path(rel).stem
        lang = None
    elif parts[-1] == "SKILL.md":
        # <skill>/SKILL.md — language-agnostic
        name = parts[0]
        lang = None
    elif len(parts) >= 3 and parts[1] in ("references", "reference"):
        # <skill>/references/<HOW-TO>.md or <skill>/reference/<HOW-TO>.md
        name = parts[0]
        lang = detect_lang(parts[2])
    else:
        name = "-".join(parts).rsplit(".", 1)[0]
        lang = None
    entries.append((name, rel, lang))

lines = [
    "// PURPOSE: Embedded skills constants compiled directly into binary",
    "use crate::project_setup::taxonomy_setup_vo::EmbeddedSkillVO;",
    "",
    "/// All embedded skills compiled into the binary for initialization.",
    "///",
    "/// Each skill ships a language-agnostic `SKILL.md` (always installed)",
    "/// plus optional language-specific `reference(s)/<HOW-TO-*.md>` files",
    "/// that are installed only when that language is detected.",
    "///",
    "/// The markdown source of truth lives in `crates/shared/skills/` — edit the",
    "/// files directly there; `build.rs` stages them into OUT_DIR so",
    "/// `include_str!` picks up changes at compile time.",
    "/// Regenerate this constant with `python3 tools/regenerate_skills.py` after",
    "/// adding, removing, or renaming a skill file.",
    f"pub const EMBEDDED_SKILLS_COUNT: usize = {len(entries)};",
    "",
    "pub const EMBEDDED_SKILLS: &[EmbeddedSkillVO] = &[",
]
for name, rel, lang in entries:
    lang_rs = "None" if lang is None else f'Some("{lang}")'
    lines.append("    EmbeddedSkillVO::new(")
    lines.append(f'        "{name}",')
    lines.append(f'        "{rel}",')
    lines.append(f'        include_str!(concat!(env!("OUT_DIR"), "/skills/{rel}")),')
    # NOTE: `crates/shared/skills/` must match this directory for `build.rs`
    # staging to work when the crate is published on crates.io (which packages
    # only paths inside the crate root). After editing any skill file in
    # `crates/shared/skills/`, run `python3 tools/regenerate_skills.py` to
    # regenerate `taxonomy_skills_constant.rs`.
    lines.append(f"        {lang_rs},")
    lines.append("    ),")
lines.append("];")
lines.append("")

OUT.write_text("\n".join(lines), encoding="utf-8")

# Rustfmt canonizes the output so `cargo fmt --check` stays clean even when
# string lengths shift after adding or removing skill files. The formatter may
# fold long single-line include_str! spans across lines; keep it here so the
# committed constant file and the generator stay in lockstep.
import subprocess  # noqa: E402  (late import to delay until after OUT is written)
subprocess.run(
    ["rustfmt", str(OUT)],
    check=True,
    stdout=subprocess.DEVNULL,
    stderr=subprocess.DEVNULL,
)

py = sum(1 for *_, l in entries if l == "python")
rs = sum(1 for *_, l in entries if l == "rust")
ts = sum(1 for *_, l in entries if l == "typescript")
gen = sum(1 for *_, l in entries if l is None)
print(f"entries={len(entries)} python={py} rust={rs} typescript={ts} generic={gen}")
print(f"skills={len({e[1].split('/')[0] for e in entries if e[1] != 'README.md'})}")
