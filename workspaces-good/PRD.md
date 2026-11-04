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

### PR-CALC-002: Subtraction operation
<!-- Tier P1: the first P1 entry. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | subtract one number from another | get the difference | P1 |

#### Proposed Solution

Subtracts one number from another and returns the result.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the subtraction capability.
3. The user receives the difference.

### PR-CALC-003: Multiplication operation
<!-- Tier P2: the first P2 entry. -->

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

### PR-CALC-004: Division operation
<!-- Tier P1: the first P1 entry. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | divide one number by another | get the quotient | P1 |

#### Proposed Solution

Divides one number by another and returns the result. Division by zero returns an error.

#### Main User Flow

1. The user supplies two numbers.
2. The calculator applies the division capability.
3. The user receives the quotient.

### PR-CALC-005: Expression parsing
<!-- Tier P2: the first P2 entry. -->

#### User Stories

| As a | I want | So that | Priority |
|------|--------|---------|----------|
| user | write an arithmetic expression | get its value | P2 |

#### Proposed Solution

Parses an arithmetic expression of addition, subtraction, multiplication, and division and evaluates it.

#### Main User Flow

1. The user types an expression.
2. The calculator parses and evaluates it.
3. The user receives the value.


## Reference

| Artifact     | Purpose                                          | Link |
|--------------|--------------------------------------------------|------|
| PRD          | Business goals and scope                         | [PRD.md](PRD.md) |
| Backlog      | What is true now, with evidence                  | [BACKLOG.md](BACKLOG.md) |
| Roadmap      | Timeline, phasing, and dependencies              | [ROADMAP.md](ROADMAP.md) |
| Architecture | System boundaries, tech stack, and infra         | [ARCHITECTURE.md](ARCHITECTURE.md) |
