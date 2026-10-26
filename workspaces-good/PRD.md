# PRD — Calculator Workspace

> Product Requirements Document. Describes WHAT this project does and WHY.
> Audience: Stakeholders, PM, Design, Engineering leads.
> Condition: [ROADMAP.md](ROADMAP.md) (workspace) + each feature [BACKLOG.md](BACKLOG.md).
> This file is specification only.

## Problem Statement

A small multi-language calculator demo needs a single, clear specification of what it does and why, so that stakeholders, PMs, designers, and engineers agree on scope without reading source code.

## Goals & Success Metrics

| # | Goal                        | Measurement              | Target       |
|---|-----------------------------|--------------------------|--------------|
| 1 | Calculator evaluates expressions correctly | `lint-arwaky-cli check .` exit code | 0 |

## User Personas

- **Ops engineer on-call at 3 a.m.**: needs to verify the calculator produces correct results quickly, without reading source.
- **New contributor**: needs to understand how to build, test, and lint the workspace in under 10 minutes.

## Scope

- **In scope**: a multi-language calculator demo with Rust features under `crates/`, Python features under `modules/`, and TypeScript features under `packages/`, all conforming to the AES layer model.
- **Out of scope**: rendering, publishing, or hosting the calculator; real-time collaboration; persistent storage of calculation history.

## Feature Requirements

### P0 — Must Have

- Addition, subtraction, multiplication, and division operations each live in their own feature folder with an orchestrator and capabilities pair.
- Every feature folder's contract and taxonomy modules are mirrored in its member's `shared/src/<feature>/` kernel.
- The six required root documents exist at the workspace root: `PRD.md`, `ROADMAP.md`, `README.md`, `AGENTS.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`.

### P1 — Should Have

- Each feature folder carries its `FRD.md` + `BACKLOG.md` doc pair.

### P2 — Nice to Have

- Benchmark suites for each operation under `benches/`.

## Non-functional Requirements (High-level)

| Category    | Commitment                  | Detail lives in          |
|-------------|-----------------------------|--------------------------|
| Performance | Expression evaluation under 10 ms | `FRD.md` of calculator feature |
| Security    | No secrets or credentials in docs | `SECURITY.md`            |
| Scalability | Four operation features, no cross-feature imports | `ARCHITECTURE.md`        |

## Open Questions / Risks

| # | Question / Risk              | Owner | Deadline | Status |
|---|------------------------------|-------|----------|--------|
| 1 | None                         | —     | —        | open   |
