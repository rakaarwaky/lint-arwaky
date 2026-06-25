Upgrade BypassChecker — 2 Changes

## 1. Split Safe vs Unsafe Unwrap Detection

Current: `.unwrap()`, `.unwrap_or()`, `.unwrap_or_default()`, `.expect()` — semua flag sebagai UnwrapExpect

Baru:
| Method | Panic? | AES304 |
|---|---|---|
| `.unwrap()` | Ya | DILARANG |
| `.expect("msg")` | Ya | DILARANG |
| `.unwrap_or_default()` | Tidak (safe) | BOLEH |
| `.unwrap_or(val)` | Tidak (safe) | BOLEH |
| `.unwrap_or_else(fn)` | Tidak (safe) | BOLEH |

Implementation: di `check_bypass_comments`, setelah match token "unwrap",
cek apakah line mengandung `.unwrap_or` → jika ya, skip (safe).

## 2. Deteksi `allow` di Cargo.toml

Method baru `check_cargo_toml` di `IBypassCheckerProtocol`:

```
fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>);
```

Scan `[workspace.lints.clippy]` section, flag semua `= "allow"` sebagai bypass AES304.

Dipanggil dari `CodeAnalysisOrchestrator.run_lint_at()` setelah scan file source.
