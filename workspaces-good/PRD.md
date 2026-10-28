# PRD — Calculator Workspace

## Problem Statement

A small multi-language calculator demo needs a single, clear specification of what it does and why, so that stakeholders, PMs, designers, and engineers agree on scope without reading source code.

## Project Requirements

### PR-CALC-001: Addition operation

| Aspect         | Detail                                                      |
|----------------|-------------------------------------------------------------|
| Description    | Adds two numbers and returns the result                     |
| User Story     | As a user, I want to add two numbers, so that I get their sum |
| In Scope       | Integer and decimal addition in the calculator feature      |
| Out of Scope   | Persistent storage of calculation history                   |
| Acceptance     | `lint-arwaky-cli check .` exits 0                           |
| Success Metric | Expression evaluation under 10 ms                           |

### PR-CALC-002: Subtraction operation

| Aspect         | Detail                                                      |
|----------------|-------------------------------------------------------------|
| Description    | Subtracts one number from another and returns the result    |
| User Story     | As a user, I want to subtract two numbers, so that I get their difference |
| In Scope       | Integer and decimal subtraction in the calculator feature   |
| Out of Scope   | Persistent storage of calculation history                   |
| Acceptance     | `lint-arwaky-cli check .` exits 0                           |
| Success Metric | Expression evaluation under 10 ms                           |

### PR-CALC-003: Multiplication operation

| Aspect         | Detail                                                      |
|----------------|-------------------------------------------------------------|
| Description    | Multiplies two numbers and returns the result               |
| User Story     | As a user, I want to multiply two numbers, so that I get their product |
| In Scope       | Integer and decimal multiplication in the calculator feature |
| Out of Scope   | Persistent storage of calculation history                   |
| Acceptance     | `lint-arwaky-cli check .` exits 0                           |
| Success Metric | Expression evaluation under 10 ms                           |

### PR-CALC-004: Division operation

| Aspect         | Detail                                                      |
|----------------|-------------------------------------------------------------|
| Description    | Divides one number by another and returns the result        |
| User Story     | As a user, I want to divide two numbers, so that I get their quotient |
| In Scope       | Integer and decimal division in the calculator feature      |
| Out of Scope   | Persistent storage of calculation history                   |
| Acceptance     | `lint-arwaky-cli check .` exits 0                           |
| Success Metric | Expression evaluation under 10 ms                           |

### PR-CALC-005: Expression parsing

| Aspect         | Detail                                                      |
|----------------|-------------------------------------------------------------|
| Description    | Parses an arithmetic expression and evaluates it            |
| User Story     | As a user, I want to write an expression, so that I get its value |
| In Scope       | Addition, subtraction, multiplication, division expressions |
| Out of Scope   | Variables, functions, persistent storage                    |
| Acceptance     | `lint-arwaky-cli check .` exits 0                           |
| Success Metric | Expression evaluation under 10 ms                           |

## Reference

| Artifact     | Purpose                                          | Link|
|--------------|--------------------------------------------------|------|
| Roadmap      | Timeline, phasing, and dependencies              | [ROADMAP.md](ROADMAP.md)|
| Architecture | System boundaries, tech stack, and infra         | [ARCHITECTURE.md](ARCHITECTURE.md)|

