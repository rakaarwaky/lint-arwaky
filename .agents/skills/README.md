# Lint Arwaky Skills

Each skill is a self-contained directory with a `SKILL.md` file defining rules, patterns, and workflows.

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
├── role-architect                   # USE for architecture review & layer validation
├── role-business-analyst            # USE for requirements & business flow review
├── role-fullstack-developer         # USE to execute plans & implement fixes
├── role-quality-analysis            # USE for PR review, CI gates, merge readiness
├── role-tech-lead                   # USE for code quality, security & performance review
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
