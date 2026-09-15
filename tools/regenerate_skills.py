#!/usr/bin/env python3
"""Regenerate taxonomy_skills_constant.rs from .agents/skills on disk."""

import pathlib
import sys

REPO = pathlib.Path(sys.argv[1]).resolve()
SKILLS = REPO / ".agents" / "skills"
OUT = REPO / "crates" / "shared" / "src" / "project_setup" / "taxonomy_skills_constant.rs"
LANGS = {"python", "rust", "typescript"}

files = sorted(p.relative_to(SKILLS).as_posix() for p in SKILLS.rglob("*") if p.is_file())

entries = []
for rel in files:
    parts = rel.split("/")
    if len(parts) == 1:
        name, lang = parts[0], None                      # skills/README.md
    elif parts[-1] == "SKILL.md":
        name, lang = parts[0], None                      # <skill>/SKILL.md
    elif len(parts) == 3 and parts[1] == "references":
        stem = pathlib.Path(parts[2]).stem
        lang = stem if stem in LANGS else None
        name = f"{parts[0]}-{stem}"
    else:
        name, lang = "-".join(parts).rsplit(".", 1)[0], None
    entries.append((name, rel, lang))

lines = [
    "// PURPOSE: Embedded skills constants compiled directly into binary",
    "use crate::project_setup::taxonomy_skills_vo::EmbeddedSkillVO;",
    "",
    "/// All embedded skills compiled into the binary for initialization.",
    "///",
    "/// Each consolidated skill ships one language-agnostic `SKILL.md` (`language: None`,",
    "/// always installed) plus optional `references/<language>.md` halves that carry the",
    "/// per-language detail and are installed only when that language is detected.",
    "/// Regenerate with `python3 tools/regenerate_skills.py` after editing `.agents/skills/`.",
    "pub const EMBEDDED_SKILLS: &[EmbeddedSkillVO] = &[",
]
for name, rel, lang in entries:
    lines.append("    EmbeddedSkillVO::new(")
    lines.append(f'        "{name}",')
    lines.append(f'        "{rel}",')
    lines.append(f'        include_str!("../../../../.agents/skills/{rel}"),')
    lines.append(f'        {"None" if lang is None else f'Some("{lang}")'},')
    lines.append("    ),")
lines.append("];")
lines.append("")

OUT.write_text("\n".join(lines), encoding="utf-8")
py = sum(1 for _, _, l in entries if l == "python")
rs = sum(1 for _, _, l in entries if l == "rust")
ts = sum(1 for _, _, l in entries if l == "typescript")
gen = sum(1 for _, _, l in entries if l is None)
print(f"entries={len(entries)} python={py} rust={rs} typescript={ts} generic={gen}")
print(f"skills={len({e[1].split('/')[0] for e in entries})}")
