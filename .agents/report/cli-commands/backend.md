
Hi Raka,

Below is a backend-focused review of the uploaded `cli-commands_v1.10.106.md` knowledge base. I treated the uploaded content as the source of truth. Some referenced crates (`import_rules`, `naming_rules`, `external_lint`, etc.) are not fully included, so my review focuses on what is visible in `cli-commands` and `shared`.

## Executive Summary

The codebase has a strong architectural intention: layered boundaries, taxonomy/contract separation, DI container wiring, and multi-language lint orchestration. However, the current implementation has several important backend issues:

1. **Architectural leakage**: `CheckCommandsSurface` contains orchestration, filtering, reporting, scoring-related decisions, and direct utility usage. This violates the stated AES layer rules and SOLID/SRP.
2. **Security concerns**:
   - `init --global` copies docs from the current project into a trusted global config directory.
   - MCP binary resolution can fall back to `PATH`, creating binary hijacking risk.
   - Path filtering uses string prefix matching, which can include unintended paths.
   - External linter execution should harden against argument injection and timeouts.
3. **Error handling weaknesses**:
   - Many failures are swallowed with `unwrap_or_default()`.
   - Maintenance/security/dependency commands can return success even when tools are missing or errors occur.
   - Invalid paths are sometimes converted into defaults instead of failing explicitly.
4. **Performance/scalability bottlenecks**:
   - Orphan detection uses raw file collection without ignore rules.
   - Multiple linters likely re-walk the filesystem independently.
   - Cross-platform symlink/directory cycle handling is buggy on non-Unix platforms.
   - Full result sets are held in memory and printed at once.
5. **Business logic correctness issues**:
   - CI mode only runs code-analysis, not the full pipeline.
   - Fix command measures improvement using only code-analysis results.
   - `--git-diff` mode ignores filter/format and does not run the full pipeline.
   - Workspace member matching uses substring matching, which can select unintended members.
6. **No database layer is present** in the uploaded code. If persistence is added later, use parameterized queries, migrations, pooling, and transactional writes.

---

# 1. Scope Note: Database Queries

The uploaded code does not contain a database layer, SQL queries, ORM usage, or persistence logic. Therefore, there are no direct SQL injection or database performance issues to review.

However, if you later persist lint reports, fix history, or project metadata, use the following baseline:

```rust
// Preferred: sqlx with compile-time checked queries
sqlx::query!(
    r#"
    INSERT INTO lint_violations (
        project_id,
        file_path,
        line,
        column,
        code,
        severity,
        message,
        created_at
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
    "#,
    project_id,
    file_path,
    line,
    column,
    code,
    severity,
    message
)
.execute(&pool)
.await?;
```

Avoid:

```rust
// Dangerous: string-built SQL
let query = format!(
    "INSERT INTO lint_violations VALUES ('{}', '{}')",
    project_id, file_path
);
```

Recommended database practices:

- Use `sqlx` or `diesel` with parameterized queries.
- Add migrations via `sqlx migrate` or `diesel migration`.
- Use a connection pool.
- Add indexes for common query paths:
  - `project_id`
  - `code`
  - `severity`
  - `file_path`
  - `created_at`
- Batch inserts for large lint reports.
- Use transactions when applying fix metadata.
- Paginate API responses.
- Store normalized paths and project identifiers rather than arbitrary user strings where possible.

---

# 2. Prioritized Findings

| Severity | Area                   | Issue                                                                         | Impact                                                            |
| -------- | ---------------------- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| Critical | Security               | `init --global` copies docs from current project into trusted global config | Untrusted project can plant malicious docs into global config     |
| Critical | Security               | MCP binary resolution falls back to bare`lint-arwaky-mcp` on PATH           | PATH hijacking / binary planting risk                             |
| High     | Security / Correctness | Path filtering uses string prefix matching                                    | Files outside intended path may be included                       |
| High     | Performance            | Orphan detection uses raw file collection without ignore rules                | Scans`node_modules`, `target`, generated files; DoS/perf risk |
| High     | Error Handling         | Audit failures are swallowed with`unwrap_or_default()`                      | Silent incorrect lint reports                                     |
| High     | Architecture           | Surface layer contains orchestration, formatting, filtering                   | Violates AES layer rules and SRP                                  |
| High     | Business Logic         | CI mode only runs code-analysis                                               | CI score is incomplete                                            |
| High     | Business Logic         | Fix command counts only code-analysis violations                              | Misleading fix results                                            |
| High     | Performance            | File walker visited-set uses inode abstraction that breaks on non-Unix        | Potential traversal bugs on Windows                               |
| Medium   | Security               | External linters should use timeouts and argument separators                  | Hung processes / argument injection risk                          |
| Medium   | API Design             | Command catalog is duplicated and inconsistent                                | CLI/MCP command mismatch                                          |
| Medium   | Error Handling         | Maintenance commands return success on missing tools/errors                   | CI cannot trust exit codes                                        |
| Medium   | Performance            | Repeated filesystem walks across linters                                      | Slow monorepo scans                                               |
| Medium   | Maintainability        | Closure-based factories are opaque                                            | Harder to test/debug                                              |
| Low      | Reporting              | SARIF/JUnit output can be improved                                            | Better CI integration                                             |
| Low      | API Design             | Global CLI flags are unused or inconsistently applied                         | Confusing UX                                                      |

---

# 3. Architectural Issues

## 3.1 Surface Layer Is Doing Orchestration Work

`CheckCommandsSurface` currently:

- Runs code-analysis.
- Runs naming, import, external, and role audits concurrently.
- Runs orphan detection.
- Filters results by path and rule code.
- Formats text/JSON/SARIF/JUnit output.
- Prints multi-workspace summaries.
- Computes violation counts and exit-code decisions.

This is too much responsibility for a surface layer.

Your `ARCHITECTURE.md` says:

> Surfaces must not contain business calculation or orchestration.

But `CheckCommandsSurface::scan()` and `scan_with_discovery()` are orchestration pipelines.

### Recommendation

Introduce a dedicated agent/contract aggregate for the full lint pipeline.

```rust
// crates/shared/src/cli-commands/contract_analysis_pipeline_aggregate.rs

use async_trait::async_trait;
use thiserror::Error;

use crate::cli_commands::taxonomy_format_vo::Format;
use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_threshold_vo::Threshold;

#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("path '{0}' does not exist")]
    PathNotFound(String),

    #[error("invalid path '{0}'")]
    InvalidPath(String),

    #[error("workspace discovery failed: {0}")]
    WorkspaceDiscovery(String),

    #[error("analysis failed: {0}")]
    Analysis(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub enum ScanMode {
    Check,
    Scan,
    Ci { threshold: Threshold },
}

#[derive(Debug, Clone)]
pub struct ScanRequest {
    pub target: FilePath,
    pub mode: ScanMode,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
}

#[derive(Debug, Clone)]
pub struct PipelineDiagnostic {
    pub source: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Default)]
pub struct ScanReport {
    pub results: LintResultList,
    pub diagnostics: Vec<PipelineDiagnostic>,
    pub score: Score,
}

impl ScanReport {
    pub fn violation_count(&self) -> usize {
        self.results.len()
    }

    pub fn has_critical(&self) -> bool {
        self.results
            .values
            .iter()
            .any(|r| r.severity == Severity::CRITICAL)
    }
}

#[async_trait]
pub trait IAnalysisPipelineAggregate: Send + Sync {
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError>;
}
```

Then the CLI surface becomes thin:

```rust
// crates/cli-commands/src/surface_scan_command.rs

use std::process::ExitCode;
use std::sync::Arc;

use shared::cli_commands::contract_analysis_pipeline_aggregate::{
    IAnalysisPipelineAggregate, ScanMode, ScanRequest,
};
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::common::taxonomy_path_vo::FilePath;

pub async fn handle_scan(
    pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    path: Option<String>,
    member: Option<String>,
    filter: Option<String>,
    format: Format,
) -> ExitCode {
    let raw_path = path.unwrap_or_else(|| ".".to_string());

    let target = match FilePath::new(raw_path.clone()) {
        Ok(target) => target,
        Err(err) => {
            eprintln!("Error: invalid path '{raw_path}': {err}");
            return ExitCode::from(2);
        }
    };

    if !std::path::Path::new(target.value()).exists() {
        eprintln!("Error: path '{}' does not exist", target.value());
        return ExitCode::from(2);
    }

    let request = ScanRequest {
        target,
        mode: ScanMode::Scan,
        filter,
        member,
        format,
    };

    match pipeline.run(request).await {
        Ok(report) => {
            // Rendering should be delegated to a ReportFormatter capability.
            // For illustration:
            println!("{}", render_report(&report, format));

            if report.violation_count() > 0 {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::from(2)
        }
    }
}

fn render_report(
    _report: &shared::cli_commands::contract_analysis_pipeline_aggregate::ScanReport,
    _format: Format,
) -> String {
    // Delegate to formatter capability:
    // - TextReportFormatter
    // - JsonReportFormatter
    // - SarifReportFormatter
    // - JunitReportFormatter
    todo!()
}
```

This gives you:

- Better SRP.
- Easier testing.
- Reusable pipeline for CLI, MCP, TUI, CI, and watch mode.
- Cleaner adherence to your own AES architecture.

---

## 3.2 Reporting Logic Should Be a Separate Capability

Currently, SARIF and JUnit formatting are embedded inside `CheckCommandsSurface`.

This violates Open/Closed Principle. If you add GitLab Code Quality, CodeClimate, HTML, or Markdown reports, you have to modify the surface.

### Recommended Design

```rust
// crates/shared/src/cli-commands/contract_report_formatter_protocol.rs

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;

pub struct ReportContext {
    pub target: FilePath,
    pub tool_name: String,
    pub tool_version: String,
}

pub trait ReportFormatter: Send + Sync {
    fn format(&self, results: &[LintResult], context: &ReportContext) -> String;
}
```

Example JSON formatter:

```rust
pub struct JsonReportFormatter;

impl ReportFormatter for JsonReportFormatter {
    fn format(&self, results: &[LintResult], _context: &ReportContext) -> String {
        serde_json::to_string_pretty(results).unwrap_or_else(|_| "[]".to_string())
    }
}
```

Example SARIF formatter:

```rust
pub struct SarifReportFormatter;

impl ReportFormatter for SarifReportFormatter {
    fn format(&self, results: &[LintResult], context: &ReportContext) -> String {
        // Move existing format_sarif_output() here.
        // Also add:
        // - rule metadata
        // - columns
        // - related locations
        // - URI-encoded artifact locations
        todo!()
    }
}
```

Then use a formatter registry:

```rust
pub fn formatter_for(format: Format) -> Box<dyn ReportFormatter> {
    match format {
        Format::Text => Box::new(TextReportFormatter),
        Format::Json => Box::new(JsonReportFormatter),
        Format::Sarif => Box::new(SarifReportFormatter),
        Format::Junit => Box::new(JunitReportFormatter),
    }
}
```

---

## 3.3 Command Catalog Is Duplicated and Inconsistent

You have at least two command metadata sources:

1. `COMMAND_CATALOG` in `taxonomy_catalog_constant.rs`
2. `CommandCatalogVO::command_catalog()` in `taxonomy_command_catalog_vo.rs`

They are not fully consistent.

For example:

- `COMMAND_CATALOG` includes `orphan`, `init`, `install`, `mcp-config`, `config-show`.
- `CommandCatalogVO` includes `plugins`, which is not present in the CLI enum.
- `CommandCatalogVO` is missing several commands present in the CLI.
- Examples use `lint-arwaky-cli`, while the clap command name is `lint-arwaky`.

This can cause MCP clients or help systems to expose incorrect capabilities.

### Fix: Single Source of Truth

```rust
// crates/shared/src/cli-commands/taxonomy_catalog_constant.rs

pub struct CommandSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub example: &'static str,
}

pub const BINARY_NAME: &str = "lint-arwaky";

pub static COMMAND_CATALOG: &[CommandSpec] = &[
    CommandSpec {
        name: "check",
        description: "Run full architecture compliance analysis",
        example: "lint-arwaky check /path",
    },
    CommandSpec {
        name: "scan",
        description: "Deep directory scan with workspace discovery",
        example: "lint-arwaky scan ./src/",
    },
    CommandSpec {
        name: "fix",
        description: "Apply safe automatic fixes",
        example: "lint-arwaky fix file.py",
    },
    CommandSpec {
        name: "ci",
        description: "CI mode with threshold and exit codes",
        example: "lint-arwaky ci /path --threshold 80",
    },
    CommandSpec {
        name: "doctor",
        description: "Diagnose environment health",
        example: "lint-arwaky doctor",
    },
    CommandSpec {
        name: "orphan",
        description: "Check if a file is dead/unreachable code",
        example: "lint-arwaky orphan <path>",
    },
    CommandSpec {
        name: "security",
        description: "Scan for security vulnerabilities",
        example: "lint-arwaky security /path",
    },
    CommandSpec {
        name: "dependencies",
        description: "Scan dependencies for known vulnerabilities",
        example: "lint-arwaky dependencies .",
    },
    CommandSpec {
        name: "watch",
        description: "Watch files and lint on change",
        example: "lint-arwaky watch ./src/",
    },
    CommandSpec {
        name: "init",
        description: "Create default config",
        example: "lint-arwaky init",
    },
    CommandSpec {
        name: "install",
        description: "Install adapter dependencies",
        example: "lint-arwaky install",
    },
    CommandSpec {
        name: "mcp-config",
        description: "Print MCP server config",
        example: "lint-arwaky mcp-config --client claude",
    },
    CommandSpec {
        name: "config-show",
        description: "Show active configuration",
        example: "lint-arwaky config-show",
    },
];
```

Then generate the catalog VO from that constant:

```rust
// crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs

use std::collections::HashMap;

use crate::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
use crate::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;
use crate::common::taxonomy_suggestion_vo::{DescriptionVO, Suggestion};
use crate::mcp_server::taxonomy_action_vo::ActionName;

pub struct CommandCatalogVO;

impl CommandCatalogVO {
    pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
        COMMAND_CATALOG
            .iter()
            .map(|spec| {
                (
                    ActionName::from(spec.name),
                    CommandMetadataVO::new(
                        DescriptionVO::new(spec.description),
                        Suggestion::new(spec.example),
                    ),
                )
            })
            .collect()
    }
}
```

---

# 4. Security Review

## 4.1 Critical: Global Init Copies Untrusted Project Docs Into Trusted Config

In `surface_setup_command.rs`, `handle_init_global()` copies docs from the current project root into the global config directory:

```rust
let doc_files = [
    "SKILL.md",
    "ARCHITECTURE.md",
    "MIGRATION_RUST.md",
    "MIGRATION_PYTHON.md",
    "MIGRATION_TYPESCRIPT.md",
    "RULES_AES.md",
];

for doc in &doc_files {
    match std::fs::read_to_string(doc) {
        Ok(content) => {
            // writes into global config
        }
        Err(_) => println!("  {doc} — not found in project root, skipping"),
    }
}
```

This is risky.

If a user runs `lint-arwaky init --global` inside an untrusted repository, that repository can supply malicious `SKILL.md`, `ARCHITECTURE.md`, etc. Those files are then placed into a trusted global config directory and may later be distributed into other projects by `lint-arwaky init`.

This is a supply-chain/persistence risk.

### Fix

Only install trusted, bundled docs.

Use `include_str!` at compile time or install from a trusted asset directory verified by checksum.

```rust
// crates/cli-commands/src/surface_setup_command.rs

struct TrustedDoc {
    name: &'static str,
    content: &'static str,
}

const TRUSTED_DOCS: &[TrustedDoc] = &[
    TrustedDoc {
        name: "SKILL.md",
        content: include_str!("../assets/SKILL.md"),
    },
    TrustedDoc {
        name: "ARCHITECTURE.md",
        content: include_str!("../assets/ARCHITECTURE.md"),
    },
    TrustedDoc {
        name: "MIGRATION_RUST.md",
        content: include_str!("../assets/MIGRATION_RUST.md"),
    },
    TrustedDoc {
        name: "MIGRATION_PYTHON.md",
        content: include_str!("../assets/MIGRATION_PYTHON.md"),
    },
    TrustedDoc {
        name: "MIGRATION_TYPESCRIPT.md",
        content: include_str!("../assets/MIGRATION_TYPESCRIPT.md"),
    },
    TrustedDoc {
        name: "RULES_AES.md",
        content: include_str!("../assets/RULES_AES.md"),
    },
];

fn write_trusted_docs(config_dir: &std::path::Path) -> bool {
    let mut all_ok = true;

    for doc in TRUSTED_DOCS {
        let target = config_dir.join(doc.name);
        let target_str = target.to_string_lossy();

        if target.exists() {
            println!("  {} — already exists, skipping", doc.name);
            continue;
        }

        match std::fs::write(&target, doc.content) {
            Ok(_) => println!("  {} — created", doc.name),
            Err(err) => {
                println!("  {} — error: {err}", doc.name);
                all_ok = false;
            }
        }

        let _ = target_str; // if needed by file_exists abstraction
    }

    all_ok
}
```

Do not copy docs from the current working project into global config unless the user explicitly asks and the source is verified.

---

## 4.2 Critical: MCP Binary Resolution Should Not Fall Back Blindly to PATH

Current code:

```rust
fn which_mcp_binary() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-mcp");
            if sibling.exists() {
                return sibling.to_string_lossy().to_string();
            }
        }
    }

    "lint-arwaky-mcp".to_string()
}
```

Returning a bare executable name means the MCP client may resolve it from `PATH`.

If an attacker can place `lint-arwaky-mcp` earlier in the user's `PATH`, they can execute arbitrary code when the MCP client launches the server.

### Fix

Fail closed if the binary cannot be resolved to an absolute, canonicalized path.

```rust
use std::path::PathBuf;

fn resolve_mcp_binary() -> Result<PathBuf, String> {
    // 1. Explicit override.
    if let Ok(explicit) = std::env::var("LINT_ARWAKY_MCP_BIN") {
        let path = PathBuf::from(explicit);

        if !path.is_file() {
            return Err(format!(
                "LINT_ARWAKY_MCP_BIN points to a non-file path: {}",
                path.display()
            ));
        }

        return path
            .canonicalize()
            .map_err(|err| format!("cannot canonicalize MCP binary: {err}"));
    }

    // 2. Sibling of current executable.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-mcp");

            if sibling.is_file() {
                return sibling
                    .canonicalize()
                    .map_err(|err| format!("cannot canonicalize MCP binary: {err}"));
            }
        }
    }

    // 3. Do not fall back to bare PATH lookup by default.
    Err(
        "lint-arwaky-mcp binary not found. Set LINT_ARWAKY_MCP_BIN to an absolute path."
            .to_string(),
    )
}
```

Then:

```rust
pub fn handle_mcp_config(client: &str) -> ExitCode {
    let binary = match resolve_mcp_binary() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error: {err}");
            return ExitCode::from(2);
        }
    };

    let binary_str = binary.to_string_lossy().to_string();

    let config = serde_json::json!({
        "mcpServers": {
            "lint-arwaky": {
                "command": binary_str,
                "args": [],
                "env": {}
            }
        }
    });

    println!("MCP Client Configuration for: {client}");
    println!("Binary: {}", binary.display());
    println!();
    println!("{}", serde_json::to_string_pretty(&config).unwrap_or_default());

    ExitCode::SUCCESS
}
```

If you must support PATH lookup, use a vetted `which` crate and still canonicalize the result. But fail closed is safer.

---

## 4.3 High: Path Filtering Uses String Prefix Matching

In `filter_and_display_results()`:

```rust
let canonical_scan_path = crate::surface_common_command::canonicalize_path(path);

let abs_path = cwd.join(&r.file.value);

r.code.code() == code
    && abs_path.to_string_lossy().starts_with(&canonical_scan_path)
```

String prefix matching is unsafe for paths.

Example:

```text
canonical_scan_path = "/project/src"
abs_path = "/project/src_secret.rs"
```

`"/project/src_secret.rs".starts_with("/project/src")` is true, even though the file is outside `/project/src`.

### Fix

Use `Path::starts_with`, not string prefix matching.

```rust
use std::path::{Path, PathBuf};

fn canonicalize_or_keep(path: &Path) -> PathBuf {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
}

fn is_within_path(base: &Path, candidate: &Path) -> bool {
    let base = canonicalize_or_keep(base);
    let candidate = canonicalize_or_keep(candidate);

    candidate.starts_with(base)
}
```

Use it in filtering:

```rust
let cwd = crate::surface_common_command::current_dir();
let base_path = std::path::Path::new(path);

let filtered_results: Vec<_> = all_results
    .into_iter()
    .filter(|result| {
        let abs_path = cwd.join(&result.file.value);

        let matches_code = match &filter {
            Some(code) => result.code.code() == code.as_str(),
            None => true,
        };

        matches_code && is_within_path(base_path, &abs_path)
    })
    .collect();
```

Better still, canonicalize the base path once:

```rust
pub struct PathScope {
    root: PathBuf,
}

impl PathScope {
    pub fn new(root: &Path) -> std::io::Result<Self> {
        Ok(Self {
            root: root.canonicalize()?,
        })
    }

    pub fn contains(&self, candidate: &Path) -> bool {
        let candidate = candidate
            .canonicalize()
            .unwrap_or_else(|_| candidate.to_path_buf());

        candidate.starts_with(&self.root)
    }
}
```

Usage:

```rust
let scope = match PathScope::new(std::path::Path::new(path)) {
    Ok(scope) => scope,
    Err(err) => {
        eprintln!("Error: cannot resolve path '{path}': {err}");
        return 0;
    }
};

let filtered_results: Vec<_> = all_results
    .into_iter()
    .filter(|result| {
        let abs_path = cwd.join(&result.file.value);
        scope.contains(&abs_path)
    })
    .collect();
```

---

## 4.4 Medium: External Linter Execution Should Use Timeouts and Argument Separators

The actual external lint execution is not fully visible, but `IExternalLintAggregate` implies subprocess-based linters.

For subprocess execution, ensure:

1. Never use a shell.
2. Use argument lists.
3. Use `--` before paths where supported.
4. Apply timeouts.
5. Capture stdout/stderr size limits.
6. Do not pass untrusted paths as options.

### Rust Example

```rust
use std::process::Command;
use std::time::Duration;

fn run_ruff(path: &str) -> Result<String, String> {
    let output = Command::new("ruff")
        .arg("check")
        .arg("--output-format")
        .arg("json")
        .arg("--")
        .arg(path)
        .output()
        .map_err(|err| format!("failed to start ruff: {err}"))?;

    if !output.status.success() {
        // Many linters return non-zero when violations exist.
        // Distinguish between "violations found" and "tool failure".
    }

    String::from_utf8(output.stdout)
        .map_err(|err| format!("invalid ruff output: {err}"))
}
```

For production, wrap with `tokio::process::Command` and timeout:

```rust
use tokio::process::Command as TokioCommand;
use tokio::time::{timeout, Duration};

async fn run_ruff_async(path: &str) -> Result<String, String> {
    let future = TokioCommand::new("ruff")
        .arg("check")
        .arg("--output-format")
        .arg("json")
        .arg("--")
        .arg(path)
        .output();

    match timeout(Duration::from_secs(120), future).await {
        Ok(Ok(output)) => {
            String::from_utf8(output.stdout)
                .map_err(|err| format!("invalid ruff output: {err}"))
        }
        Ok(Err(err)) => Err(format!("failed to start ruff: {err}")),
        Err(_) => Err("ruff timed out after 120 seconds".to_string()),
    }
}
```

### Python Adapter Example

```python
import subprocess

def run_ruff(path: str) -> str:
    try:
        result = subprocess.run(
            ["ruff", "check", "--output-format", "json", "--", path],
            capture_output=True,
            text=True,
            timeout=120,
            check=False,
        )
    except subprocess.TimeoutExpired as exc:
        raise RuntimeError("ruff timed out") from exc

    return result.stdout
```

Avoid:

```python
# Dangerous
subprocess.run(f"ruff {path}", shell=True)
```

### TypeScript Adapter Example

```ts
import { execFile } from "node:child_process";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

export async function runEslint(path: string): Promise<string> {
  const { stdout } = await execFileAsync(
    "eslint",
    ["--format", "json", "--", path],
    {
      timeout: 120_000,
      maxBuffer: 10 * 1024 * 1024,
    },
  );

  return stdout;
}
```

Avoid:

```ts
// Dangerous
await exec(`eslint ${path}`);
```

---

## 4.5 Medium: `config-show` May Leak Secrets

`handle_config_show()` prints raw config content:

```rust
println!("{}", source.raw_content);
```

If config files contain tokens, API keys, registry credentials, or environment secrets, this command may leak them into logs, CI output, or terminal scrollback.

### Recommendation

Add redaction for known secret fields before printing.

```rust
fn redact_config(raw: &str) -> String {
    // Simple line-based redaction for YAML-like config.
    raw.lines()
        .map(|line| {
            let lower = line.to_lowercase();

            if lower.contains("token")
                || lower.contains("secret")
                || lower.contains("password")
                || lower.contains("api_key")
                || lower.contains("apikey")
                || lower.contains("authorization")
            {
                if let Some((key, _)) = line.split_once(':') {
                    return format!("{key}: [REDACTED]");
                }
            }

            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
```

Then:

```rust
println!("{}", redact_config(&source.raw_content));
```

For structured configs, parse and redact fields explicitly.

---

# 5. Error Handling Review

## 5.1 High: Audit Failures Are Swallowed

In `CheckCommandsSurface::scan()`:

```rust
all_results.extend(naming_results.unwrap_or_default());
all_results.extend(import_results.unwrap_or_default());
```

If naming or import auditing fails, the CLI reports zero violations for that subsystem.

That is dangerous for CI. A failed linter should not look like a clean project.

### Fix

Introduce diagnostics and propagate subsystem failures.

```rust
use shared::cli_commands::contract_analysis_pipeline_aggregate::PipelineDiagnostic;
use shared::common::taxonomy_severity_vo::Severity;

fn merge_audit_result(
    all_results: &mut Vec<LintResult>,
    diagnostics: &mut Vec<PipelineDiagnostic>,
    source: &str,
    result: Result<Vec<LintResult>, shared::common::taxonomy_adapter_error::ScanError>,
) {
    match result {
        Ok(mut values) => all_results.append(&mut values),
        Err(err) => diagnostics.push(PipelineDiagnostic {
            source: source.to_string(),
            message: err.to_string(),
            severity: Severity::HIGH,
        }),
    }
}
```

Usage:

```rust
let mut diagnostics = Vec::new();

merge_audit_result(
    &mut all_results,
    &mut diagnostics,
    "naming-rules",
    naming_results,
);

merge_audit_result(
    &mut all_results,
    &mut diagnostics,
    "import-rules",
    import_results,
);
```

Then decide exit behavior:

```rust
let has_diagnostic_failure = diagnostics
    .iter()
    .any(|d| d.severity == Severity::HIGH || d.severity == Severity::CRITICAL);

if has_diagnostic_failure {
    for diagnostic in &diagnostics {
        eprintln!(
            "[{}] {}",
            diagnostic.severity,
            diagnostic.message
        );
    }

    return ExitCode::from(2);
}
```

---

## 5.2 High: External Lint Aggregate Should Return Errors

Current contract:

```rust
#[async_trait]
pub trait IExternalLintAggregate: Send + Sync {
    async fn scan_all(&self, path: &FilePath) -> LintResultList;
    fn adapter_names(&self) -> Vec<String>;
}
```

There is no error channel.

If an adapter crashes, times out, or is missing, the implementation is forced to swallow the error or panic.

### Fix

Return a richer result:

```rust
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_adapter_error::ScanError;

pub struct ExternalLintOutput {
    pub results: LintResultList,
    pub errors: Vec<ScanError>,
}

#[async_trait]
pub trait IExternalLintAggregate: Send + Sync {
    async fn scan_all(&self, path: &FilePath) -> ExternalLintOutput;

    fn adapter_names(&self) -> Vec<String>;
}
```

Or:

```rust
#[async_trait]
pub trait IExternalLintAggregate: Send + Sync {
    async fn scan_all(
        &self,
        path: &FilePath,
    ) -> Result<LintResultList, Vec<ScanError>>;

    fn adapter_names(&self) -> Vec<String>;
}
```

---

## 5.3 Medium: Maintenance Commands Should Not Return Success on Failure

Current `handle_security()` returns success when the tool is missing:

```rust
if !report.tool_installed {
    println!("{} not available. Please install it.", report.tool_name);
    return ExitCode::SUCCESS;
}
```

Current `handle_dependencies()` returns success even on error:

```rust
Err(e) => {
    println!("{e}");
}
```

Then falls through to `ExitCode::SUCCESS`.

This makes CI unreliable.

### Fix

Use distinct exit codes:

```text
0 = success, no violations
1 = violations/findings found
2 = operational error
3 = required tool missing
```

Example:

```rust
pub async fn handle_security(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let fp = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };

    println!("Security Vulnerability Scan — {}", fp.value);
    println!();

    let report = maintenance_orchestrator.run_security_scan(&fp).await;

    println!("Language: {}", report.language);
    println!("Tool: {}", report.tool_name);

    if !report.tool_installed {
        eprintln!(
            "Error: {} is not installed. Please install it.",
            report.tool_name
        );
        return ExitCode::from(3);
    }

    println!("Findings: {}", report.findings.len());

    for finding in &report.findings {
        println!(
            "  {} {} {}:{} {}",
            finding.severity.to_uppercase(),
            finding.test_id,
            finding.file,
            finding.line,
            finding.issue
        );
    }

    if !report.findings.is_empty() {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
```

Dependencies:

```rust
pub async fn handle_dependencies(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let fp = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };

    println!("Dependency Report — {}", fp.value);
    println!();

    match maintenance_orchestrator.run_dependency_report(&fp).await {
        Ok(report) => {
            println!("Language: {}", report.language);
            println!("Dependencies: {} total", report.dependencies.len());
            println!();
            println!("{:<25} {:<12} Type", "Package", "Version");

            for dep in report.dependencies.iter().take(30) {
                println!("{:<25} {:<12} {}", dep.name, dep.version, dep.dep_type);
            }

            if report.dependencies.len() > 30 {
                println!("... and {} more", report.dependencies.len() - 30);
            }

            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::from(2)
        }
    }
}
```

Doctor should also support a strict mode:

```rust
pub async fn handle_doctor(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    strict: bool,
) -> ExitCode {
    let diag = maintenance_orchestrator.diagnose_toolchain().await;

    // print diagnostics...

    let has_failure = diag
        .rust_tools
        .iter()
        .chain(diag.python_tools.iter())
        .chain(diag.js_tools.iter())
        .chain(diag.vcs_tools.iter())
        .any(|tool| tool.status != "OK");

    if strict && has_failure {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
```

---

## 5.4 Medium: Invalid Paths Should Not Become Defaults

There are several places where invalid paths become defaults:

```rust
FilePath::new(path).unwrap_or_default()
```

For CLI commands, invalid user input should usually fail with a clear error.

### Fix

Use a helper:

```rust
fn resolve_target_path(path: Option<String>) -> Result<FilePath, String> {
    let raw = path.unwrap_or_else(|| ".".to_string());

    let file_path = FilePath::new(raw.clone())
        .map_err(|err| format!("invalid path '{raw}': {err}"))?;

    if !std::path::Path::new(file_path.value()).exists() {
        return Err(format!("path '{}' does not exist", file_path.value()));
    }

    Ok(file_path)
}
```

Usage:

```rust
let target = match resolve_target_path(path) {
    Ok(target) => target,
    Err(err) => {
        eprintln!("Error: {err}");
        return ExitCode::from(2);
    }
};
```

---

# 6. Performance and Scalability Review

## 6

Hi Raka,

Below is a backend-focused review of the uploaded `cli-commands_v1.10.106.md` knowledge base. I treated the uploaded content as the source of truth. Some referenced crates (`import_rules`, `naming_rules`, `external_lint`, etc.) are not fully included, so my review focuses on what is visible in `cli-commands` and `shared`.

## Executive Summary

The codebase has a strong architectural intention: layered boundaries, taxonomy/contract separation, DI container wiring, and multi-language lint orchestration. However, the current implementation has several important backend issues:

1. **Architectural leakage**: `CheckCommandsSurface` contains orchestration, filtering, reporting, scoring-related decisions, and direct utility usage. This violates the stated AES layer rules and SOLID/SRP.
2. **Security concerns**:
   - `init --global` copies docs from the current project into a trusted global config directory.
   - MCP binary resolution can fall back to `PATH`, creating binary hijacking risk.
   - Path filtering uses string prefix matching, which can include unintended paths.
   - External linter execution should harden against argument injection and timeouts.
3. **Error handling weaknesses**:
   - Many failures are swallowed with `unwrap_or_default()`.
   - Maintenance/security/dependency commands can return success even when tools are missing or errors occur.
   - Invalid paths are sometimes converted into defaults instead of failing explicitly.
4. **Performance/scalability bottlenecks**:
   - Orphan detection uses raw file collection without ignore rules.
   - Multiple linters likely re-walk the filesystem independently.
   - Cross-platform symlink/directory cycle handling is buggy on non-Unix platforms.
   - Full result sets are held in memory and printed at once.
5. **Business logic correctness issues**:
   - CI mode only runs code-analysis, not the full pipeline.
   - Fix command measures improvement using only code-analysis results.
   - `--git-diff` mode ignores filter/format and does not run the full pipeline.
   - Workspace member matching uses substring matching, which can select unintended members.
6. **No database layer is present** in the uploaded code. If persistence is added later, use parameterized queries, migrations, pooling, and transactional writes.

---

# 1. Scope Note: Database Queries

The uploaded code does not contain a database layer, SQL queries, ORM usage, or persistence logic. Therefore, there are no direct SQL injection or database performance issues to review.

However, if you later persist lint reports, fix history, or project metadata, use the following baseline:

```rust
// Preferred: sqlx with compile-time checked queries
sqlx::query!(
    r#"
    INSERT INTO lint_violations (
        project_id,
        file_path,
        line,
        column,
        code,
        severity,
        message,
        created_at
    )
    VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
    "#,
    project_id,
    file_path,
    line,
    column,
    code,
    severity,
    message
)
.execute(&pool)
.await?;
```

Avoid:

```rust
// Dangerous: string-built SQL
let query = format!(
    "INSERT INTO lint_violations VALUES ('{}', '{}')",
    project_id, file_path
);
```

Recommended database practices:

- Use `sqlx` or `diesel` with parameterized queries.
- Add migrations via `sqlx migrate` or `diesel migration`.
- Use a connection pool.
- Add indexes for common query paths:
  - `project_id`
  - `code`
  - `severity`
  - `file_path`
  - `created_at`
- Batch inserts for large lint reports.
- Use transactions when applying fix metadata.
- Paginate API responses.
- Store normalized paths and project identifiers rather than arbitrary user strings where possible.

---

# 2. Prioritized Findings

| Severity | Area                   | Issue                                                                         | Impact                                                            |
| -------- | ---------------------- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| Critical | Security               | `init --global` copies docs from current project into trusted global config | Untrusted project can plant malicious docs into global config     |
| Critical | Security               | MCP binary resolution falls back to bare`lint-arwaky-mcp` on PATH           | PATH hijacking / binary planting risk                             |
| High     | Security / Correctness | Path filtering uses string prefix matching                                    | Files outside intended path may be included                       |
| High     | Performance            | Orphan detection uses raw file collection without ignore rules                | Scans`node_modules`, `target`, generated files; DoS/perf risk |
| High     | Error Handling         | Audit failures are swallowed with`unwrap_or_default()`                      | Silent incorrect lint reports                                     |
| High     | Architecture           | Surface layer contains orchestration, formatting, filtering                   | Violates AES layer rules and SRP                                  |
| High     | Business Logic         | CI mode only runs code-analysis                                               | CI score is incomplete                                            |
| High     | Business Logic         | Fix command counts only code-analysis violations                              | Misleading fix results                                            |
| High     | Performance            | File walker visited-set uses inode abstraction that breaks on non-Unix        | Potential traversal bugs on Windows                               |
| Medium   | Security               | External linters should use timeouts and argument separators                  | Hung processes / argument injection risk                          |
| Medium   | API Design             | Command catalog is duplicated and inconsistent                                | CLI/MCP command mismatch                                          |
| Medium   | Error Handling         | Maintenance commands return success on missing tools/errors                   | CI cannot trust exit codes                                        |
| Medium   | Performance            | Repeated filesystem walks across linters                                      | Slow monorepo scans                                               |
| Medium   | Maintainability        | Closure-based factories are opaque                                            | Harder to test/debug                                              |
| Low      | Reporting              | SARIF/JUnit output can be improved                                            | Better CI integration                                             |
| Low      | API Design             | Global CLI flags are unused or inconsistently applied                         | Confusing UX                                                      |

---

# 3. Architectural Issues

## 3.1 Surface Layer Is Doing Orchestration Work

`CheckCommandsSurface` currently:

- Runs code-analysis.
- Runs naming, import, external, and role audits concurrently.
- Runs orphan detection.
- Filters results by path and rule code.
- Formats text/JSON/SARIF/JUnit output.
- Prints multi-workspace summaries.
- Computes violation counts and exit-code decisions.

This is too much responsibility for a surface layer.

Your `ARCHITECTURE.md` says:

> Surfaces must not contain business calculation or orchestration.

But `CheckCommandsSurface::scan()` and `scan_with_discovery()` are orchestration pipelines.

### Recommendation

Introduce a dedicated agent/contract aggregate for the full lint pipeline.

```rust
// crates/shared/src/cli-commands/contract_analysis_pipeline_aggregate.rs

use async_trait::async_trait;
use thiserror::Error;

use crate::cli_commands::taxonomy_format_vo::Format;
use crate::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_threshold_vo::Threshold;

#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("path '{0}' does not exist")]
    PathNotFound(String),

    #[error("invalid path '{0}'")]
    InvalidPath(String),

    #[error("workspace discovery failed: {0}")]
    WorkspaceDiscovery(String),

    #[error("analysis failed: {0}")]
    Analysis(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub enum ScanMode {
    Check,
    Scan,
    Ci { threshold: Threshold },
}

#[derive(Debug, Clone)]
pub struct ScanRequest {
    pub target: FilePath,
    pub mode: ScanMode,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
}

#[derive(Debug, Clone)]
pub struct PipelineDiagnostic {
    pub source: String,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Default)]
pub struct ScanReport {
    pub results: LintResultList,
    pub diagnostics: Vec<PipelineDiagnostic>,
    pub score: Score,
}

impl ScanReport {
    pub fn violation_count(&self) -> usize {
        self.results.len()
    }

    pub fn has_critical(&self) -> bool {
        self.results
            .values
            .iter()
            .any(|r| r.severity == Severity::CRITICAL)
    }
}

#[async_trait]
pub trait IAnalysisPipelineAggregate: Send + Sync {
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError>;
}
```

Then the CLI surface becomes thin:

```rust
// crates/cli-commands/src/surface_scan_command.rs

use std::process::ExitCode;
use std::sync::Arc;

use shared::cli_commands::contract_analysis_pipeline_aggregate::{
    IAnalysisPipelineAggregate, ScanMode, ScanRequest,
};
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::common::taxonomy_path_vo::FilePath;

pub async fn handle_scan(
    pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    path: Option<String>,
    member: Option<String>,
    filter: Option<String>,
    format: Format,
) -> ExitCode {
    let raw_path = path.unwrap_or_else(|| ".".to_string());

    let target = match FilePath::new(raw_path.clone()) {
        Ok(target) => target,
        Err(err) => {
            eprintln!("Error: invalid path '{raw_path}': {err}");
            return ExitCode::from(2);
        }
    };

    if !std::path::Path::new(target.value()).exists() {
        eprintln!("Error: path '{}' does not exist", target.value());
        return ExitCode::from(2);
    }

    let request = ScanRequest {
        target,
        mode: ScanMode::Scan,
        filter,
        member,
        format,
    };

    match pipeline.run(request).await {
        Ok(report) => {
            // Rendering should be delegated to a ReportFormatter capability.
            // For illustration:
            println!("{}", render_report(&report, format));

            if report.violation_count() > 0 {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::from(2)
        }
    }
}

fn render_report(
    _report: &shared::cli_commands::contract_analysis_pipeline_aggregate::ScanReport,
    _format: Format,
) -> String {
    // Delegate to formatter capability:
    // - TextReportFormatter
    // - JsonReportFormatter
    // - SarifReportFormatter
    // - JunitReportFormatter
    todo!()
}
```

This gives you:

- Better SRP.
- Easier testing.
- Reusable pipeline for CLI, MCP, TUI, CI, and watch mode.
- Cleaner adherence to your own AES architecture.

---

## 3.2 Reporting Logic Should Be a Separate Capability

Currently, SARIF and JUnit formatting are embedded inside `CheckCommandsSurface`.

This violates Open/Closed Principle. If you add GitLab Code Quality, CodeClimate, HTML, or Markdown reports, you have to modify the surface.

### Recommended Design

```rust
// crates/shared/src/cli-commands/contract_report_formatter_protocol.rs

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;

pub struct ReportContext {
    pub target: FilePath,
    pub tool_name: String,
    pub tool_version: String,
}

pub trait ReportFormatter: Send + Sync {
    fn format(&self, results: &[LintResult], context: &ReportContext) -> String;
}
```

Example JSON formatter:

```rust
pub struct JsonReportFormatter;

impl ReportFormatter for JsonReportFormatter {
    fn format(&self, results: &[LintResult], _context: &ReportContext) -> String {
        serde_json::to_string_pretty(results).unwrap_or_else(|_| "[]".to_string())
    }
}
```

Example SARIF formatter:

```rust
pub struct SarifReportFormatter;

impl ReportFormatter for SarifReportFormatter {
    fn format(&self, results: &[LintResult], context: &ReportContext) -> String {
        // Move existing format_sarif_output() here.
        // Also add:
        // - rule metadata
        // - columns
        // - related locations
        // - URI-encoded artifact locations
        todo!()
    }
}
```

Then use a formatter registry:

```rust
pub fn formatter_for(format: Format) -> Box<dyn ReportFormatter> {
    match format {
        Format::Text => Box::new(TextReportFormatter),
        Format::Json => Box::new(JsonReportFormatter),
        Format::Sarif => Box::new(SarifReportFormatter),
        Format::Junit => Box::new(JunitReportFormatter),
    }
}
```

---

## 3.3 Command Catalog Is Duplicated and Inconsistent

You have at least two command metadata sources:

1. `COMMAND_CATALOG` in `taxonomy_catalog_constant.rs`
2. `CommandCatalogVO::command_catalog()` in `taxonomy_command_catalog_vo.rs`

They are not fully consistent.

For example:

- `COMMAND_CATALOG` includes `orphan`, `init`, `install`, `mcp-config`, `config-show`.
- `CommandCatalogVO` includes `plugins`, which is not present in the CLI enum.
- `CommandCatalogVO` is missing several commands present in the CLI.
- Examples use `lint-arwaky-cli`, while the clap command name is `lint-arwaky`.

This can cause MCP clients or help systems to expose incorrect capabilities.

### Fix: Single Source of Truth

```rust
// crates/shared/src/cli-commands/taxonomy_catalog_constant.rs

pub struct CommandSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub example: &'static str,
}

pub const BINARY_NAME: &str = "lint-arwaky";

pub static COMMAND_CATALOG: &[CommandSpec] = &[
    CommandSpec {
        name: "check",
        description: "Run full architecture compliance analysis",
        example: "lint-arwaky check /path",
    },
    CommandSpec {
        name: "scan",
        description: "Deep directory scan with workspace discovery",
        example: "lint-arwaky scan ./src/",
    },
    CommandSpec {
        name: "fix",
        description: "Apply safe automatic fixes",
        example: "lint-arwaky fix file.py",
    },
    CommandSpec {
        name: "ci",
        description: "CI mode with threshold and exit codes",
        example: "lint-arwaky ci /path --threshold 80",
    },
    CommandSpec {
        name: "doctor",
        description: "Diagnose environment health",
        example: "lint-arwaky doctor",
    },
    CommandSpec {
        name: "orphan",
        description: "Check if a file is dead/unreachable code",
        example: "lint-arwaky orphan <path>",
    },
    CommandSpec {
        name: "security",
        description: "Scan for security vulnerabilities",
        example: "lint-arwaky security /path",
    },
    CommandSpec {
        name: "dependencies",
        description: "Scan dependencies for known vulnerabilities",
        example: "lint-arwaky dependencies .",
    },
    CommandSpec {
        name: "watch",
        description: "Watch files and lint on change",
        example: "lint-arwaky watch ./src/",
    },
    CommandSpec {
        name: "init",
        description: "Create default config",
        example: "lint-arwaky init",
    },
    CommandSpec {
        name: "install",
        description: "Install adapter dependencies",
        example: "lint-arwaky install",
    },
    CommandSpec {
        name: "mcp-config",
        description: "Print MCP server config",
        example: "lint-arwaky mcp-config --client claude",
    },
    CommandSpec {
        name: "config-show",
        description: "Show active configuration",
        example: "lint-arwaky config-show",
    },
];
```

Then generate the catalog VO from that constant:

```rust
// crates/shared/src/cli-commands/taxonomy_command_catalog_vo.rs

use std::collections::HashMap;

use crate::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
use crate::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;
use crate::common::taxonomy_suggestion_vo::{DescriptionVO, Suggestion};
use crate::mcp_server::taxonomy_action_vo::ActionName;

pub struct CommandCatalogVO;

impl CommandCatalogVO {
    pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
        COMMAND_CATALOG
            .iter()
            .map(|spec| {
                (
                    ActionName::from(spec.name),
                    CommandMetadataVO::new(
                        DescriptionVO::new(spec.description),
                        Suggestion::new(spec.example),
                    ),
                )
            })
            .collect()
    }
}
```

---

# 4. Security Review

## 4.1 Critical: Global Init Copies Untrusted Project Docs Into Trusted Config

In `surface_setup_command.rs`, `handle_init_global()` copies docs from the current project root into the global config directory:

```rust
let doc_files = [
    "SKILL.md",
    "ARCHITECTURE.md",
    "MIGRATION_RUST.md",
    "MIGRATION_PYTHON.md",
    "MIGRATION_TYPESCRIPT.md",
    "RULES_AES.md",
];

for doc in &doc_files {
    match std::fs::read_to_string(doc) {
        Ok(content) => {
            // writes into global config
        }
        Err(_) => println!("  {doc} — not found in project root, skipping"),
    }
}
```

This is risky.

If a user runs `lint-arwaky init --global` inside an untrusted repository, that repository can supply malicious `SKILL.md`, `ARCHITECTURE.md`, etc. Those files are then placed into a trusted global config directory and may later be distributed into other projects by `lint-arwaky init`.

This is a supply-chain/persistence risk.

### Fix

Only install trusted, bundled docs.

Use `include_str!` at compile time or install from a trusted asset directory verified by checksum.

```rust
// crates/cli-commands/src/surface_setup_command.rs

struct TrustedDoc {
    name: &'static str,
    content: &'static str,
}

const TRUSTED_DOCS: &[TrustedDoc] = &[
    TrustedDoc {
        name: "SKILL.md",
        content: include_str!("../assets/SKILL.md"),
    },
    TrustedDoc {
        name: "ARCHITECTURE.md",
        content: include_str!("../assets/ARCHITECTURE.md"),
    },
    TrustedDoc {
        name: "MIGRATION_RUST.md",
        content: include_str!("../assets/MIGRATION_RUST.md"),
    },
    TrustedDoc {
        name: "MIGRATION_PYTHON.md",
        content: include_str!("../assets/MIGRATION_PYTHON.md"),
    },
    TrustedDoc {
        name: "MIGRATION_TYPESCRIPT.md",
        content: include_str!("../assets/MIGRATION_TYPESCRIPT.md"),
    },
    TrustedDoc {
        name: "RULES_AES.md",
        content: include_str!("../assets/RULES_AES.md"),
    },
];

fn write_trusted_docs(config_dir: &std::path::Path) -> bool {
    let mut all_ok = true;

    for doc in TRUSTED_DOCS {
        let target = config_dir.join(doc.name);
        let target_str = target.to_string_lossy();

        if target.exists() {
            println!("  {} — already exists, skipping", doc.name);
            continue;
        }

        match std::fs::write(&target, doc.content) {
            Ok(_) => println!("  {} — created", doc.name),
            Err(err) => {
                println!("  {} — error: {err}", doc.name);
                all_ok = false;
            }
        }

        let _ = target_str; // if needed by file_exists abstraction
    }

    all_ok
}
```

Do not copy docs from the current working project into global config unless the user explicitly asks and the source is verified.

---

## 4.2 Critical: MCP Binary Resolution Should Not Fall Back Blindly to PATH

Current code:

```rust
fn which_mcp_binary() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-mcp");
            if sibling.exists() {
                return sibling.to_string_lossy().to_string();
            }
        }
    }

    "lint-arwaky-mcp".to_string()
}
```

Returning a bare executable name means the MCP client may resolve it from `PATH`.

If an attacker can place `lint-arwaky-mcp` earlier in the user's `PATH`, they can execute arbitrary code when the MCP client launches the server.

### Fix

Fail closed if the binary cannot be resolved to an absolute, canonicalized path.

```rust
use std::path::PathBuf;

fn resolve_mcp_binary() -> Result<PathBuf, String> {
    // 1. Explicit override.
    if let Ok(explicit) = std::env::var("LINT_ARWAKY_MCP_BIN") {
        let path = PathBuf::from(explicit);

        if !path.is_file() {
            return Err(format!(
                "LINT_ARWAKY_MCP_BIN points to a non-file path: {}",
                path.display()
            ));
        }

        return path
            .canonicalize()
            .map_err(|err| format!("cannot canonicalize MCP binary: {err}"));
    }

    // 2. Sibling of current executable.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-mcp");

            if sibling.is_file() {
                return sibling
                    .canonicalize()
                    .map_err(|err| format!("cannot canonicalize MCP binary: {err}"));
            }
        }
    }

    // 3. Do not fall back to bare PATH lookup by default.
    Err(
        "lint-arwaky-mcp binary not found. Set LINT_ARWAKY_MCP_BIN to an absolute path."
            .to_string(),
    )
}
```

Then:

```rust
pub fn handle_mcp_config(client: &str) -> ExitCode {
    let binary = match resolve_mcp_binary() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error: {err}");
            return ExitCode::from(2);
        }
    };

    let binary_str = binary.to_string_lossy().to_string();

    let config = serde_json::json!({
        "mcpServers": {
            "lint-arwaky": {
                "command": binary_str,
                "args": [],
                "env": {}
            }
        }
    });

    println!("MCP Client Configuration for: {client}");
    println!("Binary: {}", binary.display());
    println!();
    println!("{}", serde_json::to_string_pretty(&config).unwrap_or_default());

    ExitCode::SUCCESS
}
```

If you must support PATH lookup, use a vetted `which` crate and still canonicalize the result. But fail closed is safer.

---

## 4.3 High: Path Filtering Uses String Prefix Matching

In `filter_and_display_results()`:

```rust
let canonical_scan_path = crate::surface_common_command::canonicalize_path(path);

let abs_path = cwd.join(&r.file.value);

r.code.code() == code
    && abs_path.to_string_lossy().starts_with(&canonical_scan_path)
```

String prefix matching is unsafe for paths.

Example:

```text
canonical_scan_path = "/project/src"
abs_path = "/project/src_secret.rs"
```

`"/project/src_secret.rs".starts_with("/project/src")` is true, even though the file is outside `/project/src`.

### Fix

Use `Path::starts_with`, not string prefix matching.

```rust
use std::path::{Path, PathBuf};

fn canonicalize_or_keep(path: &Path) -> PathBuf {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
}

fn is_within_path(base: &Path, candidate: &Path) -> bool {
    let base = canonicalize_or_keep(base);
    let candidate = canonicalize_or_keep(candidate);

    candidate.starts_with(base)
}
```

Use it in filtering:

```rust
let cwd = crate::surface_common_command::current_dir();
let base_path = std::path::Path::new(path);

let filtered_results: Vec<_> = all_results
    .into_iter()
    .filter(|result| {
        let abs_path = cwd.join(&result.file.value);

        let matches_code = match &filter {
            Some(code) => result.code.code() == code.as_str(),
            None => true,
        };

        matches_code && is_within_path(base_path, &abs_path)
    })
    .collect();
```

Better still, canonicalize the base path once:

```rust
pub struct PathScope {
    root: PathBuf,
}

impl PathScope {
    pub fn new(root: &Path) -> std::io::Result<Self> {
        Ok(Self {
            root: root.canonicalize()?,
        })
    }

    pub fn contains(&self, candidate: &Path) -> bool {
        let candidate = candidate
            .canonicalize()
            .unwrap_or_else(|_| candidate.to_path_buf());

        candidate.starts_with(&self.root)
    }
}
```

Usage:

```rust
let scope = match PathScope::new(std::path::Path::new(path)) {
    Ok(scope) => scope,
    Err(err) => {
        eprintln!("Error: cannot resolve path '{path}': {err}");
        return 0;
    }
};

let filtered_results: Vec<_> = all_results
    .into_iter()
    .filter(|result| {
        let abs_path = cwd.join(&result.file.value);
        scope.contains(&abs_path)
    })
    .collect();
```

---

## 4.4 Medium: External Linter Execution Should Use Timeouts and Argument Separators

The actual external lint execution is not fully visible, but `IExternalLintAggregate` implies subprocess-based linters.

For subprocess execution, ensure:

1. Never use a shell.
2. Use argument lists.
3. Use `--` before paths where supported.
4. Apply timeouts.
5. Capture stdout/stderr size limits.
6. Do not pass untrusted paths as options.

### Rust Example

```rust
use std::process::Command;
use std::time::Duration;

fn run_ruff(path: &str) -> Result<String, String> {
    let output = Command::new("ruff")
        .arg("check")
        .arg("--output-format")
        .arg("json")
        .arg("--")
        .arg(path)
        .output()
        .map_err(|err| format!("failed to start ruff: {err}"))?;

    if !output.status.success() {
        // Many linters return non-zero when violations exist.
        // Distinguish between "violations found" and "tool failure".
    }

    String::from_utf8(output.stdout)
        .map_err(|err| format!("invalid ruff output: {err}"))
}
```

For production, wrap with `tokio::process::Command` and timeout:

```rust
use tokio::process::Command as TokioCommand;
use tokio::time::{timeout, Duration};

async fn run_ruff_async(path: &str) -> Result<String, String> {
    let future = TokioCommand::new("ruff")
        .arg("check")
        .arg("--output-format")
        .arg("json")
        .arg("--")
        .arg(path)
        .output();

    match timeout(Duration::from_secs(120), future).await {
        Ok(Ok(output)) => {
            String::from_utf8(output.stdout)
                .map_err(|err| format!("invalid ruff output: {err}"))
        }
        Ok(Err(err)) => Err(format!("failed to start ruff: {err}")),
        Err(_) => Err("ruff timed out after 120 seconds".to_string()),
    }
}
```

### Python Adapter Example

```python
import subprocess

def run_ruff(path: str) -> str:
    try:
        result = subprocess.run(
            ["ruff", "check", "--output-format", "json", "--", path],
            capture_output=True,
            text=True,
            timeout=120,
            check=False,
        )
    except subprocess.TimeoutExpired as exc:
        raise RuntimeError("ruff timed out") from exc

    return result.stdout
```

Avoid:

```python
# Dangerous
subprocess.run(f"ruff {path}", shell=True)
```

### TypeScript Adapter Example

```ts
import { execFile } from "node:child_process";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

export async function runEslint(path: string): Promise<string> {
  const { stdout } = await execFileAsync(
    "eslint",
    ["--format", "json", "--", path],
    {
      timeout: 120_000,
      maxBuffer: 10 * 1024 * 1024,
    },
  );

  return stdout;
}
```

Avoid:

```ts
// Dangerous
await exec(`eslint ${path}`);
```

---

## 4.5 Medium: `config-show` May Leak Secrets

`handle_config_show()` prints raw config content:

```rust
println!("{}", source.raw_content);
```

If config files contain tokens, API keys, registry credentials, or environment secrets, this command may leak them into logs, CI output, or terminal scrollback.

### Recommendation

Add redaction for known secret fields before printing.

```rust
fn redact_config(raw: &str) -> String {
    // Simple line-based redaction for YAML-like config.
    raw.lines()
        .map(|line| {
            let lower = line.to_lowercase();

            if lower.contains("token")
                || lower.contains("secret")
                || lower.contains("password")
                || lower.contains("api_key")
                || lower.contains("apikey")
                || lower.contains("authorization")
            {
                if let Some((key, _)) = line.split_once(':') {
                    return format!("{key}: [REDACTED]");
                }
            }

            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
```

Then:

```rust
println!("{}", redact_config(&source.raw_content));
```

For structured configs, parse and redact fields explicitly.

---

# 5. Error Handling Review

## 5.1 High: Audit Failures Are Swallowed

In `CheckCommandsSurface::scan()`:

```rust
all_results.extend(naming_results.unwrap_or_default());
all_results.extend(import_results.unwrap_or_default());
```

If naming or import auditing fails, the CLI reports zero violations for that subsystem.

That is dangerous for CI. A failed linter should not look like a clean project.

### Fix

Introduce diagnostics and propagate subsystem failures.

```rust
use shared::cli_commands::contract_analysis_pipeline_aggregate::PipelineDiagnostic;
use shared::common::taxonomy_severity_vo::Severity;

fn merge_audit_result(
    all_results: &mut Vec<LintResult>,
    diagnostics: &mut Vec<PipelineDiagnostic>,
    source: &str,
    result: Result<Vec<LintResult>, shared::common::taxonomy_adapter_error::ScanError>,
) {
    match result {
        Ok(mut values) => all_results.append(&mut values),
        Err(err) => diagnostics.push(PipelineDiagnostic {
            source: source.to_string(),
            message: err.to_string(),
            severity: Severity::HIGH,
        }),
    }
}
```

Usage:

```rust
let mut diagnostics = Vec::new();

merge_audit_result(
    &mut all_results,
    &mut diagnostics,
    "naming-rules",
    naming_results,
);

merge_audit_result(
    &mut all_results,
    &mut diagnostics,
    "import-rules",
    import_results,
);
```

Then decide exit behavior:

```rust
let has_diagnostic_failure = diagnostics
    .iter()
    .any(|d| d.severity == Severity::HIGH || d.severity == Severity::CRITICAL);

if has_diagnostic_failure {
    for diagnostic in &diagnostics {
        eprintln!(
            "[{}] {}",
            diagnostic.severity,
            diagnostic.message
        );
    }

    return ExitCode::from(2);
}
```

---

## 5.2 High: External Lint Aggregate Should Return Errors

Current contract:

```rust
#[async_trait]
pub trait IExternalLintAggregate: Send + Sync {
    async fn scan_all(&self, path: &FilePath) -> LintResultList;
    fn adapter_names(&self) -> Vec<String>;
}
```

There is no error channel.

If an adapter crashes, times out, or is missing, the implementation is forced to swallow the error or panic.

### Fix

Return a richer result:

```rust
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_adapter_error::ScanError;

pub struct ExternalLintOutput {
    pub results: LintResultList,
    pub errors: Vec<ScanError>,
}

#[async_trait]
pub trait IExternalLintAggregate: Send + Sync {
    async fn scan_all(&self, path: &FilePath) -> ExternalLintOutput;

    fn adapter_names(&self) -> Vec<String>;
}
```

Or:

```rust
#[async_trait]
pub trait IExternalLintAggregate: Send + Sync {
    async fn scan_all(
        &self,
        path: &FilePath,
    ) -> Result<LintResultList, Vec<ScanError>>;

    fn adapter_names(&self) -> Vec<String>;
}
```

---

## 5.3 Medium: Maintenance Commands Should Not Return Success on Failure

Current `handle_security()` returns success when the tool is missing:

```rust
if !report.tool_installed {
    println!("{} not available. Please install it.", report.tool_name);
    return ExitCode::SUCCESS;
}
```

Current `handle_dependencies()` returns success even on error:

```rust
Err(e) => {
    println!("{e}");
}
```

Then falls through to `ExitCode::SUCCESS`.

This makes CI unreliable.

### Fix

Use distinct exit codes:

```text
0 = success, no violations
1 = violations/findings found
2 = operational error
3 = required tool missing
```

Example:

```rust
pub async fn handle_security(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let fp = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };

    println!("Security Vulnerability Scan — {}", fp.value);
    println!();

    let report = maintenance_orchestrator.run_security_scan(&fp).await;

    println!("Language: {}", report.language);
    println!("Tool: {}", report.tool_name);

    if !report.tool_installed {
        eprintln!(
            "Error: {} is not installed. Please install it.",
            report.tool_name
        );
        return ExitCode::from(3);
    }

    println!("Findings: {}", report.findings.len());

    for finding in &report.findings {
        println!(
            "  {} {} {}:{} {}",
            finding.severity.to_uppercase(),
            finding.test_id,
            finding.file,
            finding.line,
            finding.issue
        );
    }

    if !report.findings.is_empty() {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
```

Dependencies:

```rust
pub async fn handle_dependencies(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    path: Option<FilePath>,
) -> ExitCode {
    let fp = match path {
        Some(p) => p,
        None => FilePath::new(".").unwrap_or_default(),
    };

    println!("Dependency Report — {}", fp.value);
    println!();

    match maintenance_orchestrator.run_dependency_report(&fp).await {
        Ok(report) => {
            println!("Language: {}", report.language);
            println!("Dependencies: {} total", report.dependencies.len());
            println!();
            println!("{:<25} {:<12} Type", "Package", "Version");

            for dep in report.dependencies.iter().take(30) {
                println!("{:<25} {:<12} {}", dep.name, dep.version, dep.dep_type);
            }

            if report.dependencies.len() > 30 {
                println!("... and {} more", report.dependencies.len() - 30);
            }

            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::from(2)
        }
    }
}
```

Doctor should also support a strict mode:

```rust
pub async fn handle_doctor(
    maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    strict: bool,
) -> ExitCode {
    let diag = maintenance_orchestrator.diagnose_toolchain().await;

    // print diagnostics...

    let has_failure = diag
        .rust_tools
        .iter()
        .chain(diag.python_tools.iter())
        .chain(diag.js_tools.iter())
        .chain(diag.vcs_tools.iter())
        .any(|tool| tool.status != "OK");

    if strict && has_failure {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
```

---

## 5.4 Medium: Invalid Paths Should Not Become Defaults

There are several places where invalid paths become defaults:

```rust
FilePath::new(path).unwrap_or_default()
```

For CLI commands, invalid user input should usually fail with a clear error.

### Fix

Use a helper:

```rust
fn resolve_target_path(path: Option<String>) -> Result<FilePath, String> {
    let raw = path.unwrap_or_else(|| ".".to_string());

    let file_path = FilePath::new(raw.clone())
        .map_err(|err| format!("invalid path '{raw}': {err}"))?;

    if !std::path::Path::new(file_path.value()).exists() {
        return Err(format!("path '{}' does not exist", file_path.value()));
    }

    Ok(file_path)
}
```

Usage:

```rust
let target = match resolve_target_path(path) {
    Ok(target) => target,
    Err(err) => {
        eprintln!("Error: {err}");
        return ExitCode::from(2);
    }
};
```

---

# 6. Performance and Scalability Review

## 6.1 High: Orphan Detection Uses Raw File Collection Without Ignore Rules

In `scan_with_discovery()`:

```rust
let all_source_files: Vec<String> =
    shared::common::collect_all_source_files_raw(&scan_root)
        .iter()
        .map(|f| f.value.clone())
        .collect();
```

`collect_all_source_files_raw` does not apply default ignores.

That means orphan detection may scan:

- `node_modules`
- `target`
- `.venv`
- `dist`
- `build`
- generated files
- vendored dependencies

This can cause severe performance degradation in large monorepos.

### Fix

Use ignore-aware collection:

```rust
let all_source_files: Vec<String> =
    shared::common::collect_all_source_files(&scan_root)
        .iter()
        .map(|f| f.value.clone())
        .collect();
```

If you need workspace-specific ignore rules, build them explicitly:

```rust
use shared::common::utility_file::{default_ignored_paths, walk_source_files};

fn collect_workspace_source_files(
    root: &std::path::Path,
    config: &shared::config_system::taxonomy_config_vo::ArchitectureConfig,
) -> Vec<String> {
    let mut ignored = default_ignored_paths();

    for path in config.ignored_paths.values.iter() {
        let normalized = path.value.replace('/', std::path::MAIN_SEPARATOR_STR);
        if !normalized.is_empty() && !ignored.contains(&normalized) {
            ignored.push(normalized);
        }
    }

    let mut files = Vec::new();
    walk_source_files(root, &mut files, &ignored);

    files.into_iter().map(|f| f.value).collect()
}
```

Usage:

```rust
let all_source_files = collect_workspace_source_files(&scan_root, &default_config);
```

---

## 6.2 High: File Walker Visited-Set Breaks on Non-Unix Platforms

In `utility_file.rs`, inode-based cycle detection is used:

```rust
#[cfg(unix)]
fn get_inode(meta: &std::fs::Metadata) -> u64 {
    meta.ino()
}

#[cfg(not(unix))]
fn get_inode(_meta: &std::fs::Metadata) -> u64 {
    0
}
```

On non-Unix platforms, every directory returns inode `0`.

Because the visited set inserts `0` for the first directory, subsequent directories may be skipped incorrectly:

```rust
if !visited.insert(inode) {
    continue;
}
```

This can cause incomplete scans on Windows.

### Fix

Use canonical paths for cross-platform cycle detection, or use a platform-specific file ID.

Simple cross-platform version:

```rust
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn walk_source_files(dir: &Path, files: &mut Vec<FilePath>, ignored: &[String]) {
    let root = std::fs::canonicalize(dir).unwrap_or_else(|_| dir.to_path_buf());
    let mut visited: HashSet<PathBuf> = HashSet::new();

    visited.insert(root.clone());

    walk_source_files_inner(&root, files, ignored, &mut visited, &root);
}

fn walk_source_files_inner(
    dir: &Path,
    files: &mut Vec<FilePath>,
    ignored: &[String],
    visited: &mut HashSet<PathBuf>,
    root: &Path,
) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if is_ignored_dir(&path, ignored) {
            continue;
        }

        // Handle symlinks carefully.
        if let Ok(sym_meta) = std::fs::symlink_metadata(&path) {
            if sym_meta.file_type().is_symlink() {
                let target = match std::fs::canonicalize(&path) {
                    Ok(target) => target,
                    Err(_) => continue,
                };

                // Prevent symlink escape.
                if !target.starts_with(root) {
                    continue;
                }

                // Prevent cycles.
                if !visited.insert(target.clone()) {
                    continue;
                }

                if target.is_dir() {
                    walk_source_files_inner(&target, files, ignored, visited, root);
                } else if target.is_file() {
                    collect_source_file(&target, files);
                }

                continue;
            }
        }

        if path.is_dir() {
            let canonical = match std::fs::canonicalize(&path) {
                Ok(canonical) => canonical,
                Err(_) => continue,
            };

            if !visited.insert(canonical) {
                continue;
            }

            walk_source_files_inner(&path, files, ignored, visited, root);
        } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if is_source_file(ext) {
                collect_source_file(&path, files);
            }
        }
    }
}
```

For better performance on Unix, you can keep inode-based detection, but provide a correct fallback on Windows using canonical paths or the `same-file` crate.

---

## 6.3 Medium: Repeated Filesystem Walks

The pipeline currently likely walks the filesystem multiple times:

1. Code-analysis.
2. Naming rules.
3. Import rules.
4. Role rules.
5. External lint.
6. Orphan detection.

Each subsystem receives a `FilePath` and probably discovers files independently.

This is expensive in large repositories.

### Recommendation

Create a shared workspace file index.

```rust
pub struct WorkspaceFileIndex {
    pub root: PathBuf,
    pub files: Vec<FilePath>,
}

impl WorkspaceFileIndex {
    pub fn discover(root: &Path, ignored: &[String]) -> Self {
        let mut files = Vec::new();
        walk_source_files(root, &mut files, ignored);

        Self {
            root: root.to_path_buf(),
            files,
        }
    }
}
```

Then pass the index to subsystems:

```rust
#[async_trait]
pub trait INamingRunnerAggregate: Send + Sync {
    async fn run_audit_index(
        &self,
        index: &WorkspaceFileIndex,
    ) -> Result<Vec<LintResult>, ScanError>;
}
```

If changing all traits is too invasive, start by caching discovery results per scan request.

---

## 6.4 Medium: Large Reports Are Fully Materialized in Memory

`scan_with_discovery()` accumulates:

```rust
let mut global_all_results = Vec::new();
```

Then prints everything at the end.

For very large monorepos, this can consume significant memory.

### Recommendations

1. Stream JSON output where possible.
2. Write reports to `--output-dir` incrementally.
3. Paginate text output.
4. Use per-workspace report files.
5. Add a maximum violation limit for interactive mode.

Example:

```rust
pub struct ReportLimits {
    pub max_console_violations: usize,
}

impl Default for ReportLimits {
    fn default() -> Self {
        Self {
            max_console_violations: 500,
        }
    }
}
```

For JSON output, consider writing to a file:

```rust
use std::fs::File;
use std::io::BufWriter;

fn write_json_report(path: &Path, results: &[LintResult]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, results)?;

    Ok(())
}
```

If `--output-dir` is implemented, protect against path traversal:

```rust
fn safe_output_path(output_dir: &Path, file_name: &str) -> Result<PathBuf, String> {
    let output_dir = output_dir
        .canonicalize()
        .map_err(|err| format!("invalid output dir: {err}"))?;

    let target = output_dir.join(file_name);

    if !target.starts_with(&output_dir) {
        return Err("output path escapes output directory".to_string());
    }

    Ok(target)
}
```

---

## 6.5 Medium: Add Timeouts to Long-Running Audits

External linters can hang.

Wrap async operations with timeouts:

```rust
use tokio::time::{timeout, Duration};

let external_future = external_lint.scan_all(&path_obj);

match timeout(Duration::from_secs(180), external_future).await {
    Ok(result) => {
        all_results.extend(result.values);
    }
    Err(_) => {
        diagnostics.push(PipelineDiagnostic {
            source: "external-lint".to_string(),
            message: "external lint timed out after 180 seconds".to_string(),
            severity: Severity::HIGH,
        });
    }
}
```

For the whole pipeline, add a global timeout as well.

---

# 7. Business Logic Review

## 7.1 High: CI Mode Runs Only Code-Analysis

Current `handle_ci()` uses:

```rust
code_analysis_linter.run_code_analysis_path(&root)
```

This means CI mode does not run:

- naming rules
- import rules
- role rules
- external linters
- orphan detection

But the CLI documentation presents the tool as a full architecture compliance gatekeeper.

### Fix

CI mode should run the same pipeline as `scan` or `check`, then compute score and threshold.

```rust
pub async fn handle_ci(
    pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    path: Option<String>,
    threshold: Threshold,
) -> ExitCode {
    let target = match resolve_target_path(path) {
        Ok(target) => target,
        Err(err) => {
            eprintln!("Error: {err}");
            return ExitCode::from(2);
        }
    };

    let request = ScanRequest {
        target,
        mode: ScanMode::Ci { threshold },
        filter: None,
        member: None,
        format: Format::Text,
    };

    match pipeline.run(request).await {
        Ok(report) => {
            let score = report.score.value();
            let threshold_value = threshold.value() as f64;

            println!("Architecture Compliance CI");
            println!("Score: {:.1} / 100", score);
            println!("Threshold: {}", threshold.value());
            println!();

            let mut reasons = Vec::new();

            if report.has_critical() {
                reasons.push("CRITICAL violation(s) detected — auto-fail triggered".to_string());
            }

            if score < threshold_value {
                reasons.push(format!(
                    "Score below threshold ({:.1} < {:.1})",
                    score, threshold_value
                ));
            }

            if reasons.is_empty() {
                println!("Result: PASS (exit code 0)");
                ExitCode::SUCCESS
            } else {
                for reason in reasons {
                    println!("  {reason}");
                }
                println!("Result: FAIL (exit code 1)");
                ExitCode::from(1)
            }
        }
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::from(2)
        }
    }
}
```

Also avoid truncating score:

Current:

```rust
let below_threshold = (score.value() as u32) < threshold.value();
```

Better:

```rust
let below_threshold = score.value() < threshold.value() as f64;
```

---

## 7.2 High: Fix Command Measures Only Code-Analysis Violations

Current `FixCommandsSurface::run_fix()`:

```rust
let results = self.code_analysis_linter.run_code_analysis(&project_path);
println!("Found {} violations before fix ...", results.len());

let fix_result = fix_orch.execute(&project_path);

let after_results = self.code_analysis_linter.run_code_analysis(&project_path);
let fixed_count = results.len().saturating_sub(after_results.len());
```

But the comment says fixable violations include:

- AES101 naming
- AES203 unused imports
- AES304 bypass

If the before/after measurement only uses code-analysis, the reported fixed count may be wrong.

### Fix

Use the full pipeline before and after, or return structured fix results from the fix aggregate.

Better contract:

```rust
pub struct FixReport {
    pub changed_files: Vec<FilePath>,
    pub fixes_applied: Count,
    pub violations_before: Count,
    pub violations_after: Count,
    pub output: StdOutput,
}

pub trait LintFixOrchestratorAggregate: Send + Sync {
    fn execute(&self, path: &FilePath) -> Result<FixReport, FixError>;
}
```

Then:

```rust
pub fn run_fix(&self, project_path: FilePath, dry_run: bool) -> ExitCode {
    if dry_run {
        println!("[DRY-RUN] Previewing fixes for {}...", project_path.value);
    } else {
        println!("Applying safe fixes to {}...", project_path.value);
    }

    let fix_orchestrator = (self.fix_orchestrator_factory)(dry_run);

    match fix_orchestrator.execute(&project_path) {
        Ok(report) => {
            println!("{}", report.output.value);

            if dry_run {
                println!("Dry-run complete — no changes applied.");
                return ExitCode::SUCCESS;
            }

            println!(
                "Fixed {} violation(s) across {} file(s).",
                report.fixes_applied.value,
                report.changed_files.len()
            );

            if report.violations_after.value == 0 {
                println!("Fix complete — all fixable violations resolved.");
                ExitCode::SUCCESS
            } else {
                println!(
                    "Fix complete — {} violation(s) remain.",
                    report.violations_after.value
                );
                ExitCode::from(1)
            }
        }
        Err(err) => {
            eprintln!("Fix failed: {err}");
            ExitCode::from(2)
        }
    }
}
```

---

## 7.3 Medium: `check --git-diff` Ignores Filter and Format

In `handle_check()`:

```rust
if git_diff {
    rt.block_on(crate::surface_git_command::handle_git_diff(
        git_agg,
        ctx.code_analysis_linter.clone(),
        GitBranchName::new("HEAD"),
    ))
}
```

This path ignores:

- `filter`
- `format`
- full linter pipeline

Users may expect:

```bash
lint-arwaky check --git-diff --format json --filter AES204
```

to work.

### Fix

Create a pipeline request for changed files:

```rust
pub struct ChangedFileScanRequest {
    pub base: GitBranchName,
    pub filter: Option<String>,
    pub format: Format,
}
```

Then run the full pipeline only on changed files.

---

## 7.4 Medium: Workspace Member Matching Uses Substring Matching

Current:

```rust
ws_file.contains(member_name) || ws.path.value.contains(member_name)
```

This can match unintended members.

Example:

```text
--member shared
```

could match:

- `shared`
- `shared-utils`
- `shared_legacy`
- `my-shared-package`

### Fix

Use exact workspace member name matching:

```rust
fn matches_workspace_member(ws: &WorkspaceInfo, member_name: &str) -> bool {
    let path = std::path::Path::new(&ws.path.value);

    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    if file_name == member_name {
        return true;
    }

    // Optional: allow relative path match, e.g. crates/shared
    let normalized = ws.path.value.trim_end_matches('/');

    normalized == member_name
        || normalized.ends_with(&format!("/{member_name}"))
}
```

Usage:

```rust
let filtered: Vec<_> = workspaces
    .into_iter()
    .filter(|ws| matches_workspace_member(ws, member_name))
    .collect();
```

---

## 7.5 Medium: Per-Project Factory Does Not Affect External Lint or Orphan Detection

In `scan()` and `scan_with_discovery()`, the factory creates a `CheckContext`, but only these are used:

```rust
let (
    code_analysis_linter,
    naming_orchestrator,
    import_orchestrator,
    role_orchestrator,
) = (...);
```

Then external lint still uses:

```rust
self.external_lint.scan_all(&path_obj)
```

And orphan detection still uses:

```rust
self.orphan_orchestrator
```

So if the factory provides workspace-specific external lint or orphan orchestrators, they are ignored.

### Fix

Use the full context:

```rust
let ctx = effective_factory(config.clone());

let code_analysis_linter = ctx.code_analysis_linter.clone();
let naming_orchestrator = ctx.naming_orchestrator.clone();
let import_orchestrator = ctx.import_orchestrator.clone();
let role_orchestrator = ctx.role_orchestrator.clone();
let external_lint = ctx.external_lint.clone();
let orphan_orchestrator = ctx.orphan_orchestrator.clone();

let (naming_results, import_results, external_results, role_results) = rt.block_on(async {
    tokio::join!(
        naming_orchestrator.run_audit(&path_obj),
        import_orchestrator.run_audit(&path_obj),
        external_lint.scan_all(&path_obj),
        role_orchestrator.run_audit(&path_obj),
    )
});

let orphan_results = self.run_orphan_detection_pass(path, &orphan_orchestrator);
```

Apply the same change in `scan_with_discovery()`.

---

# 8. API Design Review

## 8.1 Global CLI Flags Are Unused or Inconsistent

The CLI defines:

```rust
pub verbose: bool,
pub quiet: bool,
pub output_dir: Option<String>,
pub filter: Option<String>,
```

But many commands do not use them.

### Recommendations

1. Implement `verbose` using `tracing_subscriber`.
2. Implement `quiet` by suppressing non-essential stdout.
3. Implement `output-dir` or remove it.
4. Make `filter` valid only for commands that produce violations.

Example tracing setup:

```rust
use tracing_subscriber::EnvFilter;

pub fn init_tracing(verbose: bool, quiet: bool) {
    if quiet {
        return;
    }

    let filter = if verbose {
        "debug"
    } else {
        "info"
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(filter)),
        )
        .with_writer(std::io::stderr)
        .init();
}
```

Use stderr for diagnostics/logs, stdout for machine-readable report output.

---

## 8.2 Exit Codes Should Be Documented and Consistent

Current code uses:

```rust
ExitCode::SUCCESS
ExitCode::FAILURE
ExitCode::from(1)
```

I recommend explicit exit codes:

```rust
pub const EXIT_OK: u8 = 0;
pub const EXIT_VIOLATIONS: u8 = 1;
pub const EXIT_ERROR: u8 = 2;
pub const EXIT_TOOL_MISSING: u8 = 3;
pub const EXIT_THRESHOLD_FAILED: u8 = 4;
```

Then:

```rust
ExitCode::from(EXIT_VIOLATIONS)
```

Document them in `--help`.

---

## 8.3 Trait Return Types Are Inconsistent

Examples:

```rust
async fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError>;
async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
async fn scan_all(&self, path: &FilePath) -> LintResultList;
fn check_orphans(&self, files: &[String], root_dir: &str) -> Vec<LintResult>;
```

This makes orchestration harder and encourages error swallowing.

### Recommendation

Standardize around:

```rust
pub struct AuditOutput {
    pub violations: Vec<LintResult>,
    pub diagnostics: Vec<PipelineDiagnostic>,
}

#[async_trait]
pub trait IAuditSubsystem: Send + Sync {
    async fn audit(&self, target: &FilePath) -> Result<AuditOutput, PipelineError>;

    fn name(&self) -> &str;
}
```

This improves:

- testability,
- observability,
- error reporting,
- consistency across subsystems.

---

# 9. Reporting Output Fixes

## 9.1 JUnit XML Escaping

The provided `xml_escape()` appears to push raw characters:

```rust
'&' => escaped.push_str("&"),
'<' => escaped.push_str("<"),
'>' => escaped.push_str(">"),
```

If that is the actual code, it is invalid XML escaping.

### Fix

```rust
fn xml_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());

    for c in input.chars() {
        match c {
            '&' => escaped.push_str("&"),
            '<' => escaped.push_str("<"),
            '>' => escaped.push_str(">"),
            '"' => escaped.push_str("""),
            '\'' => escaped.push_str("'"),
            other => escaped.push(other),
        }
    }

    escaped
}
```

Important: `&` must be escaped first if doing replacement-based escaping. With character iteration, the above is safe.

---

## 9.2 SARIF Improvements

Current SARIF output includes only `start_line`.

Consider adding:

```rust
#[derive(serde::Serialize)]
struct SarifRegion {
    start_line: i64,
    start_column: i64,
}
```

And:

```rust
region: SarifRegion {
    start_line: std::cmp::max(1, r.line.value()),
    start_column: std::cmp::max(1, r.column.value()),
},
```

Also consider adding rule metadata:

```rust
#[derive(serde::Serialize)]
struct SarifReportingDescriptor {
    id: String,
    name: String,
    short_description: SarifMessage,
    help_uri: String,
}
```

This improves IDE and GitHub code scanning integration.

---

# 10. Rust-Specific Backend Improvements

## 10.1 Replace Closure Factories With Named Traits

Current:

```rust
pub type OrchestratorFactory = Arc<
    dyn Fn(shared::config_system::taxonomy_config_vo::ArchitectureConfig) -> CheckContext
        + Send
        + Sync,
>;
```

This works, but it is opaque.

### Better

```rust
pub trait OrchestratorFactory: Send + Sync {
    fn create(
        &self,
        config: ArchitectureConfig,
    ) -> CheckContext;
}
```

Then:

```rust
pub struct DefaultOrchestratorFactory;

impl OrchestratorFactory for DefaultOrchestratorFactory {
    fn create(&self, config: ArchitectureConfig) -> CheckContext {
        // Build context from config.
        todo!()
    }
}
```

Benefits:

- easier mocking,
- better stack traces,
- clearer documentation,
- easier extension.

---

## 10.2 Use `thiserror` for Domain Errors

You already use `thiserror` in some shared errors. Extend this consistently.

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliCommandError {
    #[error("path '{0}' does not exist")]
    PathNotFound(String),

    #[error("invalid path '{0}'")]
    InvalidPath(String),

    #[error("configuration error: {0}")]
    Config(String),

    #[error("analysis failed: {0}")]
    Analysis(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
```

Then map to exit codes at the surface:

```rust
impl From<CliCommandError> for ExitCode {
    fn from(err: CliCommandError) -> Self {
        eprintln!("Error: {err}");

        match err {
            CliCommandError::PathNotFound(_) => ExitCode::from(2),
            CliCommandError::InvalidPath(_) => ExitCode::from(2),
            CliCommandError::Config(_) => ExitCode::from(2),
            CliCommandError::Analysis(_) => ExitCode::from(2),
            CliCommandError::Io(_) => ExitCode::from(2),
        }
    }
}
```

---

## 10.3 Avoid `unwrap_or_default()` for User Input

Prefer explicit handling:

```rust
let path = FilePath::new(raw_path.clone())
    .map_err(|err| CliCommandError::InvalidPath(raw_path.clone()))?;
```

Use defaults only when the default is semantically valid.

---

# 11. Python and TypeScript Adapter Guidance

Since this tool orchestrates Python and TypeScript linters, the adapter layer is part of the backend security boundary.

## Python Adapter Rules

Use:

```python
subprocess.run(
    ["ruff", "check", "--output-format", "json", "--", path],
    capture_output=True,
    text=True,
    timeout=120,
    check=False,
)
```

Avoid:

```python
subprocess.run(f"ruff {path}", shell=True)
```

Validate output size:

```python
MAX_OUTPUT_BYTES = 10 * 1024 * 1024

if len(result.stdout) > MAX_OUTPUT_BYTES:
    raise RuntimeError("ruff output too large")
```

Pin dependencies:

```text
ruff==0.5.0
mypy==1.10.0
bandit==1.7.9
```

Prefer isolated installation:

```bash
python -m venv .lint-arwaky-venv
.lint-arwaky-venv/bin/pip install -r requirements-lint.txt
```

Avoid global `pip install` unless explicitly requested.

---

## TypeScript Adapter Rules

Use:

```ts
import { execFile } from "node:child_process";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

await execFileAsync("eslint", ["--format", "json", "--", path], {
  timeout: 120_000,
  maxBuffer: 10 * 1024 * 1024,
});
```

Avoid:

```ts
await exec(`eslint ${path}`);
```

Prefer local project binaries:

```ts
const eslintBin = path.join(process.cwd(), "node_modules", ".bin", "eslint");
```

Pin versions in `package.json`:

```json
{
  "devDependencies": {
    "eslint": "9.5.0",
    "prettier": "3.3.2",
    "typescript": "5.5.3"
  }
}
```

Avoid `sudo npm install -g` unless absolutely necessary.

---

# 12. SOLID Principles Assessment

## Single Responsibility Principle

Current violation:

- `CheckCommandsSurface` handles CLI surface, orchestration, filtering, formatting, and exit-code policy.

Fix:

- Move pipeline orchestration to an agent aggregate.
- Move formatting to formatter capabilities.
- Move filtering to a result-filter capability.
- Keep surfaces thin.

## Open/Closed Principle

Current violation:

- Report formats are hardcoded inside the surface.

Fix:

- Add `ReportFormatter` trait.
- Add new formats without modifying existing surfaces.

## Liskov Substitution Principle

Current risk:

- Some trait implementations cannot report errors because their contracts return plain `Vec` or `LintResultList`.

Fix:

- Use consistent `Result` or structured diagnostics so implementations can fail safely.

## Interface Segregation Principle

Current risk:

- `LayerDefinition` embeds naming, code-analysis, role, and orphan rules together.

This may become unwieldy.

Possible improvement:

```rust
pub struct LayerDefinition {
    pub common: CommonLayerRules,
    pub naming: Option<NamingLayerRules>,
    pub code_analysis: Option<CodeAnalysisLayerRules>,
    pub role: Option<RoleLayerRules>,
    pub orphan: Option<OrphanLayerRules>,
}
```

This allows features to evolve independently.

## Dependency Inversion Principle

Current risk:

- Surfaces depend directly on utility helpers and concrete orchestration behavior.

Fix:

- Surfaces should depend on aggregate contracts.
- Root layer wires concrete implementations.
- Utility functions should be consumed by capabilities/agents, not surfaces.

---

# 13. Recommended Remediation Plan

## Phase 1: Security and Correctness

1. Fix `init --global` to use bundled trusted docs only.
2. Fix MCP binary resolution to use absolute canonicalized paths.
3. Fix path filtering to use `Path::starts_with`.
4. Fix XML escaping.
5. Make maintenance/security/dependency commands return non-zero on errors or findings.
6. Stop using raw file collection for orphan detection.

## Phase 2: Error Handling

1. Introduce `PipelineError` and `PipelineDiagnostic`.
2. Change subsystem traits to return errors/diagnostics.
3. Stop using `unwrap_or_default()` for audit results.
4. Add timeouts to external lint adapters.
5. Add explicit exit codes.

## Phase 3: Architecture Refactor

1. Introduce `IAnalysisPipelineAggregate`.
2. Move `scan`, `scan_with_discovery`, and CI logic out of surfaces.
3. Introduce `ReportFormatter` capabilities.
4. Introduce `PathScope` filtering capability.
5. Make CLI surfaces thin wrappers.

## Phase 4: Performance

1. Build a shared workspace file index.
2. Cache file discovery per scan.
3. Add ignore-aware orphan scanning.
4. Add bounded concurrency for workspace scans.
5. Stream large reports to disk.
6. Add incremental caching based on file hashes.

## Phase 5: API Consistency

1. Unify command catalog.
2. Align CLI binary name in examples.
3. Implement or remove `--output-dir`, `--verbose`, `--quiet`.
4. Ensure `--filter` and `--format` work consistently across commands.
5. Add integration tests for CLI exit codes.

---

# 14. Key Code Fixes Summary

Below is a condensed set of high-value fixes.

## Fix Path Filtering

```rust
use std::path::{Path, PathBuf};

pub struct PathScope {
    root: PathBuf,
}

impl PathScope {
    pub fn new(root: &Path) -> std::io::Result<Self> {
        Ok(Self {
            root: root.canonicalize()?,
        })
    }

    pub fn contains(&self, candidate: &Path) -> bool {
        let candidate = candidate
            .canonicalize()
            .unwrap_or_else(|_| candidate.to_path_buf());

        candidate.starts_with(&self.root)
    }
}
```

## Fix MCP Binary Resolution

```rust
use std::path::PathBuf;

fn resolve_mcp_binary() -> Result<PathBuf, String> {
    if let Ok(explicit) = std::env::var("LINT_ARWAKY_MCP_BIN") {
        let path = PathBuf::from(explicit);

        if !path.is_file() {
            return Err(format!(
                "LINT_ARWAKY_MCP_BIN points to a non-file path: {}",
                path.display()
            ));
        }

        return path
            .canonicalize()
            .map_err(|err| format!("cannot canonicalize MCP binary: {err}"));
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-mcp");

            if sibling.is_file() {
                return sibling
                    .canonicalize()
                    .map_err(|err| format!("cannot canonicalize MCP binary: {err}"));
            }
        }
    }

    Err("lint-arwaky-mcp not found. Set LINT_ARWAKY_MCP_BIN.".to_string())
}
```

## Fix XML Escape

```rust
fn xml_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());

    for c in input.chars() {
        match c {
            '&' => escaped.push_str("&"),
            '<' => escaped.push_str("<"),
            '>' => escaped.push_str(">"),
            '"' => escaped.push_str("""),
            '\'' => escaped.push_str("'"),
            other => escaped.push(other),
        }
    }

    escaped
}
```

## Fix Orphan File Collection

```rust
let all_source_files: Vec<String> =
    shared::common::collect_all_source_files(&scan_root)
        .iter()
        .map(|f| f.value.clone())
        .collect();
```

## Fix Threshold Comparison

```rust
let below_threshold = score.value() < threshold.value() as f64;
```

## Fix Dynamic Adapter Listing

```rust
pub fn handle_adapters(external_lint: Arc<dyn IExternalLintAggregate>) -> ExitCode {
    println!("External lint adapters:");

    for adapter in external_lint.adapter_names() {
        println!("  - {adapter}");
    }

    ExitCode::SUCCESS
}
```

## Fix Watch Signal Handler

Current handler prints inside the signal handler:

```rust
ctrlc::set_handler(move || {
    eprintln!("\nStopping watcher...");
    r.store(false, Ordering::SeqCst);
})
```

Better:

```rust
ctrlc::set_handler(move || {
    r.store(false, Ordering::SeqCst);
})
```

Then print after the watcher returns:

```rust
let exit_code = watch_aggregate.run(config, running);

println!("Watcher stopped.");

exit_code
```

---

# Final Recommendation

The biggest improvement will come from introducing a proper **analysis pipeline aggregate**:

```text
CLI Surface
   ↓
IAnalysisPipelineAggregate
   ↓
Agent Orchestrator
   ↓
Capabilities/Subsystems
   ↓
Utility Layer
```

That single refactor will resolve many of the current architectural, testing, scalability, and maintainability issues.

If you want, I can next produce a concrete patch plan file-by-file for:

1. `surface_check_command.rs`
2. `surface_check_action.rs`
3. `surface_common_command.rs`
4. `root_cli_container.rs`
5. `shared/cli-commands` contracts

with the exact refactored Rust code for the new pipeline aggregate.
