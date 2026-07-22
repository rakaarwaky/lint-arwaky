---
name: create-test-rust
description: "Generates contract, unit, and integration test suites for Rust crates. Use when adding a new capability crate, increasing coverage, or preparing a release. Triggers: create tests rust, add tests rust, create test suite rust, crate tests rust."
metadata:
  tags: [rust, testing, contract, unit, integration]
  related: [create-test-typescript, create-test-python]
---

# Create Rust Crate Test Suite

## Rules

- Contract tests verify trait implementation (all required methods exist and compile)
- Unit tests cover happy path, edge cases, and error paths
- Integration tests use the real DI container — no mocks for wiring
- Test IDs match the requirements section they validate

## Coverage Targets

| Layer        | Minimum |
| ------------ | ------- |
| Capabilities | 90%     |
| Agent        | 80%     |
| Utility      | 70%     |

## Workflow

```

Task Progress:

- [ ] Step 1: Analyze crate structure
- [ ] Step 2: Identify untested public API
- [ ] Step 3: Write contract tests
- [ ] Step 4: Write unit tests (happy → edge → error)
- [ ] Step 5: Write integration tests
- [ ] Step 6: Run suite, fix failures, repeat until green
- [ ] Step 7: Verify coverage meets target

```

### Step 1: Analyze Crate

```bash
ls crates/<name>/src/
cargo test -p <name> 2>&1 | head -20   # existing tests?
```

Identify layer: capabilities / infrastructure / agent. Note public traits and structs.

### Step 2: Identify Gaps

For each public method, answer: **"Does this function have a test?"**
No → add to test plan.

### Step 3: Contract Tests

Verify trait implementation compiles and all required methods are callable:

```rust
#[cfg(test)]
mod contract {
    use super::*;

    #[test]
    fn implements_required_trait() {
        fn assert_trait<T: MyTrait>() {}
        assert_trait::<MyCapability>();
    }
}
```

### Step 4: Unit Tests

Place in `#[cfg(test)] mod tests` inside `lib.rs` or a dedicated `tests/` submodule.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_method_happy_path() {
        // Arrange
        let sut = MyCapability::new();
        // Act
        let result = sut.public_method(valid_input());
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn public_method_empty_input() {
        let sut = MyCapability::new();
        let result = sut.public_method("");
        assert!(result.is_err());
    }

    #[test]
    fn public_method_error_propagation() {
        let sut = MyCapability::new();
        let result = sut.public_method(poisoned_input());
        assert_eq!(result.unwrap_err(), MyError::InvalidInput);
    }
}
```

### Step 5: Integration Tests

Place in `crates/<name>/tests/integration_test.rs`. Use the real DI container.

```rust
use my_crate::*;

#[test]
fn di_container_wires_all_dependencies() {
    let container = build_container();
    let cap = container.resolve::<MyCapability>();
    assert!(cap.is_some());
}

#[test]
fn cross_capability_interaction() {
    let container = build_container();
    let a = container.resolve::<CapA>().unwrap();
    let b = container.resolve::<CapB>().unwrap();
    let result = a.produce().and_then(|v| b.consume(v));
    assert!(result.is_ok());
}
```

### Step 6: Run and Fix

```bash
cargo test -p <name> -- --nocapture
# Fix failures → re-run → repeat until green
```

### Step 7: Verify Coverage

```bash
cargo tarpaulin -p <name> --fail-under <target>
```

Target from the Coverage Targets table above. If below threshold, return to Step 2.

## Quick Reference

```bash
cargo test -p <name>                    # all tests
cargo test -p <name> test_name          # single test
cargo test -p <name> -- --nocapture     # with stdout
cargo tarpaulin -p <name>              # coverage report
```

```




