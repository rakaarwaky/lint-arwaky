# AGENTS Workflow

Follow these steps in order. The output must match the Template exactly.

## 0. Choose the entry scenario

The entry point depends on the starting state. In every scenario the AGENTS.md is a behavior contract: it states how agents should act, never what the current source code does. The user's `<instruction>` overrides the code — code never edits the spec.

### Scenario 1 — Update existing AGENTS.md

**When:** an `AGENTS.md` already exists for the agent/tool and needs to be improved.
**Argument hint:** `--improve --docpath <docpath>`
`<docpath>` is the path to the existing `AGENTS.md`. `<instruction>` describes what to change. The delegate reads the current AGENTS.md, applies the improvement, and keeps every rule stable; new rules are only added when the improvement declares new behaviors. Never derive agent behavior from source code — apply the instruction against the existing spec.

**Special case — `--docpath all`:** improves every existing `AGENTS.md` in the workspace in parallel. The orchestrator discovers all `AGENTS.md` files under feature/tool directories, then fans out one delegate subagent per `AGENTS.md`, each applying the same `<instruction>` to its own AGENTS.md while keeping that file's rules stable. The same shared rule applies: no source-code derivation, instruction overrides code.

### Scenario 2 — Create new AGENTS.md, no source code yet

**When:** the agent/tool has no implementation and no `AGENTS.md`.
**Argument hint:** `--design --docpath <docpath>`
`<docpath>` is the target path for the new `AGENTS.md`. `<instruction>` describes how the agent should act. The delegate enumerates the behavior set from the instruction and the feature's PRD/ROADMAP/DESIGN context, then writes the AGENTS.md purely from the promised behavior. No source code exists to consult, so the instruction is the sole source of truth.

### Scenario 3 — Create AGENTS.md for a project that already has source code

**When:** source code exists but there is no `AGENTS.md`, or the contract must be written independently of the build.
**Argument hint:** `--make --docpath <docpath>`
`<docpath>` is the target path for the new `AGENTS.md`. `<instruction>` describes how the agent should act. Existing source code is reference only: it may inform naming of caller-observable identifiers (tool names, command flags), but the AGENTS.md never names `.py`/`.rs`/`.ts`/`.tsx` files and never states how the current build works. Where code and instruction conflict, the instruction wins.

## 1. Determine context

Identify the target scope: a single tool or a multi-agent system. This decides which sections carry content and which stay `None`.

## 2. Write frontmatter

```yaml
trigger: always
description: "<one-line: what this AGENTS.md governs>"
```

## 3. Write AGENTS.md body

Fill each section in order, matching the Template shape:

1. **Agent Overview** — one paragraph: what agents must know before generating code, docs, or suggestions.
2. **Runtime** — `Aspect | Detail` table: Language, Environment, Artifacts, Package Manager, Version Probe, Setup, Activation.
3. **Project Structure** — fenced `text` tree block (`├──`/`└──` connectors with `# <what lives here>` comments), one node per crate/workspace member.
4. **Commands** — `Category | Command | Scope | CI Gate` table. Every command verbatim from the CI gate.
5. **Reference** — `Artifact | Purpose | Link` table: README, CONTRIBUTING, ARCHITECTURE, ROADMAP, PRD.

Rules: no absolute personal paths, no secrets, no code-file references, no restating other documents.

## 4. Verify

`lint-arwaky-cli docs` passes; every required section is present.

## Verify

```bash
lint-arwaky-cli docs .
# Checks: frontmatter, H1/H2 shape, line drift vs template, crosslinks, table columns, dead-link.
```
