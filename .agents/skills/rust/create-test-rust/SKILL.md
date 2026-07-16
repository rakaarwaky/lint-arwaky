---
name: create-crate-test-suite-rust
version: 1.0.0
category: creation
tags: [aes, test, rust, contract, unit, integration]
triggers:
  - "create tests rust"
  - "add tests rust"
  - "create test suite rust"
  - "crate tests rust"
dependencies: []
related:
  - module_logic_validator
---

# create-crate-test-suite-rust

## Rules

- Contract tests MUST verify trait implementation
- Unit tests MUST cover happy path, edge cases, errors
- Integration tests MUST use real DI container
- Test IDs MUST match requirements section

## Coverage Targets

| Layer          | Minimum |
| -------------- | ------- |
| Capabilities   | 90%     |
| Infrastructure | 70%     |
| Agent          | 80%     |

## Purpose

Generate contract tests, unit tests, and integration tests for crate capabilities.

## When to Use

- Adding new capability crate
- Increasing test coverage
- Before major releases

## The Fundamental Question

> **"Does this capability have tests?"**

If no -> **Create test suite**

## Workflow

### Step 1: Analyze Crate

- List files in `crates/<name>/src/`
- Identify layers (capabilities, infrastructure, agent)
- Check existing tests

### Step 2: Create Test Module

Add `#[cfg(test)]` module in lib.rs or create separate test files.

### Step 3: Contract Tests

- Create tests that verify trait implementation
- Test all required methods exist

### Step 4: Unit Tests

- Test happy path, edge cases, errors
- Target: 90% coverage for capabilities

### Step 5: Integration Tests

- Test cross-capability interaction
- Test DI container wiring

## Test Structure

```rust
// crates/<name>/src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_method() {
        // Arrange
        let sut = MyCapability::new();

        // Act
        let result = sut.public_method();

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_public_method_error_case() {
        // Test error handling
    }
}
```

## Integration Test Structure

```rust
// crates/<name>/tests/integration_test.rs
use my_crate::*;

#[test]
fn test_di_container_wiring() {
    // Test that DI container wires all dependencies correctly
}
```

## Quick Commands

```bash
# Run all tests for a crate
cargo test -p <crate-name>

# Run tests with output
cargo test -p <crate-name> -- --nocapture

# Run specific test
cargo test -p <crate-name> test_name

# Check coverage (requires cargo-tarpaulin)
cargo tarpaulin -p <crate-name>
```
