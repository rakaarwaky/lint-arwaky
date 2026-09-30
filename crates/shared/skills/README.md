# Provisioned Skills

The skill pack that `lint-arwaky init` writes into a target project. Each
subdirectory is one skill, holding a `SKILL.md` at its root and an optional
`references/` folder of per-language playbooks.

## Prerequisites

- A built `lint-arwaky-cli` binary. The skill sources are embedded into the
  binary at compile time, so a rebuilt binary is what actually ships the skills.
- `python3` on the path, used only by the regeneration script below.

## Quick Start

```bash
# 1. Edit the markdown source in this folder.
# 2. Regenerate the embedded constant after adding, removing, or renaming a file.
python3 tools/regenerate_skills.py

# 3. Rebuild, then provision into a target project.
CARGO_INCREMENTAL=0 cargo build --release
./target/release/lint-arwaky-cli init <target-project>
```

Step 2 is required only when the *set* of skill files changes. Editing the body
of an existing file changes what the next build embeds, and the build runs
after step 2 anyway.

## Architecture

Skills are embedded into the binary at compile time via `include_str!` in the
generated skills constant under the shared crate's project-setup module. The
constant is the single source of truth at runtime; the markdown in this folder
is the source of truth in the repository.

```text
crates/shared/skills/
├── <skill-name>/SKILL.md                     # one directory per skill
└── <skill-name>/references/HOW-TO-MAKE-*.md  # optional per-language playbooks

        │  tools/regenerate_skills.py
        ▼
crates/shared/src/project_setup/  (generated skills constant)
        │  include_str! at compile time
        ▼
lint-arwaky-cli binary
        │  `lint-arwaky init` writes the embedded set
        ▼
<target-project>/.agents/skills/
```

`init` writes only the `SKILL.md` for every skill plus the `references/` files
whose language matches the languages detected in the target project. It
overwrites a provisioned skill file in place, so a skill that changed upstream
lands in the project on the next run. Skills that exist only in the project —
anything not in this folder — are left alone.

## Project Structure

| Path | Purpose |
| --- | --- |
| `crates/shared/skills/<name>/SKILL.md` | The skill body: trigger, purpose, and routing. |
| `crates/shared/skills/<name>/references/` | Per-language HOW-TO playbooks, filtered at provision time. |
| `tools/regenerate_skills.py` | Regenerates the embedded constant from this folder. |
| `crates/shared/src/project_setup/` | The generated constant that `init` reads from. |

All provisioned skills follow the `aes-<layer>` convention: `aes-taxonomy`,
`aes-contract`, `aes-capabilities`, `aes-agent`, `aes-surface`, `aes-utility`,
`aes-root`, `aes-docs`, `aes-migration`, `aes-lint-arwaky`, `aes-testing-suite`.

## Available Scripts

| Command | When to run it |
| --- | --- |
| `python3 tools/regenerate_skills.py` | After adding, removing, or renaming a skill file. |
| `CARGO_INCREMENTAL=0 cargo build --release` | After any skill content change, to re-embed. |
| `lint-arwaky-cli init <project>` | To provision the pack into a target project. |
| `lint-arwaky-cli check docs <path>` | To audit the provisioned pack for broken links. |

## Configuration

Skill provisioning has no user-facing configuration. Two behaviours are worth
knowing:

- **Language filtering.** `init` detects the target project's languages and
  provisions matching `references/` files. A Rust-only project receives the Rust
  playbooks and none of the Python or TypeScript ones.
- **Regeneration is explicit.** The embedded constant is checked in, so a new
  skill directory has no effect until the regeneration script has run.

Skills used only for working on lint-arwaky itself live in `.agents/skills/` at
the repository root and are not distributed to target projects.

## Testing

The pack is verified by the document-invariant gate, which resolves every link
in every `SKILL.md` and reference file and reports any pointer an agent would
follow into nothing:

```bash
lint-arwaky-cli check docs crates/shared/skills
```

A clean run means every skill is reachable and every cross-reference resolves.

## Contributing

1. Edit or add a skill directory in this folder, following the `aes-<layer>`
   naming convention.
2. Run `python3 tools/regenerate_skills.py` and commit the regenerated
   constant alongside the markdown change.
3. Run `lint-arwaky-cli check docs crates/shared/skills` and confirm it is clean.

Skill content changes land in the binary at the next build, so a skill fix
reaches target projects on the next `init` run of a rebuilt binary.

## License

Same license as the rest of the workspace — see [LICENSE](../../LICENSE).
