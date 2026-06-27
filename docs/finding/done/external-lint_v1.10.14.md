# Crate: external-lint (v1.10.14)

This document contains the source code for feature crate `external-lint` along with its corresponding and imported definitions from the `shared` crate.

## Problem Statement

The following issues were detected by `lint-arwaky-cli scan`:

```
============================================================
  AES Architecture Compliance Report
============================================================
  Project: /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint
  Violations: 35
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_eslint_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_eslint_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_eslint_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_eslint_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_eslint_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_prettier_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_prettier_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_prettier_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_tsc_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_tsc_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_tsc_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_tsc_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_tsc_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_py_bandit_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_py_bandit_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_py_ruff_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_audit_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_audit_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_audit_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_fmt_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_fmt_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_fmt_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_fmt_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
  [AES304] /home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_fmt_adapter.rs - AES304 UNWRAP_EXPECT: Forbidden unwrap or expect call detected.
WHY? Using unwrap or expect results in runtime panics and bypasses proper error propagation.
FIX: Replace the unwrap/expect call with structured error handling (Option/Result pattern matching or '?').
```

---

## File List

- [crates/external-lint/Cargo.toml](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/Cargo.toml)
- [crates/external-lint/src/agent_external_lint_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/agent_external_lint_orchestrator.rs)
- [crates/external-lint/src/infrastructure_js_eslint_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_eslint_adapter.rs)
- [crates/external-lint/src/infrastructure_js_prettier_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_prettier_adapter.rs)
- [crates/external-lint/src/infrastructure_js_tsc_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_js_tsc_adapter.rs)
- [crates/external-lint/src/infrastructure_py_bandit_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_py_bandit_adapter.rs)
- [crates/external-lint/src/infrastructure_py_mypy_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_py_mypy_adapter.rs)
- [crates/external-lint/src/infrastructure_py_ruff_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_py_ruff_adapter.rs)
- [crates/external-lint/src/infrastructure_rs_audit_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_audit_adapter.rs)
- [crates/external-lint/src/infrastructure_rs_clippy_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_clippy_adapter.rs)
- [crates/external-lint/src/infrastructure_rs_fmt_adapter.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_rs_fmt_adapter.rs)
- [crates/external-lint/src/infrastructure_stdio_client.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/infrastructure_stdio_client.rs)
- [crates/external-lint/src/lib.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/lib.rs)
- [crates/external-lint/src/root_external_lint_container.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/external-lint/src/root_external_lint_container.rs)
- [crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs)
- [crates/shared/src/cli-commands/contract_executor_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/contract_executor_port.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_severity_vo.rs)
- [crates/shared/src/code-analysis/contract_adapter_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_adapter_port.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/code-analysis/taxonomy_operation_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/taxonomy_operation_error.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_job_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_response_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_response_data_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/external-lint/contract_external_lint_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_aggregate.rs)
- [crates/shared/src/external-lint/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/mod.rs)
- [crates/shared/src/mcp-server/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/mcp-server/mod.rs)
- [crates/shared/src/source-parsing/contract_path_normalization_port.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/contract_path_normalization_port.rs)
- [crates/shared/src/source-parsing/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/mod.rs)
- [crates/shared/src/source-parsing/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_adapter_error.rs)
- [crates/shared/src/source-parsing/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/source-parsing/taxonomy_path_vo.rs)

---

## File: crates/external-lint/Cargo.toml

```toml
[package]
name = "external_lint-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "Adapters for external linters — Rust (clippy, rustfmt, cargo-audit), Python (ruff, mypy, bandit), JavaScript/TypeScript (eslint, prettier, tsc). Invoked via stdio."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]  # (unchanged)
anyhow.workspace = true
serde.workspace = true
serde_json.workspace = true
async-trait.workspace = true
regex.workspace = true
tracing.workspace = true
rustsec.workspace = true
dirs.workspace = true
futures.workspace = true
tokio.workspace = true
shared.workspace = true
```

---

## File: crates/external-lint/src/agent_external_lint_orchestrator.rs

```rust
// PURPOSE: ExternalLintOrchestrator — agent layer, orchestrates external linter adapters
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub struct ExternalLintOrchestrator {
    adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
}

impl ExternalLintOrchestrator {
    pub fn new(adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>) -> Self {
        Self { adapters }
    }
}

#[async_trait]
impl IExternalLintAggregate for ExternalLintOrchestrator {
    async fn scan_all(&self, path: &FilePath) -> LintResultList {
        let mut has_rs = false;
        let mut has_py = false;
        let mut has_js = false;

        fn detect_languages(
            dir: &std::path::Path,
            has_rs: &mut bool,
            has_py: &mut bool,
            has_js: &mut bool,
        ) {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let name = match path.file_name() {
                            Some(n) => n.to_string_lossy(),
                            None => continue,
                        };
                        if !matches!(
                            name.as_ref(),
                            "node_modules" | "target" | ".git" | ".jj" | "Graph-It-Live"
                        ) {
                            detect_languages(&path, has_rs, has_py, has_js);
                        }
                    } else if let Some(ext) = path.extension() {
                        match ext.to_str() {
                            Some("rs") => *has_rs = true,
                            Some("py") => *has_py = true,
                            Some("js" | "ts" | "jsx" | "tsx") => *has_js = true,
                            _ => {}
                        }
                    }
                    if *has_rs && *has_py && *has_js {
                        break;
                    }
                }
            }
        }

        let root_path = std::path::Path::new(&path.value);
        if root_path.is_file() {
            if let Some(ext) = root_path.extension() {
                match ext.to_str() {
                    Some("rs") => has_rs = true,
                    Some("py") => has_py = true,
                    Some("js" | "ts" | "jsx" | "tsx") => has_js = true,
                    _ => {}
                }
            }
        } else {
            detect_languages(root_path, &mut has_rs, &mut has_py, &mut has_js);
        }

        let mut adapter_names = Vec::new();
        if has_rs {
            adapter_names.push("clippy");
            adapter_names.push("rustfmt");
            adapter_names.push("cargo-audit");
        }
        if has_py {
            adapter_names.push("ruff");
            adapter_names.push("mypy");
            adapter_names.push("bandit");
        }
        if has_js {
            adapter_names.push("eslint");
            adapter_names.push("prettier");
            adapter_names.push("tsc");
        }

        let mut futures = Vec::new();
        for name in &adapter_names {
            if let Some(adapter) = self.adapters.get(*name) {
                let adapter: Arc<dyn ILinterAdapterPort> = adapter.clone();
                let path_clone = path.clone();
                let name_owned = name.to_string();
                futures.push(async move {
                    match adapter.scan(&path_clone).await {
                        Ok(results) => Ok::<Vec<_>, String>(results.values),
                        Err(e) => {
                            let err_msg = e.to_string();
                            if err_msg.contains("No such file or directory")
                                || err_msg.contains("os error 2")
                            {
                                eprintln!(
                                    "[warn] {} is not installed or not in system PATH. Skipping.",
                                    name_owned
                                );
                            } else {
                                eprintln!("[warn] {} adapter failed: {}", name_owned, err_msg);
                            }
                            Ok(Vec::new())
                        }
                    }
                });
            }
        }

        let results = future::join_all(futures).await;
        let mut all = Vec::new();
        for values in results.into_iter().flatten() {
            all.extend(values);
        }
        LintResultList::new(all)
    }

    fn adapter_names(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}
```

---

## File: crates/external-lint/src/infrastructure_js_eslint_adapter.rs

```rust
// PURPOSE: ESLintAdapter — ILinterAdapterPort implementation for ESLint integration

use serde_json::Value;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_adapter_error::AdapterError;
use shared::source_parsing::taxonomy_adapter_error::ScanError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

fn is_bun_available() -> bool {
    match std::process::Command::new("bun")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
    {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}

fn resolve_js_cmd(executable: &str, args: Vec<String>, working_dir: &str) -> Vec<String> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);

    if local_bin.exists() {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return cmd;
    }

    let runner = if is_bun_available() { "bunx" } else { "npx" };
    let mut cmd = vec![runner.to_string(), executable.to_string()];
    cmd.extend(args);
    cmd
}

fn resolve_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if let Ok(abs_path) = std::fs::canonicalize(path_str) {
        let mut current = if abs_path.is_file() {
            match abs_path.parent() {
                Some(p) => p.to_path_buf(),
                None => std::path::PathBuf::from("."),
            }
        } else {
            abs_path.clone()
        };

        for _ in 0..10 {
            if current.join("lint_arwaky.config.yaml").is_file()
                || current.join("lint_arwaky.config.python.yaml").is_file()
                || current.join("package.json").is_file()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
    }
    FilePath::new(".".to_string()).unwrap_or_default()
}

pub struct ESLintAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
}

impl ESLintAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
    ) -> Self {
        Self {
            executor,
            path_norm,
        }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for ESLintAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("eslint")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if Path::new(path_str).is_file()
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = match std::fs::canonicalize(path_str) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => path_str.clone(),
        };

        let cmd = resolve_js_cmd(
            "eslint",
            vec![abs_path, "--format".to_string(), "json".to_string()],
            &wd.value,
        );

        let response = match self
            .executor
            .execute_command(
                PatternList::new(cmd),
                wd.clone(),
                Some(shared::taxonomy_duration_vo::Timeout::new(60.0)),
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return Err(LinterOperationError::Scan(ScanError {
                    path: path.clone(),
                    message: ErrorMessage::new(e.to_string()),
                    error_code: None,
                    adapter_name: Some(self.name()),
                    cause: None,
                }));
            }
        };

        let stdout_str = response.stdout.to_string();
        if stdout_str.trim().is_empty() {
            return Ok(LintResultList::default());
        }

        let parsed: Value = match serde_json::from_str(&stdout_str) {
            Ok(v) => v,
            Err(e) => {
                return Err(LinterOperationError::Scan(ScanError {
                    path: path.clone(),
                    message: ErrorMessage::new(format!("Failed to parse JSON: {}", e)),
                    error_code: None,
                    adapter_name: Some(self.name()),
                    cause: None,
                }));
            }
        };

        let mut results = Vec::new();
        if let Some(files) = parsed.as_array() {
            for file_data in files {
                let filename = match file_data["filePath"].as_str() {
                    Some(s) => s.to_string(),
                    None => String::new(),
                };
                let filename_vo = self.path_norm.resolve_infrastructure_path(
                    FilePath::new(filename).unwrap_or_default(),
                    Some(path.clone()),
                );

                if let Some(messages) = file_data["messages"].as_array() {
                    for msg in messages {
                        let line_num = match msg["line"].as_u64() {
                            Some(v) => v as usize,
                            None => 1,
                        };
                        let col_num = match msg["column"].as_u64() {
                            Some(v) => v as usize,
                            None => 0,
                        };
                        let rule_id = match msg["ruleId"].as_str() {
                            Some(s) => s.to_string(),
                            None => "ESLINT".to_string(),
                        };
                        let message_text = match msg["message"].as_str() {
                            Some(s) => s.to_string(),
                            None => String::new(),
                        };
                        let sev_code = msg["severity"].as_u64().unwrap_or(1);

                        let severity = if sev_code == 2 {
                            Severity::HIGH
                        } else {
                            Severity::MEDIUM
                        };

                        results.push(LintResult {
                            file: filename_vo.clone(),
                            line: LineNumber::new(line_num as i64),
                            column: ColumnNumber::new(col_num as i64),
                            code: ErrorCode::raw(rule_id),
                            message: LintMessage::new(message_text),
                            source: Some(self.name()),
                            severity,
                            enclosing_scope: Default::default(),
                            related_locations: Default::default(),
                        });
                    }
                }
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let path_str = &path.value;
        let wd = resolve_working_dir(path);
        let abs_path = match std::fs::canonicalize(path_str) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => path_str.clone(),
        };

        let cmd = resolve_js_cmd("eslint", vec![abs_path, "--fix".to_string()], &wd.value);

        match self
            .executor
            .execute_command(
                PatternList::new(cmd),
                wd,
                Some(shared::taxonomy_duration_vo::Timeout::new(60.0)),
            )
            .await
        {
            Ok(r) => Ok(ComplianceStatus::new(r.returncode == 0)),
            Err(e) => Err(LinterOperationError::Adapter(AdapterError {
                adapter_name: self.name(),
                message: ErrorMessage::new(e.to_string()),
                error_code: None,
                command: None,
                stderr: Some(ErrorMessage::new("")),
                exit_code: None,
            })),
        }
    }
}
```

---

## File: crates/external-lint/src/infrastructure_js_prettier_adapter.rs

```rust
// PURPOSE: PrettierAdapter — ILinterAdapterPort implementation for Prettier integration

use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_adapter_error::AdapterError;
use shared::source_parsing::taxonomy_adapter_error::ScanError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

fn is_bun_available() -> bool {
    match std::process::Command::new("bun")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
    {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}

fn resolve_js_cmd(executable: &str, args: Vec<String>, working_dir: &str) -> Vec<String> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);

    if local_bin.exists() {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return cmd;
    }

    let runner = if is_bun_available() { "bunx" } else { "npx" };
    let mut cmd = vec![runner.to_string(), executable.to_string()];
    cmd.extend(args);
    cmd
}

fn resolve_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if let Ok(abs_path) = std::fs::canonicalize(path_str) {
        let mut current = if abs_path.is_file() {
            match abs_path.parent() {
                Some(p) => p.to_path_buf(),
                None => std::path::PathBuf::from("."),
            }
        } else {
            abs_path.clone()
        };

        for _ in 0..10 {
            if current.join("lint_arwaky.config.yaml").is_file()
                || current.join("lint_arwaky.config.python.yaml").is_file()
                || current.join("package.json").is_file()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
    }
    FilePath::new(".".to_string()).unwrap_or_default()
}

pub struct PrettierAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
}

impl PrettierAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
    ) -> Self {
        Self {
            executor,
            path_norm,
        }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for PrettierAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("prettier")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if Path::new(path_str).is_file()
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
            && !path_str.ends_with(".json")
            && !path_str.ends_with(".css")
            && !path_str.ends_with(".md")
            && !path_str.ends_with(".yml")
            && !path_str.ends_with(".yaml")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = match std::fs::canonicalize(path_str) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => path_str.clone(),
        };

        let cmd = resolve_js_cmd("prettier", vec!["--check".to_string(), abs_path], &wd.value);

        let response = match self
            .executor
            .execute_command(
                PatternList::new(cmd),
                wd.clone(),
                Some(shared::taxonomy_duration_vo::Timeout::new(60.0)),
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return Err(LinterOperationError::Scan(ScanError {
                    path: path.clone(),
                    message: ErrorMessage::new(e.to_string()),
                    error_code: None,
                    adapter_name: Some(self.name()),
                    cause: None,
                }));
            }
        };

        let mut results = Vec::new();
        let combined_output = format!("{}{}", response.stdout, response.stderr);

        if combined_output.contains("[warn]") {
            let filename_vo = self
                .path_norm
                .resolve_infrastructure_path(path.clone(), Some(path.clone()));
            results.push(LintResult {
                file: filename_vo,
                line: LineNumber::new(1),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw("formatting"),
                message: LintMessage::new("Code style issues found. Run Prettier to fix."),
                source: Some(self.name()),
                severity: Severity::LOW,
                enclosing_scope: Default::default(),
                related_locations: Default::default(),
            });
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let path_str = &path.value;
        let wd = resolve_working_dir(path);
        let abs_path = match std::fs::canonicalize(path_str) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => path_str.clone(),
        };

        let cmd = resolve_js_cmd("prettier", vec!["--write".to_string(), abs_path], &wd.value);

        match self
            .executor
            .execute_command(
                PatternList::new(cmd),
                wd,
                Some(shared::taxonomy_duration_vo::Timeout::new(60.0)),
            )
            .await
        {
            Ok(r) => Ok(ComplianceStatus::new(r.returncode == 0)),
            Err(e) => Err(LinterOperationError::Adapter(AdapterError {
                adapter_name: self.name(),
                message: ErrorMessage::new(e.to_string()),
                error_code: None,
                command: None,
                stderr: Some(ErrorMessage::new("")),
                exit_code: None,
            })),
        }
    }
}
```

---

## File: crates/external-lint/src/infrastructure_js_tsc_adapter.rs

```rust
// PURPOSE: TSCAdapter — ILinterAdapterPort implementation for TypeScript compiler integration

use regex::Regex;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_adapter_error::ScanError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

fn is_bun_available() -> bool {
    match std::process::Command::new("bun")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
    {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}

fn resolve_js_cmd(executable: &str, args: Vec<String>, working_dir: &str) -> Vec<String> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);

    if local_bin.exists() {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return cmd;
    }

    let runner = if is_bun_available() { "bunx" } else { "npx" };
    let mut cmd = vec![runner.to_string(), executable.to_string()];
    cmd.extend(args);
    cmd
}

fn resolve_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if let Ok(abs_path) = std::fs::canonicalize(path_str) {
        let mut current = if abs_path.is_file() {
            match abs_path.parent() {
                Some(p) => p.to_path_buf(),
                None => std::path::PathBuf::from("."),
            }
        } else {
            abs_path.clone()
        };

        for _ in 0..10 {
            if current.join("lint_arwaky.config.yaml").is_file()
                || current.join("lint_arwaky.config.python.yaml").is_file()
                || current.join("package.json").is_file()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
    }
    FilePath::new(".".to_string()).unwrap_or_default()
}

pub struct TSCAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
}

impl TSCAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
    ) -> Self {
        Self {
            executor,
            path_norm,
        }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for TSCAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("tsc")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if Path::new(path_str).is_file()
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = match std::fs::canonicalize(path_str) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => path_str.clone(),
        };

        let mut args = vec![
            "--noEmit".to_string(),
            "--pretty".to_string(),
            "false".to_string(),
        ];
        if abs_path != "." && abs_path != "./" {
            args.push(abs_path);
        }

        let cmd = resolve_js_cmd("tsc", args, &wd.value);

        let response = match self
            .executor
            .execute_command(
                PatternList::new(cmd),
                wd.clone(),
                Some(shared::taxonomy_duration_vo::Timeout::new(60.0)),
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return Err(LinterOperationError::Scan(ScanError {
                    path: path.clone(),
                    message: ErrorMessage::new(e.to_string()),
                    error_code: None,
                    adapter_name: Some(self.name()),
                    cause: None,
                }));
            }
        };

        let output = format!("{}{}", response.stdout, response.stderr);
        let mut results = Vec::new();

        let pattern1 = match Regex::new(r"^([^(]+)\((\d+),(\d+)\):\s+error\s+(TS\d+):\s+(.*)$") {
            Ok(r) => r,
            Err(_) => return Ok(LintResultList::new(vec![])),
        };
        let pattern2 = match Regex::new(r"^([^:]+):(\d+):(\d+)\s+-\s+error\s+(TS\d+):\s+(.*)$") {
            Ok(r) => r,
            Err(_) => return Ok(LintResultList::new(vec![])),
        };

        for line in output.lines() {
            let line = line.trim();
            if let Some(caps) = pattern1.captures(line).or_else(|| pattern2.captures(line)) {
                let filename = match caps.get(1) {
                    Some(m) => m.as_str().to_string(),
                    None => String::new(),
                };
                let line_num = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or(1);
                let col_num = match caps.get(3).and_then(|m| m.as_str().parse::<usize>().ok()) {
                    Some(v) => v,
                    None => 0,
                };
                let code = match caps.get(4) {
                    Some(m) => m.as_str().to_string(),
                    None => String::new(),
                };
                let msg = match caps.get(5) {
                    Some(m) => m.as_str().to_string(),
                    None => String::new(),
                };

                let filename_vo = self.path_norm.resolve_infrastructure_path(
                    FilePath::new(filename).unwrap_or_default(),
                    Some(path.clone()),
                );

                results.push(LintResult {
                    file: filename_vo,
                    line: LineNumber::new(line_num as i64),
                    column: ColumnNumber::new(col_num as i64),
                    code: ErrorCode::raw(&code),
                    message: LintMessage::new(msg),
                    source: Some(self.name()),
                    severity: Severity::HIGH,
                    enclosing_scope: Default::default(),
                    related_locations: Default::default(),
                });
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}
```

---

## File: crates/external-lint/src/infrastructure_py_bandit_adapter.rs

```rust
// PURPOSE: PyBanditAdapter — ILinterAdapterPort implementation for Bandit security scanner integration

use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_adapter_error::AdapterError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;

pub struct BanditAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    bin_path: Option<FilePath>,
}

impl BanditAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            executor,
            path_norm,
            bin_path,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
            None => "bandit".to_string(),
        }
    }

    fn map_severity(&self, severity: &str) -> Severity {
        match severity {
            "HIGH" => Severity::HIGH,
            "MEDIUM" => Severity::MEDIUM,
            "LOW" => Severity::LOW,
            _ => Severity::MEDIUM,
        }
    }
}

#[async_trait]
impl ILinterAdapterPort for BanditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("bandit")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "-r".to_string(),
            path.value.clone(),
            "--format".to_string(),
            "json".to_string(),
            "--exit-zero".to_string(),
        ];
        let command = PatternList::new(cmd);
        let working_dir = match FilePath::new(".".to_string()) {
            Ok(fp) => fp,
            Err(_) => path.clone(),
        };

        match self
            .executor
            .execute_command(
                command,
                working_dir,
                Some(shared::taxonomy_duration_vo::Timeout::new(120.0)),
            )
            .await
        {
            Ok(response) => {
                let stdout = &response.stdout;
                let parsed: Value = match serde_json::from_str(stdout) {
                    Ok(v) => v,
                    Err(_) => Value::Object(serde_json::Map::new()),
                };
                let findings = match parsed.get("results").and_then(|v| v.as_array()) {
                    Some(arr) => arr.clone(),
                    None => Vec::new(),
                };
                let mut results = Vec::new();

                for f in findings {
                    let filename = match f.get("filename").and_then(|v| v.as_str()) {
                        Some(s) => s,
                        None => "",
                    };
                    let line_number = match f.get("line_number").and_then(|v| v.as_i64()) {
                        Some(v) => v,
                        None => 0,
                    };
                    let line_range = match f
                        .get("line_range")
                        .and_then(|v| v.as_array())
                        .and_then(|a| a.first())
                        .and_then(|v| v.as_i64())
                    {
                        Some(v) => v,
                        None => 0,
                    };
                    let test_id = f.get("test_id").and_then(|v| v.as_str()).unwrap_or("B000");
                    let issue_text = match f.get("issue_text").and_then(|v| v.as_str()) {
                        Some(s) => s,
                        None => "",
                    };
                    let issue_severity = f
                        .get("issue_severity")
                        .and_then(|v| v.as_str())
                        .unwrap_or("MEDIUM");

                    let resolved = self.path_norm.resolve_infrastructure_path(
                        match FilePath::new(filename.to_string()) {
                            Ok(fp) => fp,
                            Err(_) => path.clone(),
                        },
                        Some(path.clone()),
                    );

                    results.push(LintResult {
                        file: resolved,
                        line: LineNumber::new(line_number),
                        column: ColumnNumber::new(line_range),
                        code: ErrorCode::raw(test_id),
                        message: LintMessage::new(issue_text),
                        source: Some(self.name()),
                        severity: self.map_severity(issue_severity),
                        enclosing_scope: None,
                        related_locations: LocationList::new(),
                    });
                }
                Ok(LintResultList::new(results))
            }
            Err(e) => Err(LinterOperationError::Adapter(AdapterError::new(
                self.name(),
                ErrorMessage::new(format!("Bandit execution failed: {}", e)),
            ))),
        }
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}
```

---

## File: crates/external-lint/src/infrastructure_py_mypy_adapter.rs

```rust
// PURPOSE: PyMypyAdapter — ILinterAdapterPort implementation for MyPy type checker integration

use async_trait::async_trait;
use regex::Regex;
use std::sync::Arc;

use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_adapter_error::AdapterError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;

pub struct MyPyAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    bin_path: Option<FilePath>,
}

impl MyPyAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            executor,
            path_norm,
            bin_path,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
            None => "mypy".to_string(),
        }
    }

    fn map_severity(msg_type: &str, msg: &str) -> Severity {
        let m = msg.to_lowercase();
        if msg_type == "note" {
            return Severity::LOW;
        }
        if m.contains("syntax") || m.contains("parse") {
            return Severity::CRITICAL;
        }
        if msg_type == "warning" {
            return Severity::MEDIUM;
        }
        Severity::HIGH
    }
}

#[async_trait]
impl ILinterAdapterPort for MyPyAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("mypy")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            path.value.clone(),
            "--no-error-summary".to_string(),
            "--pretty".to_string(),
            "false".to_string(),
        ];
        let command = PatternList::new(cmd);
        let working_dir = match FilePath::new(".".to_string()) {
            Ok(fp) => fp,
            Err(_) => path.clone(),
        };

        match self
            .executor
            .execute_command(
                command,
                working_dir,
                Some(shared::taxonomy_duration_vo::Timeout::new(120.0)),
            )
            .await
        {
            Ok(response) => {
                let stdout = &response.stdout;
                let re = match Regex::new(r"^([^:]+):(\d+):(\d+):\s+(\w+):\s+(.+?)\s+\[(\w+)\]$") {
                    Ok(r) => r,
                    Err(_) => match Regex::new(r"^([^:]+):(\d+):\s+(\w+):\s+(.+?)\s+\[(\w+)\]$") {
                        Ok(r) => r,
                        Err(_) => return Ok(LintResultList::new(vec![])),
                    },
                };
                let re_simple = match Regex::new(r"^([^:]+):(\d+):\s+(\w+):\s+(.+?)\s+\[(\w+)\]$") {
                    Ok(r) => r,
                    Err(_) => return Ok(LintResultList::new(vec![])),
                };
                let mut results = Vec::new();

                for line in stdout.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    if let Some(caps) = re.captures(line) {
                        let filename = match caps.get(1) {
                            Some(m) => m.as_str(),
                            None => "",
                        };
                        let line_number: i64 =
                            match caps.get(2).and_then(|m| m.as_str().parse().ok()) {
                                Some(v) => v,
                                None => 0,
                            };
                        let column: i64 = match caps.get(3).and_then(|m| m.as_str().parse().ok()) {
                            Some(v) => v,
                            None => 0,
                        };
                        let msg_type = match caps.get(4) {
                            Some(m) => m.as_str(),
                            None => "error",
                        };
                        let message = match caps.get(5) {
                            Some(m) => m.as_str(),
                            None => "",
                        };
                        let code = match caps.get(6) {
                            Some(m) => m.as_str(),
                            None => "",
                        };

                        let resolved = self.path_norm.resolve_infrastructure_path(
                            match FilePath::new(filename.to_string()) {
                                Ok(fp) => fp,
                                Err(_) => path.clone(),
                            },
                            Some(path.clone()),
                        );

                        results.push(LintResult {
                            file: resolved,
                            line: LineNumber::new(line_number),
                            column: ColumnNumber::new(column),
                            code: ErrorCode::raw(code),
                            message: LintMessage::new(message),
                            source: Some(self.name()),
                            severity: Self::map_severity(msg_type, message),
                            enclosing_scope: None,
                            related_locations: LocationList::new(),
                        });
                    } else if let Some(caps) = re_simple.captures(line) {
                        let filename = match caps.get(1) {
                            Some(m) => m.as_str(),
                            None => "",
                        };
                        let line_number: i64 =
                            match caps.get(2).and_then(|m| m.as_str().parse().ok()) {
                                Some(v) => v,
                                None => 0,
                            };
                        let msg_type = match caps.get(3) {
                            Some(m) => m.as_str(),
                            None => "error",
                        };
                        let message = match caps.get(4) {
                            Some(m) => m.as_str(),
                            None => "",
                        };
                        let code = match caps.get(5) {
                            Some(m) => m.as_str(),
                            None => "",
                        };

                        let resolved = self.path_norm.resolve_infrastructure_path(
                            match FilePath::new(filename.to_string()) {
                                Ok(fp) => fp,
                                Err(_) => path.clone(),
                            },
                            Some(path.clone()),
                        );

                        results.push(LintResult {
                            file: resolved,
                            line: LineNumber::new(line_number),
                            column: ColumnNumber::new(0),
                            code: ErrorCode::raw(code),
                            message: LintMessage::new(message),
                            source: Some(self.name()),
                            severity: Self::map_severity(msg_type, message),
                            enclosing_scope: None,
                            related_locations: LocationList::new(),
                        });
                    }
                }
                Ok(LintResultList::new(results))
            }
            Err(e) => Err(LinterOperationError::Adapter(AdapterError::new(
                self.name(),
                ErrorMessage::new(format!("MyPy execution failed: {}", e)),
            ))),
        }
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}
```

---

## File: crates/external-lint/src/infrastructure_py_ruff_adapter.rs

```rust
// PURPOSE: PyRuffAdapter — ILinterAdapterPort implementation for Ruff linter integration

use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_adapter_error::AdapterError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;

pub struct RuffAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    bin_path: Option<FilePath>,
}

impl RuffAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            executor,
            path_norm,
            bin_path,
        }
    }

    fn resolve_executable(&self) -> String {
        match self.bin_path.as_ref() {
            Some(p) => p.value.clone(),
            None => "ruff".to_string(),
        }
    }

    fn map_severity(&self, severity: &str, _code: &str) -> Severity {
        match severity {
            "error" => Severity::HIGH,
            "warning" => Severity::MEDIUM,
            "info" => Severity::LOW,
            _ => Severity::MEDIUM,
        }
    }
}

#[async_trait]
impl ILinterAdapterPort for RuffAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("ruff")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value.clone(),
            "--output-format=json".to_string(),
            "--exit-zero".to_string(),
            "--no-cache".to_string(),
        ];
        let command = PatternList::new(cmd);
        let working_dir = match FilePath::new(".".to_string()) {
            Ok(fp) => fp,
            Err(_) => path.clone(),
        };

        match self
            .executor
            .execute_command(
                command,
                working_dir,
                Some(shared::taxonomy_duration_vo::Timeout::new(60.0)),
            )
            .await
        {
            Ok(response) => {
                let stdout = &response.stdout;
                let findings: Vec<Value> = match serde_json::from_str(stdout) {
                    Ok(v) => v,
                    Err(_) => Vec::new(),
                };
                let mut results = Vec::new();

                for f in findings {
                    let filename = match f.get("filename").and_then(|v| v.as_str()) {
                        Some(s) => s,
                        None => "",
                    };
                    let row = match f
                        .get("location")
                        .and_then(|l| l.get("row"))
                        .and_then(|v| v.as_i64())
                    {
                        Some(v) => v,
                        None => 0,
                    };
                    let col = match f
                        .get("location")
                        .and_then(|l| l.get("column"))
                        .and_then(|v| v.as_i64())
                    {
                        Some(v) => v,
                        None => 0,
                    };
                    let code = f.get("code").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
                    let message = match f.get("message").and_then(|v| v.as_str()) {
                        Some(s) => s,
                        None => "",
                    };
                    let severity_str = match f.get("severity").and_then(|v| v.as_str()) {
                        Some(s) => s,
                        None => "",
                    };

                    let resolved = self.path_norm.resolve_infrastructure_path(
                        match FilePath::new(filename) {
                            Ok(fp) => fp,
                            Err(_) => path.clone(),
                        },
                        Some(path.clone()),
                    );

                    results.push(LintResult {
                        file: resolved,
                        line: LineNumber::new(row),
                        column: ColumnNumber::new(col),
                        code: ErrorCode::raw(code),
                        message: LintMessage::new(message),
                        source: Some(self.name()),
                        severity: self.map_severity(severity_str, code),
                        enclosing_scope: None,
                        related_locations: LocationList::new(),
                    });
                }
                Ok(LintResultList::new(results))
            }
            Err(e) => Err(LinterOperationError::Adapter(AdapterError::new(
                self.name(),
                ErrorMessage::new(format!("Ruff execution failed: {}", e)),
            ))),
        }
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let executable = self.resolve_executable();
        let cmd = vec![
            executable,
            "check".to_string(),
            path.value.clone(),
            "--fix".to_string(),
            "--exit-zero".to_string(),
        ];
        let command = PatternList::new(cmd);
        let working_dir = match FilePath::new(".".to_string()) {
            Ok(fp) => fp,
            Err(_) => path.clone(),
        };

        match self
            .executor
            .execute_command(
                command,
                working_dir,
                Some(shared::taxonomy_duration_vo::Timeout::new(60.0)),
            )
            .await
        {
            Ok(_) => Ok(ComplianceStatus::new(true)),
            Err(e) => Err(LinterOperationError::Adapter(AdapterError::new(
                self.name(),
                ErrorMessage::new(format!("Ruff fix failed: {}", e)),
            ))),
        }
    }
}
```

---

## File: crates/external-lint/src/infrastructure_rs_audit_adapter.rs

```rust
// PURPOSE: RsAuditAdapter — ILinterAdapterPort implementation for cargo-audit security scanning
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use tracing::debug;

pub struct CargoAuditAdapter {
    path_norm: Arc<dyn IPathNormalizationPort>,
}

impl CargoAuditAdapter {
    pub fn new(path_norm: Arc<dyn IPathNormalizationPort>) -> Self {
        Self { path_norm }
    }

    fn _resolve_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }

        let current = std::path::Path::new(path_str);
        if current.is_dir() {
            if current.join("Cargo.lock").exists() {
                return path.clone();
            }
        } else if let Some(parent) = current.parent() {
            if parent.join("Cargo.lock").exists() {
                return match FilePath::new(parent.to_string_lossy().replace('\\', "/")) {
                    Ok(fp) => fp,
                    Err(_) => path.clone(),
                };
            }
            if let Some(grandparent) = parent.parent() {
                if grandparent.join("Cargo.lock").exists() {
                    return match FilePath::new(grandparent.to_string_lossy().replace('\\', "/")) {
                        Ok(fp) => fp,
                        Err(_) => path.clone(),
                    };
                }
            }
        }

        FilePath::new("nonexistent_directory_for_cargo_lock".to_string()).unwrap_or_default()
    }
}

#[async_trait]
impl ILinterAdapterPort for CargoAuditAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("cargo-audit")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = self._resolve_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_lock = Path::new(working_dir_str).join("Cargo.lock");
        if !cargo_lock.exists() {
            debug!(
                "Skipping cargo-audit: Cargo.lock not found at {:?}",
                cargo_lock
            );
            return Ok(LintResultList::new(results));
        }

        let lockfile = match rustsec::Lockfile::load(&cargo_lock) {
            Ok(lf) => lf,
            Err(e) => {
                debug!("Failed to parse Cargo.lock: {}", e);
                return Ok(LintResultList::new(results));
            }
        };

        let db_dir = match dirs::home_dir() {
            Some(p) => p,
            None => std::path::PathBuf::from("."),
        }
        .join(".cargo")
        .join("advisory-db");
        let db = if db_dir.exists() {
            match rustsec::Database::open(&db_dir) {
                Ok(db) => db,
                Err(_) => {
                    debug!("Failed to open advisory DB, will fetch...");
                    match rustsec::Database::fetch() {
                        Ok(db) => db,
                        Err(e) => {
                            debug!("Failed to fetch advisory DB: {}", e);
                            return Ok(LintResultList::new(results));
                        }
                    }
                }
            }
        } else {
            match rustsec::Database::fetch() {
                Ok(db) => db,
                Err(e) => {
                    debug!("Failed to fetch advisory DB: {}", e);
                    return Ok(LintResultList::new(results));
                }
            }
        };

        let settings = rustsec::report::Settings::default();
        let report = rustsec::Report::generate(&db, &lockfile, &settings);

        for vuln in &report.vulnerabilities.list {
            let id = vuln.advisory.id.to_string();
            let title = &vuln.advisory.title;
            let severity_str = vuln
                .advisory
                .cvss
                .as_ref()
                .map(|c| c.severity().to_string().to_lowercase())
                .unwrap_or_else(|| "moderate".to_string());

            let severity = match severity_str.as_str() {
                "critical" => Severity::CRITICAL,
                "high" => Severity::HIGH,
                "medium" => Severity::MEDIUM,
                _ => Severity::LOW,
            };

            let resolved = self.path_norm.resolve_infrastructure_path(
                FilePath::new("Cargo.lock".to_string()).unwrap_or_else(|_| path.clone()),
                Some(path.clone()),
            );
            results.push(LintResult {
                file: resolved,
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw(format!("cargo-audit::{}", id)),
                message: LintMessage::new(format!(
                    "{}: {} ({} v{})",
                    id, title, vuln.package.name, vuln.package.version
                )),
                source: Some(AdapterName::raw("cargo-audit")),
                severity,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(true))
    }
}
```

---

## File: crates/external-lint/src/infrastructure_rs_clippy_adapter.rs

```rust
// PURPOSE: RsClippyAdapter — ILinterAdapterPort implementation for Clippy linting integration
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_adapter_error::AdapterError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use tracing::debug;

/// Adapter for Rust Clippy static analysis.
pub struct RustLinterAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    _bin_path: Option<FilePath>,
}

impl RustLinterAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            executor,
            path_norm,
            _bin_path: bin_path,
        }
    }

    fn _resolve_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }

        let current = std::path::Path::new(path_str);
        if current.is_dir() {
            if current.join("Cargo.toml").exists() {
                return path.clone();
            }
        } else if let Some(parent) = current.parent() {
            if parent.join("Cargo.toml").exists() {
                return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
            if let Some(grandparent) = parent.parent() {
                if grandparent.join("Cargo.toml").exists() {
                    return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_else(|_| path.clone());
                }
            }
        }

        FilePath::new("nonexistent_directory_for_cargo_toml".to_string()).unwrap_or_default()
    }
}

#[async_trait]
impl ILinterAdapterPort for RustLinterAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("clippy")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = self._resolve_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_toml = Path::new(working_dir_str).join("Cargo.toml");
        if !cargo_toml.exists() {
            debug!(
                "Skipping clippy scan: Cargo.toml not found at {:?}",
                cargo_toml
            );
            return Ok(LintResultList::new(results));
        }

        let cmd = vec![
            "cargo".to_string(),
            "clippy".to_string(),
            "--message-format=json".to_string(),
        ];
        let result = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir.clone(),
                Some(shared::taxonomy_duration_vo::Timeout::new(180.0)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(e.to_string()),
                ))
            })?;

        let output = if result.stdout.trim().is_empty() {
            result.stderr.clone()
        } else {
            result.stdout.clone()
        };

        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() || !line.starts_with('{') {
                continue;
            }
            match serde_json::from_str::<Value>(line) {
                Ok(data) => {
                    if data.get("reason").and_then(|r| r.as_str()) != Some("compiler-message") {
                        continue;
                    }
                    let msg = match data.get("message") {
                        Some(m) => m,
                        None => continue,
                    };
                    let level = msg
                        .get("level")
                        .and_then(|l| l.as_str())
                        .unwrap_or("warning")
                        .to_lowercase();
                    let code = msg
                        .get("code")
                        .and_then(|c| c.get("code"))
                        .and_then(|c| c.as_str())
                        .unwrap_or("clippy::warning")
                        .to_string();
                    let message_text = msg
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("Clippy finding")
                        .to_string();
                    let spans: Vec<Value> = msg
                        .get("spans")
                        .and_then(|s| s.as_array())
                        .cloned()
                        .unwrap_or_default();

                    for span in &spans {
                        let is_primary = span
                            .get("is_primary")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        if !is_primary {
                            continue;
                        }
                        let filename = match span.get("file_name").and_then(|f| f.as_str()) {
                            Some(f) if !f.is_empty() => f,
                            _ => continue,
                        };
                        let resolved_file = self.path_norm.resolve_infrastructure_path(
                            FilePath::new(filename.to_string()).unwrap_or_else(|_| path.clone()),
                            Some(path.clone()),
                        );
                        let line_num =
                            span.get("line_start").and_then(|v| v.as_u64()).unwrap_or(1) as i64;
                        let column_num = span
                            .get("column_start")
                            .and_then(|v| v.as_u64())
                            .unwrap_or(1) as i64;
                        let severity = if level == "error" {
                            Severity::HIGH
                        } else {
                            Severity::MEDIUM
                        };
                        results.push(LintResult {
                            file: resolved_file,
                            line: LineNumber::new(line_num),
                            column: ColumnNumber::new(column_num),
                            code: ErrorCode::raw(code.as_str()),
                            message: LintMessage::new(message_text.as_str()),
                            source: Some(AdapterName::raw("clippy")),
                            severity,
                            enclosing_scope: None,
                            related_locations: LocationList::new(),
                        });
                    }
                }
                Err(_) => continue,
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let working_dir = self._resolve_working_dir(path);
        let cmd = vec![
            "cargo".to_string(),
            "clippy".to_string(),
            "--fix".to_string(),
            "--allow-dirty".to_string(),
            "--allow-staged".to_string(),
        ];
        let _ = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir,
                Some(shared::taxonomy_duration_vo::Timeout::new(180.0)),
            )
            .await;
        Ok(ComplianceStatus::new(true))
    }
}
```

---

## File: crates/external-lint/src/infrastructure_rs_fmt_adapter.rs

```rust
// PURPOSE: RsFmtAdapter — ILinterAdapterPort implementation for rustfmt integration
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_adapter_error::AdapterError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use tracing::debug;

pub struct RustFmtAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    _bin_path: Option<FilePath>,
}

impl RustFmtAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
        bin_path: Option<FilePath>,
    ) -> Self {
        Self {
            executor,
            path_norm,
            _bin_path: bin_path,
        }
    }

    fn _resolve_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }
        let current = std::path::Path::new(path_str);
        if current.is_dir() {
            if current.join("Cargo.toml").exists() {
                return path.clone();
            }
        } else if let Some(parent) = current.parent() {
            if parent.join("Cargo.toml").exists() {
                return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
            if let Some(grandparent) = parent.parent() {
                if grandparent.join("Cargo.toml").exists() {
                    return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_else(|_| path.clone());
                }
            }
        }
        FilePath::new("nonexistent_directory_for_cargo_toml".to_string()).unwrap_or_default()
    }
}

#[async_trait]
impl ILinterAdapterPort for RustFmtAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("rustfmt")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let mut results = Vec::new();
        let working_dir = self._resolve_working_dir(path);
        let working_dir_str = &working_dir.value;

        let cargo_toml = Path::new(working_dir_str).join("Cargo.toml");
        if !cargo_toml.exists() {
            debug!("Skipping rustfmt: Cargo.toml not found at {:?}", cargo_toml);
            return Ok(LintResultList::new(results));
        }

        let cmd = vec![
            "cargo".to_string(),
            "fmt".to_string(),
            "--check".to_string(),
        ];
        let result = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir.clone(),
                Some(shared::taxonomy_duration_vo::Timeout::new(120.0)),
            )
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    self.name(),
                    ErrorMessage::new(e.to_string()),
                ))
            })?;

        if result.returncode == 0 {
            return Ok(LintResultList::new(results));
        }

        let output = result.stdout + &result.stderr;
        let mut current_file = String::new();
        for line in output.lines() {
            if line.starts_with("Diff in ") {
                current_file = line
                    .trim_start_matches("Diff in ")
                    .trim_end_matches(':')
                    .trim()
                    .to_string();
            } else if line.starts_with("--- ") || line.starts_with("+++ ") {
                continue;
            }

            if line.starts_with('+') && !line.starts_with("+++") {
                let resolved = self.path_norm.resolve_infrastructure_path(
                    FilePath::new(current_file.clone()).unwrap_or_else(|_| path.clone()),
                    Some(path.clone()),
                );
                results.push(LintResult {
                    file: resolved,
                    line: LineNumber::new(0),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::raw("rustfmt::unformatted"),
                    message: LintMessage::new(line.trim().to_string()),
                    source: Some(AdapterName::raw("rustfmt")),
                    severity: Severity::MEDIUM,
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
        }

        if results.is_empty() {
            results.push(LintResult {
                file: FilePath::new("Cargo.toml".to_string()).unwrap_or_default(),
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw("rustfmt::unformatted"),
                message: LintMessage::new("Project is not formatted by rustfmt".to_string()),
                source: Some(AdapterName::raw("rustfmt")),
                severity: Severity::MEDIUM,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let working_dir = self._resolve_working_dir(path);
        let cmd = vec!["cargo".to_string(), "fmt".to_string()];
        let _ = self
            .executor
            .execute_command(
                PatternList::new(cmd),
                working_dir,
                Some(shared::taxonomy_duration_vo::Timeout::new(120.0)),
            )
            .await;
        Ok(ComplianceStatus::new(true))
    }
}
```

---

## File: crates/external-lint/src/infrastructure_stdio_client.rs

```rust
// PURPOSE: StdioClient — ICommandExecutorPort implementation via stdio
use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::mcp_server::taxonomy_job_vo::ResponseData;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_duration_vo::Timeout;
use tokio::process::Command;

pub struct StdioClient {
    timeout: Duration,
}

impl StdioClient {
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }
}

#[async_trait]
impl ICommandExecutorPort for StdioClient {
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        let timeout_val = match timeout {
            Some(d) => Duration::from_secs(d.value() as u64),
            None => self.timeout,
        };
        let cmd_list: Vec<&str> = command.values.iter().map(|s| s.as_str()).collect();
        if cmd_list.is_empty() {
            anyhow::bail!("Empty command");
        }
        let mut cmd = Command::new(cmd_list[0]);
        if cmd_list.len() > 1 {
            cmd.args(&cmd_list[1..]);
        }
        cmd.current_dir(&working_dir.value)
            .env("PYTHONUNBUFFERED", "1");
        cmd.kill_on_drop(true);

        let result = tokio::time::timeout(timeout_val, cmd.output()).await;
        match result {
            Ok(Ok(output)) => {
                let mut meta_map = HashMap::new();
                meta_map.insert(
                    "protocol".to_string(),
                    serde_json::Value::String("Stdio".to_string()),
                );
                Ok(ResponseData {
                    value: Some(serde_json::Value::Null),
                    stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                    stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                    returncode: match output.status.code() {
                        Some(c) => c as i64,
                        None => -1,
                    },
                    metadata: meta_map,
                })
            }
            Ok(Err(e)) => anyhow::bail!("Command execution failed: {}", e),
            Err(_) => anyhow::bail!("Command timed out after {}s", timeout_val.as_secs()),
        }
    }

    async fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}
```

---

## File: crates/external-lint/src/lib.rs

```rust
// PURPOSE: Module declarations for external-lint (external linter adapters)
pub use shared::source_parsing::taxonomy_adapter_error::{
    AdapterError, ScanError, ValidationError,
};

pub mod agent_external_lint_orchestrator;
pub mod infrastructure_stdio_client;
pub use infrastructure_stdio_client::StdioClient;
pub mod infrastructure_js_eslint_adapter;
pub use infrastructure_js_eslint_adapter::ESLintAdapter;
pub mod infrastructure_js_prettier_adapter;
pub use infrastructure_js_prettier_adapter::PrettierAdapter;
pub mod infrastructure_js_tsc_adapter;
pub use infrastructure_js_tsc_adapter::TSCAdapter;
pub mod infrastructure_py_bandit_adapter;
pub use infrastructure_py_bandit_adapter::BanditAdapter;
pub mod infrastructure_py_mypy_adapter;
pub use infrastructure_py_mypy_adapter::MyPyAdapter;
pub mod infrastructure_py_ruff_adapter;
pub use infrastructure_py_ruff_adapter::RuffAdapter;
pub mod infrastructure_rs_audit_adapter;
pub use infrastructure_rs_audit_adapter::CargoAuditAdapter;
pub mod infrastructure_rs_fmt_adapter;
pub use infrastructure_rs_fmt_adapter::RustFmtAdapter;
pub mod infrastructure_rs_clippy_adapter;
pub use infrastructure_rs_clippy_adapter::RustLinterAdapter;
pub mod root_external_lint_container;
pub use root_external_lint_container::ExternalLintContainer;
```

---

## File: crates/external-lint/src/root_external_lint_container.rs

```rust
// PURPOSE: ExternalLintContainer — root layer, wires orchestrator with infrastructure adapters
use std::collections::HashMap;
use std::sync::Arc;

use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;

pub struct ExternalLintContainer {
    aggregate: Arc<dyn IExternalLintAggregate>,
}

impl ExternalLintContainer {
    pub fn new(path_norm: Arc<dyn IPathNormalizationPort>) -> Self {
        let executor: Arc<dyn shared::cli_commands::contract_executor_port::ICommandExecutorPort> =
            Arc::new(crate::infrastructure_stdio_client::StdioClient::new(
                std::time::Duration::from_secs(60),
            ));
        let mut adapters: HashMap<String, Arc<dyn ILinterAdapterPort>> = HashMap::new();
        adapters.insert(
            "ruff".to_string(),
            Arc::new(crate::infrastructure_py_ruff_adapter::RuffAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            )),
        );
        adapters.insert(
            "bandit".to_string(),
            Arc::new(crate::infrastructure_py_bandit_adapter::BanditAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            )),
        );
        adapters.insert(
            "mypy".to_string(),
            Arc::new(crate::infrastructure_py_mypy_adapter::MyPyAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            )),
        );
        adapters.insert(
            "eslint".to_string(),
            Arc::new(crate::infrastructure_js_eslint_adapter::ESLintAdapter::new(
                executor.clone(),
                path_norm.clone(),
            )),
        );
        adapters.insert(
            "prettier".to_string(),
            Arc::new(
                crate::infrastructure_js_prettier_adapter::PrettierAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                ),
            ),
        );
        adapters.insert(
            "tsc".to_string(),
            Arc::new(crate::infrastructure_js_tsc_adapter::TSCAdapter::new(
                executor.clone(),
                path_norm.clone(),
            )),
        );
        adapters.insert(
            "clippy".to_string(),
            Arc::new(
                crate::infrastructure_rs_clippy_adapter::RustLinterAdapter::new(
                    executor.clone(),
                    path_norm.clone(),
                    None,
                ),
            ),
        );
        adapters.insert(
            "rustfmt".to_string(),
            Arc::new(crate::infrastructure_rs_fmt_adapter::RustFmtAdapter::new(
                executor.clone(),
                path_norm.clone(),
                None,
            )),
        );
        adapters.insert(
            "cargo-audit".to_string(),
            Arc::new(
                crate::infrastructure_rs_audit_adapter::CargoAuditAdapter::new(path_norm.clone()),
            ),
        );

        Self {
            aggregate: Arc::new(
                crate::agent_external_lint_orchestrator::ExternalLintOrchestrator::new(adapters),
            ),
        }
    }

    pub fn aggregate(&self) -> Arc<dyn IExternalLintAggregate> {
        self.aggregate.clone()
    }
}
```

---

## File: crates/shared/src/auto-fix/taxonomy_fix_applied_event.rs

```rust
// PURPOSE: FixApplied — domain event published when a lint fix is applied
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::Timestamp;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FixApplied {
    pub path: FilePath,
    pub adapter: AdapterName,
    pub error_code: ErrorCode,
    pub changes_count: Count,
    #[serde(default)]
    pub timestamp: Timestamp,
}

impl FixApplied {
    pub fn new(
        path: FilePath,
        adapter: AdapterName,
        error_code: ErrorCode,
        changes_count: Count,
    ) -> Self {
        Self {
            path,
            adapter,
            error_code,
            changes_count,
            timestamp: Timestamp::default(),
        }
    }
}
```

---

## File: crates/shared/src/cli-commands/contract_executor_port.rs

```rust
// PURPOSE: Port: ICommandExecutorPort — trait for executing shell commands and capturing response
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_duration_vo::Timeout;
use crate::mcp_server::taxonomy_job_vo::ResponseData;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[async_trait::async_trait]
pub trait ICommandExecutorPort: Send + Sync {
    /// Execute a command and return the response.
    async fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData>;

    /// Check the health of the execution transport.
    async fn health_check(&self) -> anyhow::Result<ResponseData>;
}
```

---

## File: crates/shared/src/cli-commands/mod.rs

```rust
// cli-commands — taxonomy and contract types
pub mod contract_executor_port;
pub mod taxonomy_catalog_constant;
pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_score_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_transport_error;
```

---

## File: crates/shared/src/cli-commands/taxonomy_result_vo.rs

```rust
// PURPOSE: LintResult, LintResultList, FilePathSet — value objects for lint violation results
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_position_vo::Position;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResult {
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub code: ErrorCode,
    pub message: LintMessage,
    pub source: Option<AdapterName>,
    pub severity: Severity,
    pub enclosing_scope: Option<ScopeRef>,
    pub related_locations: LocationList,
}

impl LintResult {
    /// Convenience constructor used by architecture checkers (make_result / mk pattern).
    pub fn new_arch(
        file: &str,
        line: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    pub fn position(&self) -> Position {
        Position {
            line: self.line.clone(),
            column: self.column.clone(),
        }
    }
    pub fn identity(&self) -> Identity {
        Identity::new(format!(
            "{}:{}:{}:{:?}",
            self.file, self.line, self.code, self.source
        ))
    }
}

/// Generate a `Vec<T>`-backed newtype with `Default`, `new`, `iter`,
/// `len`, `is_empty`, `push`, and `append`. Used for the `LintResultList`
/// wrapper below; siblings `ImportInfoList`/`PrimitiveViolationList` in
/// `taxonomy_import_source_vo.rs` carry the same surface.
macro_rules! lint_result_list_wrapper {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub values: Vec<$item>,
        }

        impl $name {
            pub fn new(value: Vec<$item>) -> Self {
                Self { values: value }
            }
            pub fn iter(&self) -> std::slice::Iter<'_, $item> {
                self.values.iter()
            }
            pub fn len(&self) -> usize {
                self.values.len()
            }
            pub fn is_empty(&self) -> bool {
                self.values.is_empty()
            }
            pub fn push(&mut self, item: $item) {
                self.values.push(item);
            }
            pub fn append(&mut self, item: $item) {
                self.values.push(item);
            }
        }
    };
}

lint_result_list_wrapper!(LintResultList, LintResult);
```

---

## File: crates/shared/src/cli-commands/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — re-export from common for backward compatibility
//
// This module exists so dependents can keep using the
// `cli_commands::taxonomy_severity_vo::Severity` import path. The real
// definition lives in `common::taxonomy_severity_vo` and is re-exported
// here to avoid breaking any code that still imports from the legacy path.
pub use crate::common::taxonomy_severity_vo::Severity;
```

---

## File: crates/shared/src/code-analysis/contract_adapter_port.rs

```rust
// PURPOSE: ILinterAdapterPort — port trait for linter adapter implementations (Ruff, Mypy, Clippy, etc.)

use async_trait::async_trait;

use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::taxonomy_operation_error::LinterOperationError;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_message_vo::ComplianceStatus;
use crate::source_parsing::taxonomy_path_vo::FilePath;

/// Abstract interface for linter adapters.
/// Implemented by Infrastructure (e.g., RuffAdapter, MypyAdapter).
#[async_trait]
pub trait ILinterAdapterPort: Send + Sync {
    fn name(&self) -> AdapterName;
    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError>;
    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError>;
}
```

---

## File: crates/shared/src/code-analysis/mod.rs

```rust
// code-analysis — taxonomy and contract types
pub mod contract_adapter_port;
pub mod contract_bypass_checker_protocol;
pub mod contract_class_protocol;
pub mod contract_code_analysis_aggregate;
pub mod contract_code_metric_analyzer_protocol;
pub mod contract_cycle_protocol;
pub mod contract_dead_inheritance_protocol;
pub mod contract_layer_detection_aggregate;
pub mod contract_line_protocol;
pub mod taxonomy_analysis_vo;
pub mod taxonomy_code_analysis_rule_vo;
pub mod taxonomy_governance_entity;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
```

---

## File: crates/shared/src/code-analysis/taxonomy_operation_error.rs

```rust
// PURPOSE: LinterOperationError — structured error type for linter operation failures (scan, fix, report)
/// linter_operation_error — Unified error type for linter adapter operations.
/* UNKNOWN: ErrorMessage */
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::source_parsing::taxonomy_adapter_error::AdapterError;
use crate::source_parsing::taxonomy_adapter_error::ScanError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum LinterOperationError {
    #[error("Scan error: {0}")]
    Scan(ScanError),

    #[error("Adapter error: {0}")]
    Adapter(AdapterError),
}

impl LinterOperationError {
    pub fn message(&self) -> ErrorMessage {
        let _ = &LineNumber::default();
        ErrorMessage::new(self.to_string())
    }
}

impl From<ScanError> for LinterOperationError {
    fn from(e: ScanError) -> Self {
        LinterOperationError::Scan(e)
    }
}

impl From<AdapterError> for LinterOperationError {
    fn from(e: AdapterError) -> Self {
        LinterOperationError::Adapter(e)
    }
}
```

---

## File: crates/shared/src/common/mod.rs

```rust
// common — truly shared types used by multiple features
pub mod taxonomy_action_vo;
pub mod taxonomy_adapter_name_vo;
pub mod taxonomy_common_error;
pub mod taxonomy_common_vo;
pub mod taxonomy_definition_vo;
pub mod taxonomy_duration_vo;
pub mod taxonomy_error_vo;
pub mod taxonomy_job_id_vo;
pub mod taxonomy_job_vo;
pub mod taxonomy_layer_vo;
pub mod taxonomy_lint_vo;
pub mod taxonomy_message_vo;
pub mod taxonomy_name_vo;
pub mod taxonomy_response_data_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_suggestion_vo;
pub mod taxonomy_value_object_utility;
```

---

## File: crates/shared/src/common/taxonomy_adapter_name_vo.rs

```rust
// PURPOSE: AdapterName — validated newtype for adapter/linter name strings
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// adapter_name_vo — Adapter and tool identifier value objects.
///
/// Adapter/tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AdapterName {
    pub value: String,
}

impl AdapterName {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new AdapterName from a string.
    ///
    /// # Errors
    /// Returns an error if the adapter name is empty or only whitespace.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("Adapter name cannot be empty".to_string());
        }
        Ok(AdapterName {
            value: value.trim().to_string(),
        })
    }

    /// Create a raw AdapterName without error validation (for static compile-time safe inputs).
    pub fn raw<S: Into<String>>(value: S) -> Self {
        AdapterName {
            value: value.into(),
        }
    }
}

impl std::ops::Deref for AdapterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for AdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for AdapterName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::AdapterName;

    #[test]
    fn test_adapter_name_new() {
        let name = AdapterName::new("ruff").unwrap_or_default();
        assert_eq!(name.value, "ruff");

        // Test trimming
        let name = AdapterName::new("  ruff  ").unwrap_or_default();
        assert_eq!(name.value, "ruff");

        // Test that internal spaces are preserved
        let name = AdapterName::new("my adapter").unwrap_or_default();
        assert_eq!(name.value, "my adapter");
    }

    #[test]
    fn test_adapter_name_invalid() {
        assert!(AdapterName::new("").is_err());
        assert!(AdapterName::new("   ").is_err());
        assert!(AdapterName::new("\t\n  ").is_err());
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_vo.rs

```rust
// PURPOSE: BooleanVO, ColumnNumber, Count, DataFlowList, LineContentList, LineNumber, PatternList, Score, Timestamp — common VOs
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_severity_vo::Severity;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct BooleanVO {
    pub value: bool,
}

impl BooleanVO {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for BooleanVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for BooleanVO {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for BooleanVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BooleanVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for BooleanVOVisitor {
            type Value = BooleanVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanVO { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<bool>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(BooleanVO { value: val })
            }
        }
        deserializer.deserialize_any(BooleanVOVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ColumnNumber {
    pub value: i64,
}

impl ColumnNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ColumnNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for ColumnNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColumnNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for ColumnNumberVisitor {
            type Value = ColumnNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ColumnNumber { value: val })
            }
        }
        deserializer.deserialize_any(ColumnNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Count {
    pub value: i64,
}

impl Count {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for Count {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Count {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CountVisitor {}
        impl<'de> serde::de::Visitor<'de> for CountVisitor {
            type Value = Count;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Count { value: val })
            }
        }
        deserializer.deserialize_any(CountVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataFlowList {
    pub values: Vec<ErrorMessage>,
}

impl DataFlowList {
    pub fn new(value: Vec<ErrorMessage>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ErrorMessage] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ErrorMessage> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ErrorMessage) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobIdList {
    pub values: Vec<JobId>,
}

impl JobIdList {
    pub fn new(value: Vec<JobId>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[JobId] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, JobId> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: JobId) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineContentList {
    pub values: Vec<LineContentVO>,
}

impl LineContentList {
    pub fn new(value: Vec<LineContentVO>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[LineContentVO] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LineContentVO> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LineContentVO) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
#[derive(Default)]
pub struct LineNumber {
    pub value: i64,
}

impl LineNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for LineNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for LineNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LineNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for LineNumberVisitor {
            type Value = LineNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(LineNumber { value: val })
            }
        }
        deserializer.deserialize_any(LineNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct PatternList {
    pub values: Vec<String>,
}

impl PatternList {
    pub fn new(value: impl IntoPatternListValues) -> Self {
        Self {
            values: value.into_pattern_list_values(),
        }
    }
    pub fn values(&self) -> &[String] {
        &self.values
    }
}

impl PatternList {
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: String) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseDataList {
    pub values: Vec<ResponseData>,
}

impl ResponseDataList {
    pub fn new(value: Vec<ResponseData>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ResponseData] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ResponseData> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ResponseData) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Score {
    pub value: f64,
}

impl Score {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn is_perfect(&self) -> bool {
        self.value >= 100.0
    }
    pub fn is_passing(&self, threshold: &Score) -> bool {
        self.value >= threshold.value
    }
    pub fn deduct(&self, severity: &Severity) -> Score {
        Score {
            value: self.value - severity.score_impact(),
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.value)
    }
}

impl From<f64> for Score {
    fn from(v: f64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ScoreVisitor {}
        impl<'de> serde::de::Visitor<'de> for ScoreVisitor {
            type Value = Score;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<f64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Score { value: val })
            }
        }
        deserializer.deserialize_any(ScoreVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Timestamp {
    pub value: String,
}

impl Timestamp {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn now() -> Self {
        Self {
            value: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Timestamp {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Timestamp {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimestampVisitor {}
        impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
            type Value = Timestamp;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Timestamp { value: val })
            }
        }
        deserializer.deserialize_any(TimestampVisitor {})
    }
}

// Custom Coercion Traits for PatternList

pub trait IntoPatternListValues {
    fn into_pattern_list_values(self) -> Vec<String>;
}

impl IntoPatternListValues for &str {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IntoPatternListValues for String {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self]
    }
}

impl IntoPatternListValues for Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self
    }
}

impl IntoPatternListValues for Vec<&str> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.into_iter().map(|s| s.to_string()).collect()
    }
}

impl IntoPatternListValues for &Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ErrorMessage {
    pub value: String,
}

impl ErrorMessage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ErrorMessage {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ErrorMessage {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_error_vo.rs

```rust
// PURPOSE: ErrorCode — value object for AES error code identification
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// error_code_vo — Error code value object.
///
/// Linter error code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ErrorCode {
    code: String,
}

impl ErrorCode {
    pub fn code(&self) -> &str {
        &self.code
    }
    /// Create a new ErrorCode from a string.
    ///
    /// # Errors
    /// Returns an error if the code is empty.
    pub fn new<S: Into<String>>(code: S) -> Result<Self, String> {
        let code = code.into();
        if code.is_empty() {
            return Err("Error code cannot be empty".to_string());
        }
        Ok(ErrorCode { code })
    }

    /// Create a raw ErrorCode without error validation.
    pub fn raw<S: Into<String>>(code: S) -> Self {
        ErrorCode { code: code.into() }
    }

    /// Returns true if the code is a style error (starts with E, W, or D).
    pub fn is_style(&self) -> bool {
        self.code.starts_with('E') || self.code.starts_with('W') || self.code.starts_with('D')
    }
    pub fn is_logic(&self) -> bool {
        self.code.starts_with('F') || self.code.starts_with('I')
    }
    pub fn is_security(&self) -> bool {
        self.code.starts_with('B')
    }
    pub fn is_architecture(&self) -> bool {
        self.code.starts_with("AES")
    }
}

impl std::ops::Deref for ErrorCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Hash for ErrorCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::ErrorCode;

    #[test]
    fn test_error_code_new() {
        let ec = ErrorCode::new("E123").unwrap_or_default();
        assert_eq!(ec.code, "E123");
        assert!(ec.is_style());
        assert!(!ec.is_logic());
        assert!(!ec.is_security());
        assert!(!ec.is_architecture());

        let ec = ErrorCode::new("W999").unwrap_or_default();
        assert!(ec.is_style());

        let ec = ErrorCode::new("D404").unwrap_or_default();
        assert!(ec.is_style());

        let ec = ErrorCode::new("F001").unwrap_or_default();
        assert!(ec.is_logic());

        let ec = ErrorCode::new("I999").unwrap_or_default();
        assert!(ec.is_logic());

        let ec = ErrorCode::new("B001").unwrap_or_default();
        assert!(ec.is_security());

        let ec = ErrorCode::new("AES123").unwrap_or_default();
        assert!(ec.is_architecture());
    }

    #[test]
    fn test_error_code_invalid() {
        assert!(ErrorCode::new("").is_err());
    }
}
```

---

## File: crates/shared/src/common/taxonomy_job_vo.rs

```rust
// PURPOSE: PipelineJob, SuccessStatus, EnvContentVO, McpConfigVO — value objects for pipeline job lifecycle tracking
// ResponseData is re-exported from common for backward compatibility
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::string_value_object;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::common::taxonomy_response_data_vo::ResponseData;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum JobStatus {
    #[serde(rename = "pending")]
    PENDING,
    #[serde(rename = "running")]
    RUNNING,
    #[serde(rename = "completed")]
    COMPLETED,
    #[serde(rename = "failed")]
    FAILED,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::PENDING => write!(f, "pending"),
            JobStatus::RUNNING => write!(f, "running"),
            JobStatus::COMPLETED => write!(f, "completed"),
            JobStatus::FAILED => write!(f, "failed"),
        }
    }
}

// Manual impl: `SuccessStatus` overrides `Display` to render "SUCCESS"/"FAILURE"
// instead of `true`/`false`, and the macro does not currently support a clean
// `bool` cast (Rust forbids `i64 as bool`). Kept as a hand-rolled VO.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SuccessStatus {
    pub value: bool,
}

impl Default for SuccessStatus {
    fn default() -> Self {
        Self::new(false)
    }
}

impl SuccessStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for SuccessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.value {
            write!(f, "SUCCESS")
        } else {
            write!(f, "FAILURE")
        }
    }
}

impl std::ops::Deref for SuccessStatus {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.value
    }
}

/// `HashMap<String, serde_json::Value>` payload VOs. Wrapped via macro so they
/// pick up the standard `new`/`value`/`Default`/serde impls.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LintStatusActionArgs {
    #[serde(default)]
    pub value: HashMap<String, serde_json::Value>,
}

impl Default for LintStatusActionArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl LintStatusActionArgs {
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
        }
    }
    pub fn value(&self) -> &HashMap<String, serde_json::Value> {
        &self.value
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.get(key)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterMetadata {
    pub name: AdapterName,
    pub class_path: String,
    #[serde(default)]
    pub description: String,
}

impl AdapterMetadata {
    pub fn new(name: AdapterName, class_path: String) -> Self {
        Self {
            name,
            class_path,
            description: String::new(),
        }
    }
}

string_value_object!(EnvContentVO);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpConfigVO {
    #[serde(default)]
    pub value: HashMap<String, serde_json::Value>,
}

impl McpConfigVO {
    pub fn new(value: HashMap<String, serde_json::Value>) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &HashMap<String, serde_json::Value> {
        &self.value
    }
}
```

---

## File: crates/shared/src/common/taxonomy_lint_vo.rs

```rust
// PURPOSE: CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint — VOs for lint violations
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_source_vo::ContentString;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::source_parsing::taxonomy_path_vo::FilePath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeRef {
    pub name: DescriptionVO,
    #[serde(default)]
    pub kind: DescriptionVO,
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub start_line: Option<LineNumber>,
    #[serde(default)]
    pub end_line: Option<LineNumber>,
}

impl ScopeRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: DescriptionVO::new(name),
            kind: DescriptionVO::new("function"),
            file: None,
            start_line: None,
            end_line: None,
        }
    }
    pub fn has_range(&self) -> bool {
        self.start_line.as_ref().is_some_and(|l| l.value > 0)
            && self.end_line.as_ref().is_some_and(|l| l.value > 0)
    }
}

impl std::fmt::Display for ScopeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref file) = self.file {
            write!(f, "{} {} in {}", self.kind.value, self.name.value, file)
        } else if !self.kind.value.is_empty() {
            write!(f, "{} {}", self.kind.value, self.name.value)
        } else {
            write!(f, "{}", self.name.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub line: Option<LineNumber>,
    #[serde(default)]
    pub column: Option<ColumnNumber>,
    #[serde(default)]
    pub description: DescriptionVO,
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    pub fn new() -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            description: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if let Some(ref file) = self.file {
            parts.push(file.value.clone());
        }
        if let Some(ref line) = self.line {
            let mut s = line.value.to_string();
            if let Some(ref col) = self.column {
                if col.value > 0 {
                    s = format!("{}:{}", line.value, col.value);
                }
            }
            parts.push(s);
        }
        let result = if parts.is_empty() {
            "unknown".to_string()
        } else {
            parts.join(":")
        };
        if self.description.value.is_empty() {
            write!(f, "{}", result)
        } else {
            write!(f, "{} — {}", result, self.description.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationList {
    #[serde(default)]
    pub values: Vec<Location>,
}

impl LocationList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl LocationList {
    pub fn push(&mut self, item: Location) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for LocationList {
    type Target = Vec<Location>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViolationConstraint {
    pub rule: DescriptionVO,
    #[serde(default)]
    pub min_value: DescriptionVO,
    #[serde(default)]
    pub max_value: DescriptionVO,
}

impl ViolationConstraint {
    pub fn new(rule: impl Into<String>) -> Self {
        Self {
            rule: DescriptionVO::new(rule),
            min_value: DescriptionVO::new(String::new()),
            max_value: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for ViolationConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rule.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandArgs {
    #[serde(default)]
    pub args: Vec<ContentString>,
}

impl Default for CommandArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandArgs {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }
}

impl std::fmt::Display for CommandArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.args
                .iter()
                .map(|a| a.value.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeBounds {
    #[serde(default)]
    pub start: Option<LineNumber>,
    #[serde(default)]
    pub end: Option<LineNumber>,
}
```

---

## File: crates/shared/src/common/taxonomy_message_vo.rs

```rust
// PURPOSE: ComplianceStatus, LintMessage — VOs for compliance status and violation messages
use crate::string_value_object;

string_value_object!(LintMessage);

/// Boolean compliance flag. Written manually because `bool` is not supported
/// by the `string_value_object!` macro (`i64 as bool` is not a valid Rust cast).
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ComplianceStatus {
    pub value: bool,
}

impl ComplianceStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for ComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for ComplianceStatus {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_response_data_vo.rs

```rust
// PURPOSE: ResponseData — value object for pipeline job response data
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseData {
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub returncode: i64,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for ResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseData {
    pub fn new() -> Self {
        Self {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        }
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.value.as_ref()
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.as_ref().and_then(|v| v.get(key))
    }
}
```

---

## File: crates/shared/src/common/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — value object for violation severity levels (critical, high, medium, low)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    #[serde(rename = "info")]
    #[default]
    INFO,
    #[serde(rename = "low")]
    LOW,
    #[serde(rename = "medium")]
    MEDIUM,
    #[serde(rename = "high")]
    HIGH,
    #[serde(rename = "critical")]
    CRITICAL,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::INFO => write!(f, "info"),
            Severity::LOW => write!(f, "low"),
            Severity::MEDIUM => write!(f, "medium"),
            Severity::HIGH => write!(f, "high"),
            Severity::CRITICAL => write!(f, "critical"),
        }
    }
}

impl Severity {
    pub fn score_impact(&self) -> f64 {
        match self {
            Severity::INFO => 0.0,
            Severity::LOW => 1.0,
            Severity::MEDIUM => 2.0,
            Severity::HIGH => 3.0,
            Severity::CRITICAL => 5.0,
        }
    }
}
```

---

## File: crates/shared/src/external-lint/contract_external_lint_aggregate.rs

```rust
// PURPOSE: IExternalLintAggregate — contract for running external linter adapters
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IExternalLintAggregate: Send + Sync {
    async fn scan_all(&self, path: &FilePath) -> LintResultList;
    fn adapter_names(&self) -> Vec<String>;
}
```

---

## File: crates/shared/src/external-lint/mod.rs

```rust
// file-watch — taxonomy and contract types
pub mod contract_external_lint_aggregate;
```

---

## File: crates/shared/src/mcp-server/mod.rs

```rust
// mcp-server — taxonomy and contract types
// Re-export from common for backward compatibility
pub use crate::common::taxonomy_action_vo;
pub use crate::common::taxonomy_job_vo;
```

---

## File: crates/shared/src/source-parsing/contract_path_normalization_port.rs

```rust
// PURPOSE: IPathNormalizationPort — port trait for file path normalization across platforms
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IPathNormalizationPort: Send + Sync {
    fn normalize_path(&self, path: FilePath) -> FilePath;
    fn resolve_infrastructure_path(
        &self,
        path: FilePath,
        context_path: Option<FilePath>,
    ) -> FilePath;
}
```

---

## File: crates/shared/src/source-parsing/mod.rs

```rust
// source-parsing — taxonomy and contract types
pub mod contract_language_detector_port;
pub mod contract_parser_port;
pub mod contract_path_normalization_port;
pub mod contract_scanner_provider_port;
pub mod infrastructure_file_collector_provider;
pub mod taxonomy_adapter_error;
pub mod taxonomy_barrel_provider_vo;
pub mod taxonomy_file_collector_helper;
pub mod taxonomy_language_detector_helper;
pub mod taxonomy_naming_error;
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_semantic_error;
pub use infrastructure_file_collector_provider::{
    collect_all_source_files, count_loc, walk_rs_files, FileCollectorProvider,
};
```

---

## File: crates/shared/src/source-parsing/taxonomy_adapter_error.rs

```rust
// PURPOSE: AdapterError, ScanError, ValidationError — structured error types for adapter operations
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::Constraint;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_error::ExitCode;
use crate::common::taxonomy_common_error::FieldName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_source_vo::ContentString;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct AdapterError {
    pub adapter_name: AdapterName,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub command: Option<ContentString>,
    #[serde(default)]
    pub stderr: Option<ErrorMessage>,
    #[serde(default)]
    pub exit_code: Option<ExitCode>,
}

impl AdapterError {
    pub fn new(adapter_name: AdapterName, message: ErrorMessage) -> Self {
        Self {
            adapter_name,
            message,
            error_code: None,
            command: None,
            stderr: None,
            exit_code: None,
        }
    }
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self
            .error_code
            .as_ref()
            .map(|c| format!(" [{}]", c))
            .unwrap_or_default();
        write!(f, "[{}]{} {}", self.adapter_name, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ScanError {
    pub path: FilePath,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub adapter_name: Option<AdapterName>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl ScanError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            path,
            message,
            error_code: None,
            adapter_name: None,
            cause: None,
        }
    }
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let adapter = self
            .adapter_name
            .as_ref()
            .map(|a| format!(" ({})", a))
            .unwrap_or_default();
        let code = self
            .error_code
            .as_ref()
            .map(|c| format!(" [{}]", c))
            .unwrap_or_default();
        write!(
            f,
            "Scan failed{}{}: {} — {}",
            adapter, code, self.path, self.message
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ValidationError {
    pub field_name: FieldName,
    pub message: ErrorMessage,
    #[serde(default)]
    pub constraint: Option<Constraint>,
    #[serde(default)]
    pub value: Option<String>,
}

impl ValidationError {
    pub fn new(field_name: FieldName, message: ErrorMessage) -> Self {
        Self {
            field_name,
            message,
            constraint: None,
            value: None,
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Validation failed on '{}': {}",
            self.field_name, self.message
        )
    }
}
```

---

## File: crates/shared/src/source-parsing/taxonomy_path_vo.rs

```rust
// PURPOSE: FilePath, DirectoryPath — value objects for validated file and directory paths
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// file_path_vo — File and directory path value objects.
///
/// File path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilePath {
    pub value: String,
}

impl FilePath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new FilePath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and collapse multiple slashes.
        value = value.replace('\\', "/");
        // Remove all trailing slashes
        while value.ends_with('/') && value.len() > 1 {
            value.pop();
        }
        // If after normalization it's empty, then it was all slashes -> treat as root
        if value.is_empty() {
            return Ok(FilePath {
                value: "/".to_string(),
            });
        }
        Ok(FilePath { value })
    }

    /// File extension without dot.
    pub fn extension(&self) -> String {
        let special_files = [
            "Makefile",
            "Dockerfile",
            "Dockerfile.dev",
            "Dockerfile.prod",
            ".bashrc",
            ".profile",
            ".zshrc",
            ".gitignore",
            ".dockerignore",
        ];
        // Operate on the basename, not the full path — `./foo.rs` must still yield
        // `rs` as its extension, and `.bashrc` (which is fully a basename) must NOT
        // be confused with a hidden file mid-path.
        let basename = match self.value.rsplit('/').next() {
            Some(b) => b,
            None => return String::new(),
        };
        if special_files.contains(&basename) || basename.starts_with('.') {
            return String::new();
        }
        match basename.rsplit('.').next() {
            Some(ext) => ext.to_string(),
            None => String::new(),
        }
    }

    /// Check if path has given extension (without dot).
    pub fn has_extension(&self, ext: &str) -> bool {
        self.extension().eq_ignore_ascii_case(ext)
    }

    /// Extract filename/basename of the path.
    pub fn basename(&self) -> String {
        match self.value.rsplit('/').next() {
            Some(f) => f.to_string(),
            None => self.value.clone(),
        }
    }

    /// Check if the path is a barrel file.
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "main.py" | "py.typed" | "app.py" | "lib.rs"
        )
    }
}

impl std::ops::Deref for FilePath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for FilePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Directory path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct DirectoryPath {
    pub value: String,
}

impl DirectoryPath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new DirectoryPath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("Directory path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and remove trailing slash.
        value = value.replace('\\', "/");
        // Remove trailing slash unless it's just "/"
        if value.ends_with('/') && value.len() > 1 {
            value.pop();
        }
        Ok(DirectoryPath { value })
    }
}

impl std::ops::Deref for DirectoryPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for DirectoryPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for DirectoryPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DirectoryPath::new(s).map_err(serde::de::Error::custom)
    }
}

impl Hash for DirectoryPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::{DirectoryPath, FilePath};

    #[test]
    fn test_file_path_new() {
        let fp = FilePath::new("test.txt").unwrap_or_default();
        assert_eq!(fp.value, "test.txt");
        assert_eq!(fp.extension(), "txt");
        assert!(fp.has_extension("txt"));
        assert!(!fp.has_extension("md"));

        // Test normalization
        let fp = FilePath::new("path\\to\\file.txt").unwrap_or_default();
        assert_eq!(fp.value, "path/to/file.txt");

        let fp = FilePath::new("path/to/file/").unwrap_or_default();
        assert_eq!(fp.value, "path/to/file");

        let fp = FilePath::new("/").unwrap_or_default();
        assert_eq!(fp.value, "/");

        let fp = FilePath::new("///").unwrap_or_default();
        assert_eq!(fp.value, "/");
    }

    #[test]
    fn test_file_path_invalid() {
        assert!(FilePath::new("").is_err());
        assert!(FilePath::new("   ").is_err());
    }

    #[test]
    fn test_directory_path_new() {
        let dp = DirectoryPath::new("test/dir").unwrap_or_default();
        assert_eq!(dp.value, "test/dir");

        let dp = DirectoryPath::new("test/dir/").unwrap_or_default();
        assert_eq!(dp.value, "test/dir");

        let dp = DirectoryPath::new("/").unwrap_or_default();
        assert_eq!(dp.value, "/");
    }

    #[test]
    fn test_directory_path_invalid() {
        assert!(DirectoryPath::new("").is_err());
        assert!(DirectoryPath::new("   ").is_err());
    }

    /// Regression: `./foo.rs` must report `rs` as its extension, not empty string.
    /// The old implementation treated any path starting with `.` as having no
    /// extension, which caused `LanguageDetector::is_lintable` to skip relative
    /// paths emitted by `std::fs::read_dir` in `collect_source_files`. Result: zero
    /// files collected when the user runs `lint-arwaky check .` on a directory
    /// tree with non-`.git`-anchored paths.
    #[test]
    fn test_extension_with_dot_slash_prefix() {
        let fp = FilePath::new("./foo.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
        let fp = FilePath::new("./nested/foo.py").unwrap_or_default();
        assert_eq!(fp.extension(), "py");
        let fp = FilePath::new(".//foo.ts").unwrap_or_default();
        assert_eq!(fp.extension(), "ts");
    }

    /// Regression: a hidden-file basename (e.g. `.bashrc`) must still report no
    /// extension, since the basename itself starts with a dot.
    #[test]
    fn test_extension_hidden_basename() {
        let fp = FilePath::new(".bashrc").unwrap_or_default();
        assert_eq!(fp.extension(), "");
        let fp = FilePath::new("/home/user/.gitignore").unwrap_or_default();
        assert_eq!(fp.extension(), "");
    }

    /// Regression: full paths must still resolve the extension on the basename.
    #[test]
    fn test_extension_full_path() {
        let fp =
            FilePath::new("/tmp/bypass_test/capabilities_unwrap_checker.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
        let fp = FilePath::new("crates/code-analysis/src/foo.rs").unwrap_or_default();
        assert_eq!(fp.extension(), "rs");
    }

    /// Makefile / Dockerfile — special filenames, no extension.
    #[test]
    fn test_extension_special_filenames() {
        let fp = FilePath::new("Makefile").unwrap_or_default();
        assert_eq!(fp.extension(), "");
        let fp = FilePath::new("Dockerfile").unwrap_or_default();
        assert_eq!(fp.extension(), "");
    }
}
```

---
