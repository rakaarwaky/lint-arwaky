# Lint Arwaky Skills

 Each skill is a self-contained directory with a `SKILL.md` file defining rules, patterns, and workflows.

## Structure

.agents/skills/
├── add-docs-<language></language>              # USE when you work on documentation
├── cleanup-files-<language></language>         # USE when you work on formatting
├── consolidate-files-<language></language>     # USE when you work on consolidation
├── create-agent-<language></language>          # USE when you work on Agent layer
├── create-capabilities-<language></language>   # USE when you work on Capabilities layer
├── create-contract-<language></language>       # USE when you work on Contract layer
├── create-surface-<language></language>        # USE when you work on Surface layer
├── create-taxonomy-<language></language>       # USE when you work on Taxonomy layer
├── create-test-<language></language>           # USE when you work on Test
├── fix-bypass-<language></language>            # USE when you remove bypass
└── lint-arwaky-<language></language>           # USE when you run scan vioaltion

## Naming Convention

All skills follow the pattern `<action>-<language>`:

- **Action**: What the skill does (create, fix, cleanup, add, lint, etc.)
- **Language**: Target language (`python` or `rust` or `typescript`)

## Usage

Each skill is invoked by trigger keywords defined in its `SKILL.md` file. The AI agent matches user requests to the appropriate skill based on triggers and context.

## AES Architecture Reference

Full 7-layer specification: [ARCHITECTURE.md](../../ARCHITECTURE.md)
