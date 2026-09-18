# Lint Arwaky Skills

Each skill is a self-contained directory with a `SKILL.md` file defining rules, patterns, and workflows.

## Prerequisites

- `lint-arwaky` installed or built from source (`cargo build --release`).
- A project with `crates/`, `modules/`, or `packages/` directories (or a mix).

## Quick Start

```bash
# List all bundled skills
ls crates/skills/

# Read a specific skill
cat crates/skills/lint-arwaky/SKILL.md

# Regenerate the embedded constant after editing any SKILL.md
python3 tools/regenerate_skills.py
```

## Project Structure

```
crates/skills/
├── add-docs/            # Documentation workflows
├── author-skill-md/     # Skill authoring guidance
├── cleanup-consolidate/ # Dead code removal, duplicate merging
├── create-agent/        # Agent layer scaffolding
├── create-capabilities/ # Capabilities layer scaffolding
├── create-contract/     # Contract layer scaffolding
├── create-root/         # Root/container scaffolding
├── create-surface/      # Surface layer scaffolding
├── create-taxonomy/     # Taxonomy layer scaffolding
├── create-utility/      # Utility layer scaffolding
├── fix-bypass/          # Bypass suppression cleanup
├── lint-arwaky/         # AES scan + fix workflow
├── setup-ci-quality-gates/  # CI, gates, branch protection
└── testing-suite/       # Test suite generation
```

## Available Scripts

| Script | Purpose |
|--------|---------|
| `python3 tools/regenerate_skills.py` | Rebuild `EMBEDDED_SKILLS` constant in `crates/shared/src/project_setup/taxonomy_skills_constant.rs` |

## Configuration

No external configuration. Skill content is embedded at build time via `include_str!` into the binary.

## Testing

Skills are validated by `lint-arwaky` self-lint (CI "Self-Lint" job) and by the test workspaces in `workspaces-bad/` / `workspaces-good/`. No dedicated test suite for the skill content itself.

## Contributing

- Each skill lives in its own directory with a `SKILL.md` and optional `references/` subdirectory.
- After adding, removing, or renaming a skill file, run `python3 tools/regenerate_skills.py`.
- Follow the naming conventions below.

## License

MIT — see [LICENSE](../../LICENSE).

## Structure

.agents/skills/
├── add-docs-<language>              # USE when you work on documentation
├── cleanup-consolidate-<language>   # USE when you work on cleanup & consolidation
├── create-agent-<language>          # USE when you work on Agent layer
├── create-capabilities-<language>   # USE when you work on Capabilities layer
├── create-contract-<language>       # USE when you work on Contract layer
├── create-root-<language>           # USE when you work on Root layer
├── create-surface-<language>        # USE when you work on Surface layer
├── create-skill-all                # USE when you create a new skill
├── create-taxonomy-<language>       # USE when you work on Taxonomy layer
├── create-test-<language>           # USE when you work on Test
├── create-utility-<language>        # USE when you work on Utility layer
├── fix-bypass-<language>            # USE when you remove bypass
├── lint-arwaky-<language>           # USE when you run scan violation
└── setup-ci-quality-gates           # USE to set up CI, quality gates, branch protection, AI review bots

## Naming Convention

### Language Skills

Follow the pattern `<action>-<language>`:

- **Action**: What the skill does (create, fix, cleanup, add, lint, etc.)
- **Language**: Target language (`python`, `rust`, or `typescript`)

### Role Skills

Follow the pattern `role-<role-name>`:

- **Role**: The review or execution role (architect, business-analyst, tech-lead, fullstack-developer)
- Role skills produce plan files (architect, business-analyst, tech-lead) or execute them (fullstack-developer)

### Process/Infrastructure Skills

Follow the pattern `setup-<subject>` (gerund/action form):

- **Subject**: What the skill sets up (ci-quality-gates, etc.)
- These are not tied to one language — they document processes, tooling, or repo infrastructure.

## Role Workflow

Roles follow a pipeline: **Architect** → **Business Analyst** → **Tech Lead** → **Fullstack Developer** → **Quality Analyst**

1. **Architect** reviews layer boundaries, naming, orphans, scalability, and data flow
2. **Business Analyst** reviews requirements clarity, business flow, logic implementation, and traceability
3. **Tech Lead** reviews security, performance, error handling, SOLID principles, and code quality
4. **Fullstack Developer** executes all plans and generates execution reports
5. **Quality Analyst** reviews PR from fullstack developer — CI gates, AES compliance, test results, report accuracy, merge readiness

## Usage

Each skill is invoked by trigger keywords defined in its `SKILL.md` file. The AI agent matches user requests to the appropriate skill based on triggers and context.

## AES Architecture Reference

Full 7-layer specification: [ARCHITECTURE.md](../../ARCHITECTURE.md)
