#!/usr/bin/env python3
"""Regenerate taxonomy_skills_constant.rs from crates/skills on disk.

Each skill ships one language-agnostic SKILL.md (always installed) plus
optional references/<HOW-TO-MAKE-*.md> files. Reference files whose
filename contains PYTHON, RUST, or TYPESCRIPT are installed only when that
language is detected; all others are language-agnostic.

Run:  python3 tools/regenerate_skills.py [repo-root]
"""

import pathlib
import re
import sys

REPO = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
SKILLS = REPO / "crates" / "skills"
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
    elif len(parts) >= 3 and parts[1] == "references":
        # <skill>/references/<HOW-TO>.md
        name = parts[0]
        lang = detect_lang(parts[2])
    else:
        name = "-".join(parts).rsplit(".", 1)[0]
        lang = None
    entries.append((name, rel, lang))

lines = [
    "// PURPOSE: Embedded skills constants compiled directly into binary",
    "use crate::project_setup::taxonomy_skills_vo::EmbeddedSkillVO;",
    "",
    "/// All embedded skills compiled into the binary for initialization.",
    "///",
    "/// Each skill ships a language-agnostic `SKILL.md` (always installed)",
    "/// plus optional language-specific `references/<HOW-TO-*.md>` files",
    "/// that are installed only when that language is detected.",
    "///",
    "/// The markdown source of truth lives in `crates/skills/` — edit the",
    "/// files directly there; `include_str!` picks up changes at compile time.",
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
    lines.append(f'        include_str!("../../../skills/{rel}"),')
    lines.append(f"        {lang_rs},")
    lines.append("    ),")
lines.append("];")
lines.append("")

OUT.write_text("\n".join(lines), encoding="utf-8")
py = sum(1 for *_, l in entries if l == "python")
rs = sum(1 for *_, l in entries if l == "rust")
ts = sum(1 for *_, l in entries if l == "typescript")
gen = sum(1 for *_, l in entries if l is None)
print(f"entries={len(entries)} python={py} rust={rs} typescript={ts} generic={gen}")
print(f"skills={len({e[1].split('/')[0] for e in entries if e[1] != 'README.md'})}")
