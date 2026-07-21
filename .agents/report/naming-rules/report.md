
---

# 1. `report.md`

Save this as:

```text
docs/reports/naming-rules-backend-review.md
```

```md
# Backend Review Report — `naming-rules` v1.10.106

Date: 2026-07-21  
Reviewer: Expert Backend Review  
Crate: `naming_rules-lint-arwaky`  
Version: `1.10.106`  
Scope: Rust implementation of AES101 and AES102 naming-rule enforcement.

---

## 1. Executive Summary

The `naming-rules` crate has a sound architectural intention and follows a clear layered structure:

- `root_` wiring
- `agent_` orchestration
- `capabilities_` rule execution
- `contract_` protocols
- `taxonomy_` value objects
- `utility_` helpers

However, the current implementation contains several backend-quality issues related to:

- correctness
- reliability
- maintainability
- SOLID compliance
- filesystem scalability
- configuration safety

The most important confirmed correctness issue is:

> The canonical layer name for the `surface_` prefix must be `surface`, not `surfaces`.

The current code maps:

```rust
surface_ -> surfaces
```

which can cause layer lookup failures and false negatives when the layer map uses `surface`.

---

## 2. Confirmed Canonical Layer Mapping

The valid layer mapping must be:

| File Prefix       | Canonical Layer Name |
| ----------------- | -------------------- |
| `taxonomy_`     | `taxonomy`         |
| `contract_`     | `contract`         |
| `utility_`      | `utility`          |
| `capabilities_` | `capabilities`     |
| `agent_`        | `agent`            |
| `surface_`      | `surface`          |
| `root_`         | `root`             |

Therefore:

```text
surface_login_controller.rs -> surface
```

not:

```text
surface_login_controller.rs -> surfaces
```

---

## 3. Findings

| Severity | Area            | Issue                                                                     | Impact                                                            |
| -------- | --------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| High     | Correctness     | `surface_` maps to `surfaces` instead of `surface`                  | Layer lookup can fail, causing suffix rules to be skipped         |
| High     | Reliability     | `run_audit` returns `Vec<LintResult>` instead of `Result`           | Scan failures cannot be reported                                  |
| High     | Maintainability | `INamingCheckerProtocol` forces no-op methods                           | Violates Interface Segregation Principle                          |
| High     | Maintainability | Duplicate`_make_result()` and `_detect_layer()` logic                 | DRY violation and inconsistent lint results                       |
| Medium   | Correctness     | AES101 minimum word count is hardcoded while config exposes`word_count` | Config and behavior drift                                         |
| Medium   | Correctness     | Default config uses`word_count: 2`, while FRD requires at least 3 words | Incorrect default behavior                                        |
| Medium   | Performance     | Filesystem walk collects all files into memory                            | Poor scalability for large repositories                           |
| Medium   | Performance     | Blocking filesystem walk inside async function                            | Can block async runtime                                           |
| Medium   | Security        | No explicit path traversal, symlink, or max-depth protection              | Potential DoS or workspace escape                                 |
| Medium   | Reliability     | Config parsing silently falls back to defaults                            | Invalid config can silently disable rules                         |
| Low      | Correctness     | Exceptions are matched by basename only                                   | Over-broad exemptions                                             |
| Low      | Correctness     | Multi-dot filenames such as`foo.test.ts` may produce false positives    | TS/JS test files may be incorrectly flagged                       |
| Low      | Maintainability | Violation messages still mention`infrastructure_`                       | FRD says`utility_` replaces former `infrastructure_` concerns |

---

## 4. Detailed Review

### 4.1 High: `surface_` Layer Mapping Is Incorrect

Current code in:

```text
crates/shared/src/common/utility_layer_detector.rs
```

contains:

```rust
const PREFIX_MAP: &[(&str, &str)] = &[
    ("taxonomy_", "taxonomy"),
    ("contract_", "contract"),
    ("capabilities_", "capabilities"),
    ("utility_", "utility"),
    ("agent_", "agent"),
    ("surface_", "surfaces"),
    ("root_", "root"),
];
```

This is incorrect.

The valid mapping is:

```rust
surface_ -> surface
```

Recommended fix: derive layer names from the canonical `LAYER_PREFIXES` constant so there is a single source of truth.

---

### 4.2 High: Audit API Cannot Report Failures

Current contract:

```rust
async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
```

This cannot distinguish between:

- no violations found
- target path missing
- permission denied
- invalid configuration
- filesystem failure

Recommended contract:

```rust
async fn run_audit(
    &self,
    target: &FilePath,
) -> Result<Vec<LintResult>, ScanError>;
```

---

### 4.3 High: Interface Segregation Violation

Current protocol:

```rust
pub trait INamingCheckerProtocol: Send + Sync {
    async fn check_file_naming(...);
    async fn check_domain_suffixes(...);
}
```

But:

- `NamingConventionChecker` implements `check_domain_suffixes` as no-op
- `SuffixPrefixChecker` implements `check_file_naming` as no-op

This violates ISP.

Recommended split:

```rust
pub trait INamingConventionChecker: Send + Sync {
    async fn check_file_naming(...);
}

pub trait ISuffixPrefixChecker: Send + Sync {
    async fn check_domain_suffixes(...);
}
```

---

### 4.4 High: Duplicated Checker Helpers

Both checkers duplicate:

```rust
fn _detect_layer(...)
fn _make_result(...)
```

This violates DRY and increases the chance of inconsistent lint output.

Recommended fix:

Create shared helpers:

```text
crates/shared/src/naming-rules/utility_naming_checker.rs
```

with:

```rust
pub fn detect_layer(...)
pub fn layer_keys(...)
pub fn is_exception(...)
pub fn file_level_result(...)
```

---

### 4.5 Medium: AES101 Minimum Word Count Is Not Config-Driven

FRD requires at least 3 words.

Current default config uses:

```rust
NamingConfig::new(Count::new(2))
```

Current checker hardcodes:

```rust
min_words: 3
```

Recommended fix:

- default config should use `3`
- checker should read from config

---

### 4.6 Medium: Filesystem Walk Is Blocking and Memory-Heavy

Current code:

```rust
let all_files = walk_recursive(target, Some(&self.ignored_patterns));
```

This is synchronous and materializes all files.

Recommended future improvements:

- use `spawn_blocking` if running inside Tokio
- introduce streaming directory walker
- add symlink protection
- add max-depth protection
- add workspace confinement
- ignore `.git`, `node_modules`, `target`, `dist`, `build`, `.venv`

These are not fully included in the patch below because they may require new workspace dependencies such as `tokio`, `walkdir`, and `globset`.

---

### 4.7 Medium: Configuration Parsing Silently Falls Back to Defaults

Current code:

```rust
let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(yaml_str).unwrap_or_default();
```

and later:

```rust
Err(e) => {
    eprintln!("[warn] Failed to deserialize ArchitectureConfig: {:?}", e);
    ArchitectureConfig::default()
}
```

This is dangerous for a compliance tool.

Recommended future fix:

```rust
pub fn parse_config_yaml(yaml_str: &str) -> Result<ArchitectureConfig, ConfigParseError>
```

Invalid user configuration should fail loudly, not silently default.

---

## 5. Database Review

No database queries are present in the uploaded crate.

If lint results are persisted later, use:

- parameterized queries
- connection pooling
- transactional bulk inserts
- indexes on `run_id`, `file`, `code`, and `severity`
- statement timeouts
- migration-managed schema

---

## 6. SOLID Assessment

### Single Responsibility Principle

`NamingConventionChecker` currently handles:

- AES101 stem validation
- AES102 unknown-prefix detection

Long term, consider separating unknown-prefix detection into its own rule checker.

### Open/Closed Principle

Suffix policy handling is string-based:

```rust
if def.naming.suffix_policy.value == SUFFIX_POLICY_STRICT
```

Long term, model suffix policy as a typed enum or strategy object.

### Liskov Substitution Principle

No-op trait methods are a Liskov smell.

Splitting the trait resolves this.

### Interface Segregation Principle

Fixed by splitting:

```rust
INamingConventionChecker
ISuffixPrefixChecker
```

### Dependency Inversion Principle

The orchestrator already depends on abstractions.

This patch improves DIP by centralizing helper behavior and making checker contracts more precise.

---

## 7. Patch Scope

The provided patch includes:

- fix `surface_ -> surface`
- split `INamingCheckerProtocol`
- make `run_audit` return `Result`
- centralize lint-result creation
- centralize layer detection helpers
- use config-driven AES101 minimum word count
- change default `word_count` to `3`
- replace `infrastructure_` guidance with `utility_`
- add shared `is_valid_aes_stem` validator

The patch intentionally does **not** yet introduce:

- Tokio `spawn_blocking`
- `walkdir`
- `globset`
- streaming file walker
- full config parser rewrite

Those should be handled in follow-up PRs because they affect dependencies and broader runtime behavior.

---

## 8. Validation Plan

Run:

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

Manual validation:

```bash
cargo run -- scan crates/naming-rules
```

Expected:

```text
Violations: 0
```

Additional test cases:

```text
surface_login_controller.rs       -> surface layer, pass if suffix allowed
surface_login_helper.rs           -> surface layer, fail if helper not allowed
taxonomy_user_vo.rs               -> pass
taxonomy_user_checker.rs          -> fail if checker forbidden in taxonomy
capabilities_user_checker.rs      -> pass
capabilities_user_vo.rs           -> fail if vo forbidden in capabilities
root_naming_rules_container.rs    -> pass
root_naming_rules_service.rs      -> fail if service not allowed in root
db.rs                             -> AES101 fail
user_checker.rs                   -> AES101 fail if min_words = 3
```

---

## 9. Final Recommendation

Merge the provided patch as a correctness and maintainability fix.

Then open follow-up issues for:

1. secure filesystem walker
2. async-safe blocking traversal
3. streaming lint results
4. fallible configuration parsing
5. glob-based exception matching
6. test/spec file exemption policy

```

---

# 2. GitHub Issue Template

Save this as:

```text
.github/ISSUE_TEMPLATE/naming-rules-reliability-fix.md
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
