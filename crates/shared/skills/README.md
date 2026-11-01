# README — Provisioned Skills

## Project Overview

`lint-arwaky init` provisions this skill pack into a target project. Each
subdirectory is one skill, holding a `SKILL.md` at its root and an optional
`references/` folder of per-language playbooks. The skill markdown in this
folder is the repository source of truth; it is embedded into the binary at
compile time via `include_str!` in the generated skills constant under the
shared crate's project-setup module, and `init` writes it into the target
project's `.agents/skills/` directory. All provisioned skills follow the
`aes-<layer>` convention: `aes-taxonomy`, `aes-contract`, `aes-capabilities`,
`aes-agent`, `aes-surface`, `aes-utility`, `aes-root`, `aes-docs`,
`aes-migration`, `aes-lint-arwaky`, `aes-testing-suite`.

## Prerequisites

- A built `lint-arwaky-cli` binary. The skill sources are embedded into the
  binary at compile time, so a rebuilt binary is what actually ships the skills.
- `python3` on the path, used only by the regeneration script below.

## Quick Start

| Step | Action | Command |
| --- | --- | --- |
| 1 | Edit the markdown source in this folder. | — |
| 2 | Update the embedded constant after adding, removing, or renaming a skill file. | `crates/shared/src/project_setup/taxonomy_project_setup_constant.rs` (manual update) |
| 3 | Rebuild, then provision into a target project. | `CARGO_INCREMENTAL=0 cargo build --release` → `./target/release/lint-arwaky-cli init <target-project>` |

Step 2 is required only when the *set* of skill files changes. Editing the
body of an existing file changes what the next build embeds, and the build
runs after step 2 anyway.

## Reference

| Artifact | Purpose | Link |
| --- | --- | --- |
| `crates/shared/skills/<name>/SKILL.md` | The skill body: trigger, purpose, and routing. | [SKILL.md](aes-agent/SKILL.md) |
| `crates/shared/skills/<name>/references/` | Per-language HOW-TO playbooks, filtered at provision time. | (language-matched `references/` subdirectory) |
| `crates/shared/src/project_setup/taxonomy_project_setup_constant.rs` | Maintains the embedded constant — update manually after adding, removing, or renaming a skill file. | (see Quick Start, Step 2) |
| `DESCRIPTION.md` | One-line pack description provisioned alongside the skills. | [DESCRIPTION.md](DESCRIPTION.md) |
| LICENSE | License shared with the rest of the workspace. | `LICENSE` at repo root |

Skill provisioning has no user-facing configuration. Two behaviours are worth
knowing:

- **Language filtering.** `init` detects the target project's languages and
  provisions matching `references/` files. A Rust-only project receives the
  Rust playbooks and none of the Python or TypeScript ones.
- **Regeneration is explicit.** The embedded constant is checked in, so a new
  skill directory has no effect until the regeneration script has run.

The pack is verified by the document-invariant gate, which resolves every link
in every `SKILL.md` and reference file and reports any pointer an agent would
follow into nothing:

```bash
lint-arwaky-cli docs crates/shared/skills
```

A clean run means every skill is reachable and every cross-reference resolves.
