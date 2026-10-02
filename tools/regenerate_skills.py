#!/usr/bin/env python3
"""Regenerate taxonomy_project_setup_constant.rs from crates/shared/skills.

Each skill ships one language-agnostic SKILL.md (always installed) plus
optional reference(s)/<HOW-TO-MAKE-*.md> files. Reference files whose
filename contains PYTHON, RUST, or TYPESCRIPT are installed only when that
language is detected; all others are language-agnostic.

`crates/shared/skills/` is the single source of truth for the markdown.
`build.rs` copies it into `OUT_DIR` so the `include_str!` calls below can
resolve, and `[package] include` ships the same files in published crates.

Rust cannot enumerate a directory at compile time, so this script walks the
skills tree once and emits `EMBEDDED_SKILLS` — one `EmbeddedSkillVO` per file,
carrying the name, relative path, embedded content, and language tag. That
constant is what `lint-arwaky init` writes into a target project's
`.agents/skills/`.

Run after adding, removing, or renaming any file under `crates/shared/skills/`:

    python3 tools/regenerate_skills.py [repo-root]

The catalog is verified against the directory by the
`catalog_matches_the_skills_directory` test in
`crates/dispatcher/tests/unit_dispatcher_setup_skills.rs`, so a skill file
that was never embedded fails CI rather than shipping silently uninstalled.
"""

import pathlib
import re
import sys

REPO = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
SKILLS = REPO / "crates" / "shared" / "skills"
# The taxonomy constant was renamed to `taxonomy_project_setup_constant.rs` when
# the shared crate's filenames were consolidated to match their domain folder
# (#422). Writing to the old name produced an untracked orphan module that is
# declared in no `mod.rs`, so it drew AES501 (taxonomy orphan) and AES305
# (duplicate code) on the very next self-lint — the failure mode the rename was
# meant to prevent.
OUT = REPO / "crates" / "shared" / "src" / "project_setup" / "taxonomy_project_setup_constant.rs"

# Fail loudly rather than emitting a file nothing compiles. A wrong output path
# is the exact defect this script had, and it went unnoticed for a year because
# the script exits 0 either way.
if not OUT.parent.is_dir():
    sys.exit(f"error: output directory does not exist: {OUT.parent}")
if not SKILLS.is_dir():
    sys.exit(f"error: skills directory does not exist: {SKILLS}")

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
    # `crate::`, not `crate::project_setup::` — this file is a sibling module of
    # the VO it imports. The old spelling named a path that stopped existing
    # when the folder names changed, which is why the committed constant had to
    # be hand-corrected after the last regeneration.
    "use crate::taxonomy_project_setup_vo::EmbeddedSkillVO;",
    "",
    "/// All embedded skills compiled into the binary for initialization.",
    "///",
    "/// Each skill ships a language-agnostic `SKILL.md` (always installed)",
    "/// plus optional language-specific `reference(s)/<HOW-TO-*.md>` files",
    "/// that are installed only when that language is detected.",
    "///",
    "/// The markdown source of truth lives in `crates/shared/skills/`. `build.rs`",
    "/// copies it into OUT_DIR so `include_str!` picks up changes at compile time;",
    "/// `[package] include` ships the same source in the published crate.",
    "/// Regenerate this constant with `python3 tools/regenerate_skills.py` after",
    "/// adding, removing, or renaming a skill file —",
    "/// `catalog_matches_the_skills_directory` in",
    "/// `crates/dispatcher/tests/unit_dispatcher_setup_skills.rs` fails otherwise.",
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
    # NOTE: `crates/shared/skills/` is the single source of truth for skill
    # markdown. `build.rs` stages it into OUT_DIR at compile time; for published
    # crates the staging copy is packaged via `[package] include` in Cargo.toml.
    # After editing any skill file there, run this script — the
    # `catalog_matches_the_skills_directory` test fails on a stale catalog.
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
