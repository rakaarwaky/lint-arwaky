# PRD — Calculator

## Project Overview

A small multi-language calculator demo needs a single, clear specification of
what it does and why, so that stakeholders, PMs, designers, and engineers
agree on scope without reading source code.

## Goals

- One calculator contract shared across Rust, Python, and TypeScript.
- A clean, gated multi-language demo a user can build and run locally.
- Reproducible architecture gates that fail loudly on boundary violations.

## Non-Goals

- Persistent storage of calculation history.
- A general-purpose expression evaluator with variables and functions.
- Web, mobile, or hosted surfaces beyond the local demo.

## Target Users

| Persona | Need | Current Pain | Desired Outcome |
|---------|------|--------------|-----------------|
| Engineer | Share one calculator contract across three languages | Three hand-rolled variants drift apart | One gated, versioned contract |
| Maintainer | Keep the architecture boundary enforced | Manual review misses layer crossings | Automated gates catch every violation |

## Scope

### In Scope

- Addition, subtraction, multiplication, and division operations.
- Expression parsing and evaluation.
- The shared layer contract and per-feature vertical slices.

### Out of Scope

- Persistent storage of calculation history.
- Variables, functions, and persistent state in the evaluator.
- Hosted or networked execution.

---

## Project Requirements

### PR-CALC-001: Addition operation
<!-- Tier P0: entry -001 is always P0. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | add two numbers | get their sum | P0 |

#### Proposed Solution

Adds two numbers and returns the result. Integer and decimal addition live in the calculator feature; persistent storage of calculation history is out of scope.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the addition capability.
3. The user receives the sum.

### PR-CALC-002: Addition operation
<!-- Tier P1: the first P1 entry. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | add two numbers | get their sum | P1 |

#### Proposed Solution

Adds two numbers and returns the result (second addition entry in the folder order).

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the addition capability.
3. The user receives the sum.

### PR-CALC-003: Calculator operation
<!-- Tier P2: the first P2 entry. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | use the calculator | get calculation results | P2 |

#### Proposed Solution

Provides the core calculator capability across operations.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the calculator capability.
3. The user receives the result.

### PR-CALC-004: Calculator operation
<!-- Tier P1: the first P1 entry. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | use the calculator | get calculation results | P1 |

#### Proposed Solution

Provides the core calculator capability across operations.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the calculator capability.
3. The user receives the result.

### PR-CALC-005: Calculator operation
<!-- Tier P2: the first P2 entry. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | use the calculator | get calculation results | P2 |

#### Proposed Solution

Provides the core calculator capability across operations.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the calculator capability.
3. The user receives the result.

### PR-CALC-006: CLI operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | use the CLI | run commands from the terminal | P2 |

#### Proposed Solution

Provides the command-line interface entry point for the calculator workspace.

#### Main User Flow

1. The user invokes a CLI subcommand.
2. The CLI parses arguments and delegates to the dispatcher.
3. The result is printed to the terminal.

### PR-CALC-007: CLI operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | use the CLI | run commands from the terminal | P2 |

#### Proposed Solution

Provides the command-line interface entry point for the calculator workspace.

#### Main User Flow

1. The user invokes a CLI subcommand.
2. The CLI parses arguments and delegates to the dispatcher.
3. The result is printed to the terminal.

### PR-CALC-008: CLI operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | use the CLI | run commands from the terminal | P2 |

#### Proposed Solution

Provides the command-line interface entry point for the calculator workspace.

#### Main User Flow

1. The user invokes a CLI subcommand.
2. The CLI parses arguments and delegates to the dispatcher.
3. The result is printed to the terminal.

### PR-CALC-009: Division operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | divide one number by another | get the quotient | P2 |

#### Proposed Solution

Divides one number by another and returns the result. Division by zero returns an error.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the division capability.
3. The user receives the quotient.

### PR-CALC-010: Division operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | divide one number by another | get the quotient | P2 |

#### Proposed Solution

Divides one number by another and returns the result. Division by zero returns an error.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the division capability.
3. The user receives the quotient.

### PR-CALC-011: Multiplication operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | multiply two numbers | get the product | P2 |

#### Proposed Solution

Multiplies two numbers and returns the result.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the multiplication capability.
3. The user receives the product.

### PR-CALC-012: Multiplication operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | multiply two numbers | get the product | P2 |

#### Proposed Solution

Multiplies two numbers and returns the result.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the multiplication capability.
3. The user receives the product.

### PR-CALC-013: Shared operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | use shared contracts | keep all features on one data model | P2 |

#### Proposed Solution

Hosts shared contracts, value objects, and parsers used by every feature.

#### Main User Flow

1. A rule imports a shared contract VO.
2. The VO carries parsed file data, dependency edges, and language flags.
3. All rules read the same shape of data, preventing drift.

### PR-CALC-014: Shared operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | use shared contracts | keep all features on one data model | P2 |

#### Proposed Solution

Hosts shared contracts, value objects, and parsers used by every feature.

#### Main User Flow

1. A rule imports a shared contract VO.
2. The VO carries parsed file data, dependency edges, and language flags.
3. All rules read the same shape of data, preventing drift.

### PR-CALC-015: Shared operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | use shared contracts | keep all features on one data model | P2 |

#### Proposed Solution

Hosts shared contracts, value objects, and parsers used by every feature.

#### Main User Flow

1. A rule imports a shared contract VO.
2. The VO carries parsed file data, dependency edges, and language flags.
3. All rules read the same shape of data, preventing drift.

### PR-CALC-016: Single Orchestrator operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | run a single orchestrator | coordinate work across features | P2 |

#### Proposed Solution

The single orchestrator coordinates execution across calculator features and reports results.

#### Main User Flow

1. The orchestrator is invoked with a request.
2. It delegates to the appropriate feature capability.
3. Results are aggregated and returned.

### PR-CALC-017: Subtraction operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | subtract one number from another | get the difference | P2 |

#### Proposed Solution

Subtracts one number from another and returns the result.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the subtraction capability.
3. The user receives the difference.

### PR-CALC-018: Subtraction operation

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | subtract one number from another | get the difference | P2 |

#### Proposed Solution

Subtracts one number from another and returns the result.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the subtraction capability.
3. The user receives the difference.

## Reference

| Artifact     | Purpose                                          | Link |
|--------------|--------------------------------------------------|------|
| PRD          | Business goals and scope                         | [PRD.md](PRD.md) |
| Backlog      | What is true now, with evidence                  | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies              | [ROADMAP.md](ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra         | [ARCHITECTURE.md](ARCHITECTURE.md) |
