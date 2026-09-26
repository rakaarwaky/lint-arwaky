# Provisioned Skills

Each subdirectory is one skill, holding a `SKILL.md` at its root.

## How it reaches a project

1. `scripts/install.sh` (via `--mode local`, `global`, or `remote`) copies this
   folder into the XDG config directory at `~/.config/lint-arwaky/.agents/skills/`.
2. `lint-arwaky init` copies each skill from there into the target project's
   `.agents/skills/`.

Init overwrites a provisioned skill file in place, so a skill that changed
upstream lands in the project on the next run. Skills that exist only in the
project — anything not in this folder — are left alone.

## Layout

skills/
├── <skill-name>/SKILL.md   # one directory per skill

## Naming

- Language skills: `<action>-<language>` (e.g. `create-agent-rust`,
  `fix-bypass-python`, `lint-arwaky-typescript`)
- Role skills: `role-<role-name>` (e.g. `role-architect`, `role-tech-lead`)

## Notes

Skills in this folder are the provisioned set. Skills used only for working on
lint-arwaky itself live in `.agents/skills/` and are not distributed to target
projects.
