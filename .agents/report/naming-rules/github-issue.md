
---
name: 🏗️ Naming Rules — Reliability, Correctness, and SOLID Fix
about: Track fixes for surface-layer mapping, trait segregation, error propagation, and naming-rule reliability.
title: "[naming-rules] Fix surface mapping, ISP violation, error handling, and AES101/AES102 reliability"
labels: ["bug", "tech-debt", "architecture", "naming-rules", "priority: high"]
assignees: ""
---
## 🔍 Description

The `naming-rules` crate enforces AES101 and AES102 naming conventions.

The current implementation has several correctness and maintainability issues that can cause false negatives, false positives, and silent failures.

The most important confirmed issue is:

> The canonical layer for the `surface_` prefix must be `surface`, not `surfaces`.

---

## 🚨 Confirmed Issues

### 1. Incorrect `surface_` Layer Mapping

Current behavior:

```text
surface_login_controller.rs -> surface
```


Expected behavior:

```text
surface_login_controller.rs -> surface
```

Impact:

- layer lookup can fail
- suffix validation can be skipped
- false negatives can occur for surface-layer files

---

### 2. Interface Segregation Violation

Current trait:

```rust
INamingCheckerProtocol
```

forces both checkers to implement unused no-op methods.

Expected:

```rust
INamingConventionChecker
ISuffixPrefixChecker
```

---

### 3. Audit API Cannot Report Failures

Current:

```rust
async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
```

Expected:

```rust
async fn run_audit(
    &self,
    target: &FilePath,
) -> Result<Vec<LintResult>, ScanError>;
```

---

### 4. Duplicated Checker Logic

Both checkers duplicate:

```rust
_detect_layer()
_make_result()
```

Expected:

Centralize helpers in:

```text
crates/shared/src/naming-rules/utility_naming_checker.rs
```

---

### 5. AES101 Minimum Word Count Is Inconsistent

FRD requires at least 3 words.

Current default config uses:

```rust
Count::new(2)
```

Expected default:

```rust
Count::new(3)
```

Checker should also use config-driven `word_count`.

---

### 6. Violation Messages Mention `infrastructure_`

FRD states:

> `utility_` covers the former `infrastructure_` concerns.

Messages should reference `utility_`, not `infrastructure_`.

---

## ✅ Acceptance Criteria

- [ ] `surface_` maps to `surface`, not `surfaces`.
- [ ] Layer detection uses a single source of truth.
- [ ] `INamingCheckerProtocol` is split into `INamingConventionChecker` and `ISuffixPrefixChecker`.
- [ ] No checker implements a no-op method from an unrelated rule.
- [ ] `run_audit` returns `Result<Vec<LintResult>, ScanError>`.
- [ ] Missing target path returns an error instead of silently producing zero violations.
- [ ] Shared helpers are used for layer detection and lint-result construction.
- [ ] AES101 minimum word count is read from configuration.
- [ ] Default AES101 minimum word count is `3`.
- [ ] Violation messages reference `utility_`, not `infrastructure_`.
- [ ] Existing scan still reports zero violations for compliant crates.
- [ ] Unit tests cover `surface_`, `taxonomy_`, `capabilities_`, and `root_` cases.

---

## 📎 Follow-Up Work

These should be tracked separately:

- [ ] Add secure filesystem walker.
- [ ] Add symlink protection.
- [ ] Add workspace path confinement.
- [ ] Add max-depth scanning limit.
- [ ] Add `spawn_blocking` for filesystem traversal.
- [ ] Add streaming file iteration.
- [ ] Make `parse_config_yaml` return `Result`.
- [ ] Support glob-based exception matching.
- [ ] Add explicit `.test.ts` / `.spec.ts` exemption policy.
- [ ] Add persistence layer for lint results if needed.

---

## 🧪 Validation

Run:

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

Manual scan:

```bash
cargo run -- scan crates/naming-rules
```

Expected:

```text
Violations: 0
```

```

---

# 3. Git Diff Patch

Save this as:

```text
patches/0001-fix-naming-rules-surface-isp-error-handling.patch
```
