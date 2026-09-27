# Provisioned Skills

Each subdirectory is one skill, holding a `SKILL.md` at its root.

## How it reaches a project

Skills are **embedded into the binary at compile time** via `include_str!` in
[`taxonomy_skills_constant.rs`](../../shared/src/project_setup/taxonomy_skills_constant.rs).
`lint-arwaky init` writes them directly from those constants into the target project's
`.agents/skills/`, filtered by the languages detected in that project.

1. Edit the markdown source here; changes land in the binary at the next build.
2. Run `python3 tools/regenerate_skills.py` after adding, removing, or renaming a skill file.
3. `lint-arwaky init` writes the filtered set to `.agents/skills/` on the target.

Init overwrites a provisioned skill file in place, so a skill that changed
upstream lands in the project on the next run. Skills that exist only in the
project — anything not in this folder — are left alone.

## Layout

skills/
├── <skill-name>/SKILL.md   # one directory per skill
└── <skill-name>/references/<HOW-TO-*.md>  # optional language-specific playbooks

## Naming

All provisioned skills follow the `aes-<layer>` convention:
`aes-taxonomy`, `aes-contract`, `aes-capabilities`, `aes-agent`,
`aes-surface`, `aes-utility`, `aes-root`, `aes-docs`, `aes-migration`,
`aes-lint-arwaky`, `aes-testing-suite`.

## Notes

Skills in this folder are the provisioned set. Skills used only for working on
lint-arwaky itself live in `.agents/skills/` and are not distributed to target
projects.
