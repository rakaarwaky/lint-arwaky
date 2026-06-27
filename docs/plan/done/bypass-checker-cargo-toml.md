Upgrade BypassChecker — 2 Changes

## 1. Split Safe vs Unsafe Unwrap Detection

Current: `.unwrap()`, `.unwrap_or()`, `.unwrap_or_default()`, `.expect()` — all flagged as UnwrapExpect

New:

| Method                 | Panic?    | AES304    |
| ---------------------- | --------- | --------- |
| `.unwrap()`            | Yes       | FORBIDDEN |
| `.expect("msg")`       | Yes       | FORBIDDEN |
| `.unwrap_or_default()` | No (safe) | ALLOWED   |
| `.unwrap_or(val)`      | No (safe) | ALLOWED   |
| `.unwrap_or_else(fn)`  | No (safe) | ALLOWED   |

Implementation: in `check_bypass_comments`, after matching the "unwrap" token,
check whether the line contains `.unwrap_or` → if yes, skip (safe).

## 2. Detect `allow` in Cargo.toml

New method `check_cargo_toml` in `IBypassCheckerProtocol`:

```
fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>);
```

Scan the `[workspace.lints.clippy]` section, flag all `= "allow"` as AES304 bypass.

Called from `CodeAnalysisOrchestrator.run_lint_at()` after scanning the source file.
