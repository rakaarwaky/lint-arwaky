# Lint Arwaky Skills

Each skill is a self-contained directory with a `SKILL.md` file defining rules, patterns, and
workflows. Language-specific detail lives in `references/<language>.md` inside the same directory,
so one skill covers Rust, Python and TypeScript instead of shipping three near-identical copies.

`lint-arwaky init` installs these files into a target project: every `SKILL.md` is always written,
and a `references/<language>.md` is written only when that language is detected in the project.
The list is compiled into the binary — regenerate
`crates/shared/src/project_setup/taxonomy_skills_constant.rs` with
`python3 tools/regenerate_skills.py` after editing this directory.

## Structure

```
.agents/skills/
├── add-docs/                    # USE when you work on documentation
├── author-skill-md/             # USE when you create a new skill
├── cleanup-consolidate/         # USE when you work on cleanup & consolidation
│   ├── SKILL.md                 # shared methodology
│   └── references/              # per-language procedure
│       ├── python.md
│       ├── rust.md
│       └── typescript.md
├── create-agent/                # USE when you work on the Agent layer
├── create-capabilities/         # USE when you work on the Capabilities layer
├── create-contract/             # USE when you work on the Contract layer
├── create-root/                 # USE when you work on the Root layer
├── create-surface/              # USE when you work on the Surface layer
├── create-taxonomy/             # USE when you work on the Taxonomy layer
├── create-utility/              # USE when you work on the Utility layer
├── fix-bypass/                  # USE when you remove bypass comments
├── lint-arwaky/                 # USE when you run a scan and fix violations
├── setup-ci-quality-gates/      # USE to set up CI, quality gates, branch protection, AI review bots
└── testing-suite/               # USE when you generate test suites
```

## Naming Convention

### Skills

One directory per skill, named after what it does, with no language suffix — the language is a
reference file inside the skill, not a separate skill: `create-taxonomy/references/python.md`.

### Layer Creation Skills

Follow the pattern `create-<layer>` where `<layer>` is one of the seven AES layers:
`taxonomy`, `contract`, `utility`, `capabilities`, `agent`, `surface`, `root`.

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

Each skill is invoked by trigger keywords defined in its `SKILL.md` file. The AI agent matches user
requests to the appropriate skill based on triggers and context, then opens the `references/` file
that matches the language it is about to write.

## AES Architecture Reference

Full 7-layer specification: [ARCHITECTURE.md](../../ARCHITECTURE.md)
