# Plan: Ratatui TUI — File Browser Style (Ranger-like)

## Goal
TUI interaktif mirip **ranger** (terminal file manager) untuk `lint-arwaky`. Path project dimasukkan sekali di awal, lalu user bisa:
- Navigasi folder structure (crates/, packages/, modules/)
- Lihat detected AES layers per file/folder (warna-coded)
- Jalanin perintah lint (`check`, `scan`, `fix`, dll) di file/folder yang sedang dipilih
- Preview hasil lint di panel sebelah

## Current State
Dialoguer TUI (`surface_tui_command.rs`) — flat menu, shell-out ke CLI binary.

---

## Source Code Context

### 1. Current TUI Entry Point (`crates/root_tui_main_entry.rs`)

```rust
// PURPOSE: main entry point for lint-arwaky-tui — interactive TUI launcher
use std::process::ExitCode;

use lint_arwaky::cli_commands::surface_tui_command::TuiCommandSurface;

pub struct TuiMainEntry {}

fn main() -> ExitCode {
    TuiCommandSurface::run()
}
```

### 2. Current TUI Implementation (`crates/cli-commands/src/surface_tui_command.rs`)

```rust
// PURPOSE: TuiCommandSurface — interactive menu-driven TUI for lint-arwaky-tui binary
use console::{style, Term};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use std::process::ExitCode;

pub struct TuiCommandSurface;

impl TuiCommandSurface {
    pub fn run() -> ExitCode {
        run_tui_loop()
    }
}

fn cli_binary() -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let sibling = dir.join("lint-arwaky-cli");
            if sibling.exists() {
                return sibling.to_string_lossy().to_string();
            }
        }
    }
    "lint-arwaky-cli".to_string()
}

fn print_header(term: &Term) {
    let _ = term.clear_screen();
    println!(
        "{}",
        style("╔══════════════════════════════════════════════════╗")
            .cyan()
            .bold()
    );
    println!(
        "{}  {}  {}",
        style("║").cyan().bold(),
        style("  Lint Arwaky TUI  -- Code Quality Gateway")
            .white()
            .bold(),
        style("║").cyan().bold()
    );
    println!(
        "{}",
        style("╚══════════════════════════════════════════════════╝")
            .cyan()
            .bold()
    );
    println!();
}

fn ask_path(prompt: &str, default: &str) -> String {
    match Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_string())
        .interact_text()
    {
        Ok(input) => input,
        Err(_) => default.to_string(),
    }
}

fn run_cmd(args: &[&str]) {
    let cli = cli_binary();
    println!(
        "\n{} {} {}\n",
        style(">").green().bold(),
        style("Running:").dim(),
        style(format!("{} {}", cli, args.join(" "))).yellow()
    );
    let status = std::process::Command::new(&cli).args(args).status();
    match status {
        Ok(s) if s.success() => {
            println!(
                "\n{} {}",
                style("OK").green().bold(),
                style("Done.").green()
            )
        }
        Ok(s) => {
            let code = match s.code() {
                Some(c) => c,
                None => -1,
            };
            println!("\n{} Exit code: {}", style("FAIL").red().bold(), code)
        }
        Err(e) => println!("\n{} Failed to run binary: {e}", style("FAIL").red().bold()),
    }
}

fn pause() {
    let _ = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt(style("Press Enter to return to menu").dim().to_string())
        .default(String::new())
        .allow_empty(true)
        .interact_text();
}

#[derive(Clone, Copy, PartialEq)]
enum MenuKind {
    Action,
    Separator,
}

struct MenuItem {
    label: &'static str,
    id: &'static str,
    kind: MenuKind,
}

const MENU: &[MenuItem] = &[
    MenuItem { label: "[check]   AES self-lint audit", id: "check", kind: MenuKind::Action },
    MenuItem { label: "[scan]    Full multi-adapter scan", id: "scan", kind: MenuKind::Action },
    MenuItem { label: "[fix]     Apply safe automatic fixes", id: "fix", kind: MenuKind::Action },
    MenuItem { label: "[ci]      CI mode (exit 1 if score < N)", id: "ci", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    MenuItem { label: "[orphan]        Check orphan files (AES501-506)", id: "orphan", kind: MenuKind::Action },
    MenuItem { label: "[security]      Vulnerability scan", id: "security", kind: MenuKind::Action },
    MenuItem { label: "[duplicates]    Duplication detection", id: "duplicates", kind: MenuKind::Action },
    MenuItem { label: "[dependencies]  Library vulnerability scan", id: "dependencies", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    MenuItem { label: "[watch]  Watch and lint on changes", id: "watch", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    MenuItem { label: "[doctor]      Diagnose environment", id: "doctor", kind: MenuKind::Action },
    MenuItem { label: "[init]        Create default config", id: "init", kind: MenuKind::Action },
    MenuItem { label: "[install]     Install adapter deps", id: "install", kind: MenuKind::Action },
    MenuItem { label: "[mcp-config]  Print MCP config", id: "mcp-config", kind: MenuKind::Action },
    MenuItem { label: "[config-show] Show active config", id: "config-show", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    MenuItem { label: "[install-hook]   Install git pre-commit", id: "install-hook", kind: MenuKind::Action },
    MenuItem { label: "[uninstall-hook] Remove git pre-commit", id: "uninstall-hook", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    MenuItem { label: "[adapters]  List active adapters", id: "adapters", kind: MenuKind::Action },
    MenuItem { label: "[version]   Show version", id: "version", kind: MenuKind::Action },
    MenuItem { label: "", id: "", kind: MenuKind::Separator },
    MenuItem { label: "Exit", id: "exit", kind: MenuKind::Action },
];

pub fn run_tui_loop() -> ExitCode {
    let term = Term::stdout();
    loop {
        print_header(&term);
        let selectable: Vec<(usize, &MenuItem)> = MENU
            .iter()
            .enumerate()
            .filter(|(_, m)| m.kind == MenuKind::Action)
            .collect();
        let display_labels: Vec<&str> = selectable.iter().map(|(_, m)| m.label).collect();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select command")
            .items(&display_labels)
            .default(0)
            .interact_on_opt(&term);
        let pick = match selection {
            Ok(Some(i)) => i,
            Ok(None) | Err(_) => break,
        };
        let item = selectable[pick].1;
        println!();
        match item.id {
            "exit" => break,
            "check" => { let p = ask_path("Path", "."); run_cmd(&["check", &p]); pause(); }
            "scan" => { let p = ask_path("Path", "."); run_cmd(&["scan", &p]); pause(); }
            "fix" => { let p = ask_path("Path", "."); run_cmd(&["fix", &p]); pause(); }
            "ci" => {
                let p = ask_path("Path", ".");
                let t: String = match Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Threshold")
                    .default("80".to_string())
                    .interact_text()
                { Ok(input) => input, Err(_) => "80".to_string() };
                run_cmd(&["ci", &p, "--threshold", &t]);
                pause();
            }
            "orphan" => { let p = ask_path("Path", "."); run_cmd(&["orphan", &p]); pause(); }
            "security" => { let p = ask_path("Path", "."); run_cmd(&["security", &p]); pause(); }
            "duplicates" => { let p = ask_path("Path", "."); run_cmd(&["duplicates", &p]); pause(); }
            "dependencies" => { let p = ask_path("Path", "."); run_cmd(&["dependencies", &p]); pause(); }
            "watch" => { let p = ask_path("Path", "."); run_cmd(&["watch", &p]); pause(); }
            "doctor" => { run_cmd(&["doctor"]); pause(); }
            "init" => { run_cmd(&["init"]); pause(); }
            "install" => { run_cmd(&["install"]); pause(); }
            "mcp-config" => { run_cmd(&["mcp-config"]); pause(); }
            "config-show" => { run_cmd(&["config-show"]); pause(); }
            "install-hook" => { run_cmd(&["install-hook"]); pause(); }
            "uninstall-hook" => { run_cmd(&["uninstall-hook"]); pause(); }
            "adapters" => { run_cmd(&["adapters"]); pause(); }
            "version" => { run_cmd(&["version"]); pause(); }
            _ => {}
        }
    }
    println!("\n{}", style("Bye!").cyan().bold());
    ExitCode::SUCCESS
}
```

---

## Source Code: Existing Taxonomy & Contracts (shared crate)

### 3. Layer Detection Helper (`shared/src/import-rules/taxonomy_path_helper.rs`)

```rust
// PURPOSE: taxonomy_path_helper — pure utility functions for path matching and layer extraction
use std::path::Path;

pub fn extract_layer_from_prefix(filename: &str) -> Option<String> {
    let stem = Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    const PREFIX_MAP: &[(&str, &str)] = &[
        ("taxonomy_", "taxonomy"),
        ("contract_", "contract"),
        ("capabilities_", "capabilities"),
        ("infrastructure_", "infrastructure"),
        ("agent_", "agent"),
        ("surface_", "surfaces"),
        ("root_", "root"),
    ];

    for &(prefix, layer) in PREFIX_MAP {
        if stem.starts_with(prefix) {
            return Some(layer.to_string());
        }
    }
    None
}

pub fn get_relative_path(file_path: &str, root_dir: &str) -> String {
    let normalized_file = match Path::new(file_path)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => file_path.replace('\\', "/"),
    };
    let normalized_root = match Path::new(root_dir)
        .canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        Ok(p) => p,
        Err(_) => root_dir.trim_end_matches('/').replace('\\', "/"),
    };
    if normalized_file.starts_with(&normalized_root) {
        normalized_file[normalized_root.len()..]
            .trim_start_matches('/')
            .to_string()
    } else {
        normalized_file
    }
}
```

### 4. Severity VO (`shared/src/common/taxonomy_severity_vo.rs`)

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

### 5. LintResult VO (`shared/src/cli-commands/taxonomy_result_vo.rs`)

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

macro_rules! lint_result_list_wrapper {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub values: Vec<$item>,
        }
        impl $name {
            pub fn new(value: Vec<$item>) -> Self { Self { values: value } }
            pub fn iter(&self) -> std::slice::Iter<'_, $item> { self.values.iter() }
            pub fn len(&self) -> usize { self.values.len() }
            pub fn is_empty(&self) -> bool { self.values.is_empty() }
            pub fn push(&mut self, item: $item) { self.values.push(item); }
            pub fn append(&mut self, item: $item) { self.values.push(item); }
        }
    };
}

lint_result_list_wrapper!(LintResultList, LintResult);
```

### 6. ICodeAnalysisAggregate Contract (`shared/src/code-analysis/contract_code_analysis_aggregate.rs`)

```rust
// PURPOSE: ICodeAnalysisAggregate — aggregate trait for code-analysis checks (AES301–AES305)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;

pub trait ICodeAnalysisAggregate: Send + Sync {
    fn run_code_analysis(&self, project_root: &str) -> LintResultList;
    fn run_code_analysis_dir(&self, src_dir: &str) -> LintResultList;
    fn run_code_analysis_path(&self, path: &str) -> Vec<LintResult>;
    fn calc_score(&self, results: &[LintResult]) -> f64;
    fn check_critical(&self, results: &[LintResult]) -> bool;
    fn format_report(&self, results: &LintResultList, project_root: &str) -> String;
}
```

### 7. IImportRunnerAggregate Contract (`shared/src/import-rules/contract_import_runner_aggregate.rs`)

```rust
// PURPOSE: IImportRunnerAggregate — contract for import-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IImportRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

### 8. INamingRunnerAggregate Contract (`shared/src/naming-rules/contract_naming_runner_aggregate.rs`)

```rust
// PURPOSE: INamingRunnerAggregate — contract for naming-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait INamingRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

### 9. IRoleRunnerAggregate Contract (`shared/src/role-rules/contract_role_runner_aggregate.rs`)

```rust
// PURPOSE: IRoleRunnerAggregate — contract for role-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IRoleRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

### 10. IExternalLintAggregate Contract (`shared/src/external-lint/contract_external_lint_aggregate.rs`)

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

### 11. IOrphanAggregate Contract (`shared/src/orphan-detector/contract_orphan_aggregate.rs`)

```rust
// PURPOSE: IOrphanAggregate — aggregate trait bundling all orphan detection protocols
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use std::collections::HashSet;

pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String>;
    fn check_orphans(
        &self,
        layer_detector: &dyn ILayerDetectionAggregate,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult>;
}
```

### 12. FixCommandsSurface (`crates/cli-commands/src/surface_fix_command.rs`)

```rust
// PURPOSE: FixCommandsSurface — CLI surface for auto-fix operations
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

pub struct FixCommandsSurface {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub fix_orchestrator_factory:
        Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
}

impl FixCommandsSurface {
    pub fn new(
        code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
        fix_orchestrator_factory: Arc<
            dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
        >,
    ) -> Self {
        Self {
            code_analysis_linter,
            fix_orchestrator_factory,
        }
    }

    pub fn fix(&self, path: &str) {
        let canonical = match PathBuf::from(path).canonicalize() {
            Ok(p) => p,
            Err(_) => PathBuf::from(path),
        };
        let project_path = FilePath {
            value: canonical.to_string_lossy().to_string(),
        };
        self.run_fix(project_path, false);
    }

    pub fn run_fix(&self, project_path: FilePath, dry_run: bool) {
        if dry_run {
            println!("[DRY-RUN] Previewing fixes for {}...", project_path.value);
        } else {
            println!("Applying safe fixes to {}...", project_path.value);
        }

        let results = self
            .code_analysis_linter
            .run_code_analysis(&project_path.value);
        println!("Found {} violations before fix", results.len());

        let fix_orch = (self.fix_orchestrator_factory)(dry_run);
        let fix_result = fix_orch.execute(&project_path);

        println!("{}", fix_result.output.value);

        if !dry_run {
            let after_results = self
                .code_analysis_linter
                .run_code_analysis(&project_path.value);
            let fixed_count = results.len().saturating_sub(after_results.len());
            println!(
                "Fixed {} violations ({} remaining)",
                fixed_count,
                after_results.len()
            );
            println!("Fix complete.");
        } else {
            println!("Dry-run complete — no changes applied.");
        }
    }
}
```

### 13. WatchCommandsSurface (`crates/cli-commands/src/surface_watch_command.rs`)

```rust
// PURPOSE: WatchCommandsSurface — CLI surface for file watching with auto-lint on changes
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

pub struct WatchCommandsSurface {}

impl Default for WatchCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl WatchCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn handle_watch(watch_aggregate: Arc<dyn IWatchAggregate>, path: Option<String>) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let config = WatchConfig::from_path(root);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    if let Err(e) = ctrlc::set_handler(move || {
        eprintln!("\nStopping watcher...");
        r.store(false, Ordering::SeqCst);
    }) {
        eprintln!("[error] failed to set Ctrl+C handler: {}", e);
        return ExitCode::FAILURE;
    }

    watch_aggregate.run(config, running)
}
```

### 14. Common VOs (`shared/src/common/taxonomy_common_vo.rs` — excerpt)

```rust
// Key types used by TUI:

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct BooleanVO { pub value: bool }
impl BooleanVO { pub fn new(value: bool) -> Self { Self { value } } }

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct LineNumber { pub value: i64 }
impl LineNumber { pub fn new(value: i64) -> Self { Self { value } } }

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct ColumnNumber { pub value: i64 }
impl ColumnNumber { pub fn new(value: i64) -> Self { Self { value } } }

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct Count { pub value: i64 }
impl Count { pub fn new(value: i64) -> Self { Self { value } } }

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct Score { pub value: f64 }
impl Score { pub fn new(value: f64) -> Self { Self { value } } }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PatternList { pub values: Vec<String> }
impl PatternList {
    pub fn new(value: impl IntoPatternListValues) -> Self {
        Self { values: value.into_pattern_list_values() }
    }
}
```

### 15. Shared Lib Module Structure (`crates/shared/src/lib.rs`)

```rust
// PURPOSE: shared — all taxonomy types, contract traits, and shared definitions
// No dependencies on other feature crates — this is the foundation layer.

#[path = "common/mod.rs"]
pub mod common;
pub use common::*;

#[path = "auto-fix/mod.rs"]
pub mod auto_fix;
#[path = "cli-commands/mod.rs"]
pub mod cli_commands;
#[path = "code-analysis/mod.rs"]
pub mod code_analysis;
#[path = "config-system/mod.rs"]
pub mod config_system;
#[path = "external-lint/mod.rs"]
pub mod external_lint;
#[path = "file-system/mod.rs"]
pub mod file_system;
#[path = "file-watch/mod.rs"]
pub mod file_watch;
#[path = "git-hooks/mod.rs"]
pub mod git_hooks;
#[path = "import-rules/mod.rs"]
pub mod import_rules;
#[path = "mcp-server/mod.rs"]
pub mod mcp_server;
#[path = "naming-rules/mod.rs"]
pub mod naming_rules;
#[path = "orphan-detector/mod.rs"]
pub mod orphan_detector;
#[path = "project-setup/mod.rs"]
pub mod project_setup;
#[path = "role-rules/mod.rs"]
pub mod role_rules;
#[path = "source-parsing/mod.rs"]
pub mod source_parsing;
```

---

## Source Code: Workspace Config (Full)

### 16. Root `Cargo.toml` (complete relevant sections)

```toml
[workspace]
resolver = "2"
members = ["crates/*"]

[workspace.lints.clippy]
result_large_err = "warn"
manual_unwrap_or_default = "allow"
manual_unwrap_or = "allow"
unnecessary_result_map_or_else = "allow"
unnecessary_option_map_or_else = "allow"
needless_lifetimes = "allow"

[workspace.dependencies]
# Workspace member crates
shared = { package = "shared-lint-arwaky", path = "crates/shared" }
source_parsing = { package = "source_parsing-lint-arwaky", path = "crates/source-parsing" }
naming_rules = { package = "naming_rules-lint-arwaky", path = "crates/naming-rules" }
import_rules = { package = "import_rules-lint-arwaky", path = "crates/import-rules" }
code_analysis = { package = "code_analysis-lint-arwaky", path = "crates/code-analysis" }
auto_fix = { package = "auto_fix-lint-arwaky", path = "crates/auto-fix" }
cli_commands = { package = "cli_commands-lint-arwaky", path = "crates/cli-commands" }
config_system = { package = "config_system-lint-arwaky", path = "crates/config-system" }
file_watch = { package = "file_watch-lint-arwaky", path = "crates/file-watch" }
git_hooks = { package = "git_hooks-lint-arwaky", path = "crates/git-hooks" }
external_lint = { package = "external_lint-lint-arwaky", path = "crates/external-lint" }
mcp_server = { package = "mcp_server-lint-arwaky", path = "crates/mcp-server" }
orphan_detector = { package = "orphan_detector-lint-arwaky", path = "crates/orphan-detector" }
project_setup = { package = "project_setup-lint-arwaky", path = "crates/project-setup" }
maintenance = { package = "maintenance-lint-arwaky", path = "crates/maintenance" }
role_rules = { package = "role_rules-lint-arwaky", path = "crates/role-rules" }

# External dependencies (pinned versions)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
regex = "1.10"
tokio = { version = "1.52.3", features = ["full"] }
notify = "6"
notify-debouncer-mini = "0.4"
once_cell = "1.21.4"
async-trait = "0.1.89"
thiserror = "1.0.52"
anyhow = "1.0.102"
serde_yaml = "0.9.34"
toml = "1.1.2"
clap = { version = "4.6.1", features = ["derive"] }
reqwest = { version = "0.13.4", features = ["blocking"] }
chrono = "0.4.44"
rmcp = { version = "0.16", features = ["server", "transport-io"] }
rand = "0.10.1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
rustsec = "0.33"
dirs = "6.0"
dialoguer = "0.11"
console = "0.15"
ctrlc = "3.4"
futures = "0.3"
# ← TO BE ADDED for TUI upgrade:
# ratatui = "0.29"
# crossterm = "0.28"

[package]
name = "lint_arwaky-arwaky"
version = "1.10.43"
edition = "2021"
description = "Autonomous code quality and architecture enforcement for AI agents and developers, enforcing 27 Agentic Engineering System (AES) rules."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"

[lib]
name = "lint_arwaky"
path = "crates/lib.rs"

[[bin]]
name = "lint-arwaky-cli"
path = "crates/root_cli_main_entry.rs"

[[bin]]
name = "lint-arwaky-mcp"
path = "crates/root_mcp_main_entry.rs"

[[bin]]
name = "lint-arwaky-tui"
path = "crates/root_tui_main_entry.rs"

[dependencies]
shared.workspace = true
source_parsing.workspace = true
naming_rules.workspace = true
import_rules.workspace = true
code_analysis.workspace = true
auto_fix.workspace = true
cli_commands.workspace = true
config_system.workspace = true
file_watch.workspace = true
git_hooks.workspace = true
external_lint.workspace = true
mcp_server.workspace = true
orphan_detector.workspace = true
project_setup.workspace = true
maintenance.workspace = true
role_rules.workspace = true
# ... other deps ...

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = "symbols"
panic = "abort"
debug = false
incremental = false
```

---

## UX Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  Path: /home/project/lint-arwaky                      [Ctrl+Q] Quit │
├──────────┬──────────────────┬──────────────────────────────────────┤
│  crates/ │  ► cli-commands/ │  File Preview / Lint Results        │
│  docs/   │    src/          │                                      │
│  shared/ │      ▼ surface_  │  AES203: OK                         │
│  ...     │        check_    │  AES204: OK                         │
│          │        scan_     │  Violations: 0                      │
│          │        tui_      │                                      │
│          │        fix_      │  [F1] Check  [F2] Scan  [F3] Fix    │
│          │      infrastruc… │  [F4] Watch  [F5] Doctor            │
│          │    Cargo.toml    │                                      │
│          │  src/            │                                      │
│          │  tests/          │                                      │
│          │                  │                                      │
├──────────┴──────────────────┴──────────────────────────────────────┤
│  c:check  s:scan  f:fix  t:ci  w:watch  o:orphan  d:doctor  i:init│  ← Shortcut bar row 1
│  I:install  m:mcp  C:config  H:hook  U:unhook  a:adapter  v:version │  ← Shortcut bar row 2
│  ^S:security  ^D:duplicates  ^P:dependencies  ?:help  q:quit       │  ← Shortcut bar row 3 (Ctrl+)
│  Status: Ready  |  Selected: crates/cli-commands/src/  |  0 viol.  │  ← Status bar
└─────────────────────────────────────────────────────────────────────┘

> **Shortcut bar selalu kelihatan di layar — 2 baris di bawah — user gak perlu inget shortcut.**
> Shortcut bar juga **context-sensitive**: waktu lagi preview hasil lint, baris 1 berubah jadi action terkait (R:rerun, F:fix, E:export, Esc:back).
```

## CLI Command Coverage

| # | CLI Command | Shortcut | Flags di TUI | Status |
|---|-------------|----------|-------------|--------|
| 1 | `check` | `c` | `--git-diff` toggle | ✅ |
| 2 | `scan` | `s` | — | ✅ |
| 3 | `fix` | `f` | `--dry-run` toggle | ✅ |
| 4 | `ci` | `t` | `--threshold <N>` dialog | ✅ |
| 5 | `orphan` | `o` | — | ✅ |
| 6 | `security` | `Ctrl+S` | — | ✅ |
| 7 | `duplicates` | `Ctrl+D` | — | ✅ |
| 8 | `dependencies` | `Ctrl+P` | — | ✅ |
| 9 | `watch` | `w` | — | ✅ |
| 10 | `doctor` | `d` | — | ✅ |
| 11 | `init` | `i` | `--global` toggle | ✅ |
| 12 | `install` | `I` | `--sudo` toggle | ✅ |
| 13 | `mcp-config` | `m` | `--client` dropdown | ✅ |
| 14 | `config-show` | `C` | — | ✅ |
| 15 | `install-hook` | `H` | — | ✅ |
| 16 | `uninstall-hook` | `U` | — | ✅ |
| 17 | `adapters` | `a` | — | ✅ |
| 18 | `version` | `v` | — | ✅ |

**Kesimpulan: Semua 18 CLI commands punya shortcut + coverage penuh.**

### 3-panel layout (ranger-style):
| Panel | Content |
|-------|---------|
| **Left** (narrow) | Parent directories / drive list |
| **Middle** (main) | Current directory contents + layer badges |
| **Right** (detail) | File preview / lint results / action output |

---

## Layer Badges (di panel tengah)
Setiap file/folder dikasih badge layer AES:

```
  [taxonomy]  taxonomy_path_vo.rs
  [contract]  contract_parser_port.rs
  [infra]     infrastructure_scanner.rs
  [agent]     agent_orchestrator.rs
  [surface]   surface_check_command.rs
  [root]      root_container.rs
  [---]       main.rs
```

Warna per layer:
- `taxonomy` → cyan
- `contract` → blue
- `capabilities` → magenta
- `infrastructure` → yellow
- `agent` → green
- `surfaces` → red
- `root` → white bold

---

## Actions (pada file/folder terseleksi)

| Key | Aksi | Deskripsi |
|-----|------|-----------|
| `Enter` | Buka folder / preview file | Navigasi ke folder atau preview file |
| `l` | Buka folder | Sama kaya Enter |
| `h` | Back | Ke parent directory |
| `j` / `k` | Navigasi | Gerak ke atas/bawah |
| `gg` / `G` | Lompat | Ke awal/akhir list |
| `/` | Search | Cari file/folder |
| `c` | Check | Jalankan `check` di selection |
| `s` | Scan | Jalankan `scan` di selection |
| `f` | Fix | Jalankan `fix` di selection |
| `w` | Watch | Jalankan `watch` di selection |
| `o` | Orphan | Cek orphan di selection |
| `Ctrl+S` | Security | Scan security |
| `Ctrl+D` | Duplicates | Deteksi duplikasi |
| `Ctrl+P` | Dependencies | Scan dependency |
| `t` | CI | CI mode (threshold) — tanya threshold value |
| `d` | Doctor | Diagnosa environment |
| `i` | Init | Init config |
| `I` | Install | Install adapter |
| `m` | MCP Config | Print MCP config |
| `C` | Config Show | Lihat config aktif |
| `H` | Install Hook | Install git hook |
| `U` | Uninstall Hook | Remove git hook |
| `a` | Adapters | List adapters |
| `v` | Version | Show version |
| `q` / `Ctrl+Q` | Quit | Keluar |
| `?` | Help | Tampilkan shortcut |
| Mouse click | Pilih item | Klik kiri untuk select |
| Scroll | Scroll panel | Scroll wheel |

---

## Flags (Command Modifiers)

Beberapa CLI command punya flags. Di TUI, flags bisa di-set lewat dialog atau toggle:

| Command | Flag | TUI Behavior |
|---------|------|-------------|
| `check` | `--git-diff` | Toggle di preview panel sebelum pencet `c` |
| `fix` | `--dry-run` | Toggle: [X] Dry-run sebelum pencet `f` |
| `ci` | `--threshold <N>` | Dialog input angka threshold saat pencet `t` |
| `init` | `--global` | Toggle: [ ] Global config |
| `install` | `--sudo` | Toggle: [X] Use sudo |
| `mcp-config` | `--client <name>` | Dropdown: claude/cursor/windsurp/copilot |

Flags disimpan sementara di `AppState.action_flags` dan bisa diubah sebelum eksekusi.

## Path Input (Startup)

Saat pertama kali dijalankan:
```
┌────────────────────────────────────────────┐
│  Enter project path:                      │
│  [/home/project/lint-arwaky]              │
│                                            │
│  [OK]  [Use current dir]  [Browse...]      │
└────────────────────────────────────────────┘
```
- Bisa ketik manual
- Bisa browse pake file dialog
- Bisa pake current directory (default)
- Path ini jadi root — navigasi gak bisa ke atas dari root

---

## Architecture (AES Layers)

```
crates/tui/src/
  taxonomy_state_vo.rs              ← AppState, PanelState, FileEntry
  taxonomy_file_entry_vo.rs         ← FileEntry with layer detection, metadata
  taxonomy_tui_event_vo.rs          ← NavigationEvent, ActionEvent
  contract_file_system_port.rs      ← IFileSystemPort — read dir, file info
  contract_lint_executor_port.rs    ← ILintExecutorPort — execute lint actions
  contract_view_port.rs             ← IViewPort — render trait per panel
  capabilities_file_browser.rs      ← Directory listing, sorting, filtering
  capabilities_layer_detector.rs    ← Detect AES layer from filename (reuse taxonomy_path_helper)
  capabilities_lint_executor.rs     ← Call domain libs directly (no subprocess)
  capabilities_action_handler.rs    ← Map key events to actions
  infrastructure_crossterm_provider.rs ← Terminal, raw mode, mouse capture, events
  agent_tui_orchestrator.rs         ← Main loop: event → state → render (3 panels)
  surface_file_panel.rs             ← Middle panel: file list + layer badges
  surface_preview_panel.rs          ← Right panel: preview / results
  surface_tree_panel.rs             ← Left panel: directory tree
  surface_path_dialog.rs            ← Startup path input dialog
  surface_help_overlay.rs           ← Help screen overlay
  root_tui_container.rs             ← DI container wiring
```

---

## Implementation Phases

### Phase 1 — File Browser Core
1. Create `crates/tui/` scaffold + `Cargo.toml`
2. Add `ratatui` + `crossterm` workspace deps
3. `taxonomy_state_vo.rs` — `AppState` with `current_path`, `selected_index`, `entries: Vec<FileEntry>`, `panel_focus`
4. `taxonomy_file_entry_vo.rs` — `FileEntry { name, path, is_dir, layer, violations_count }`
5. `contract_file_system_port.rs` — `IFileSystemPort { read_dir, is_file, metadata }`
6. `capabilities_layer_detector.rs` — wrap `taxonomy_path_helper::extract_layer_from_prefix`
7. `capabilities_file_browser.rs` — `list_directory(path) → Vec<FileEntry>`, sorting (dirs first, alpha)
8. `infrastructure_crossterm_provider.rs` — terminal init, raw mode, mouse capture, event polling
9. `surface_file_panel.rs` — render file list with layer badges
10. `agent_tui_orchestrator.rs` — basic loop: render file list, `j`/`k` navigate, `Enter`/`l` open dir, `h` go up
11. `root_tui_container.rs` — DI wiring
12. Update `root_tui_main_entry.rs`, `Cargo.toml` workspace members + bins
13. **Verify**: `check .` 0 violations, navigate folder structure

### Phase 2 — Path Dialog + Preview Panel
1. `surface_path_dialog.rs` — startup path input (text input + browse + current dir)
2. `surface_preview_panel.rs` — basic file preview (syntax highlight optional, plaintext first)
3. `surface_tree_panel.rs` — left panel: directory tree with expand/collapse
4. 3-panel layout: tree | files | preview
5. Mouse click: click on file to select, click on panel to focus
6. Tab/shift-tab to cycle panel focus
7. **Verify**: browse, preview, mouse click

### Phase 3 — Lint Actions (No Subprocess)
1. `contract_lint_executor_port.rs` — `ILintExecutorPort { check, scan, fix, watch, ... }`
2. `capabilities_lint_executor.rs` — call domain functions from `cli-commands`, `code-analysis`, etc.
3. `capabilities_action_handler.rs` — map `c`, `s`, `f`, `w`, etc. to executor calls
4. Right panel shows streaming output / results table when action runs
5. Progress bar for long operations (check, scan)
6. **Verify**: All actions work on selected file/folder

### Phase 4 — Polish
1. Search (`/`) — fuzzy find files in current dir
2. Sort options: by name, by layer, by violations count
3. `gg`/`G` jump to top/bottom
4. `?` help overlay
5. `surface_help_overlay.rs` — scrollable keybindings reference
6. Error handling: inline error display, retry option
7. Color theme consistency
8. **Verify**: `check .` 0 violations, `cargo test` all pass

---

## Dependencies Baru (root `Cargo.toml`)

```toml
ratatui = "0.29"
crossterm = "0.28"
```

---

## Files Changed

| File | Action |
|------|--------|
| `Cargo.toml` | +`ratatui`, +`crossterm` workspace deps; +`crates/tui` member; +`lint-arwaky-tui` bin (update) |
| `crates/root_tui_main_entry.rs` | Update ke `tui::root_tui_container::TuiContainer::run()` |
| `crates/cli-commands/src/surface_tui_command.rs` | No change (legacy) |
| `shared/src/lib.rs` | No change (tidak perlu shared/tui/) |

## New Files (~22 files)

| File | Layer | Desc |
|------|-------|------|
| `crates/tui/Cargo.toml` | — | Package manifest |
| `crates/tui/src/lib.rs` | root | Re-exports |
| `crates/tui/src/taxonomy_state_vo.rs` | taxonomy | AppState |
| `crates/tui/src/taxonomy_file_entry_vo.rs` | taxonomy | FileEntry |
| `crates/tui/src/taxonomy_tui_event_vo.rs` | taxonomy | Events |
| `crates/tui/src/contract_file_system_port.rs` | contract | IFileSystemPort |
| `crates/tui/src/contract_lint_executor_port.rs` | contract | ILintExecutorPort |
| `crates/tui/src/contract_view_port.rs` | contract | IViewPort |
| `crates/tui/src/capabilities_file_browser.rs` | capabilities | Dir listing |
| `crates/tui/src/capabilities_layer_detector.rs` | capabilities | Layer detection |
| `crates/tui/src/capabilities_lint_executor.rs` | capabilities | Execute actions |
| `crates/tui/src/capabilities_action_handler.rs` | capabilities | Key→action mapping |
| `crates/tui/src/infrastructure_crossterm_provider.rs` | infrastructure | Terminal + events |
| `crates/tui/src/agent_tui_orchestrator.rs` | agent | Main event loop |
| `crates/tui/src/surface_file_panel.rs` | surfaces | Middle: file list |
| `crates/tui/src/surface_preview_panel.rs` | surfaces | Right: preview |
| `crates/tui/src/surface_tree_panel.rs` | surfaces | Left: tree view |
| `crates/tui/src/surface_path_dialog.rs` | surfaces | Startup dialog |
| `crates/tui/src/surface_help_overlay.rs` | surfaces | Help overlay |
| `crates/tui/src/root_tui_container.rs` | root | DI container |

---

## Key Design Decisions

1. **Ranger-style 3-panel**: tree | file list | preview — familiar UX untuk power user terminal.

2. **Layer badges on files**: Setiap file langsung keliatan layer AES-nya dari warna badge. Bikin developer sadar arsitektur tanpa harus mikir.

3. **Actions on selected item**: Bukan milih dari menu, tapi select file/folder dulu, baru pencet shortcut. Mirip ranger: select → action.

4. **No subprocess**: All actions call Rust library functions directly via `capabilities_lint_executor`.

5. **Mouse support**: Click to select, click to focus panel, scroll wheel, click on action buttons di preview panel.

6. **Layer detector reuse**: `capabilities_layer_detector` wraps `taxonomy_path_helper::extract_layer_from_prefix` dari shared — zero duplication.

7. **Always-visible shortcuts**: 3 baris shortcut di bottom screen — user NEVER perlu inget shortcut. Context-sensitive: baris 1 berubah sesuai konteks (file browsing vs hasil lint). Ini mencegah user lupa.

8. **State-driven**: `AppState` adalah single source of truth. Render adalah pure function dari state.

---

## Testing Checklist

- [ ] `cargo build --release` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `lint-arwaky-tui` launches and shows file browser
- [ ] Navigate up/down with `j`/`k`
- [ ] Open directory with `Enter` or `l`
- [ ] Go back with `h`
- [ ] Layer badges display correctly (taxonomy=cyan, contract=blue, etc.)
- [ ] Run `c` (check) on selected file/folder
- [ ] Run `s` (scan) on selected file/folder
- [ ] Run `f` (fix) on selected file/folder
- [ ] Right panel shows lint results
- [ ] Mouse click selects file
- [ ] `q` or `Ctrl+Q` quits gracefully
- [ ] Path input dialog works on startup
- [ ] Shortcut bar always visible at bottom
- [ ] Context-sensitive shortcuts change when viewing lint results
- [ ] Search (`/`) finds files in current directory
- [ ] `gg`/`G` jumps to top/bottom
- [ ] `?` shows help overlay

---

## TUI Dependency Flow (What TUI Calls from Domain Crates)

The TUI does NOT shell out to CLI binary. Instead, it calls Rust library functions directly:

```
┌─────────────────────────────────────────────────────────────────┐
│  crates/tui/src/capabilities_lint_executor.rs                   │
│  (calls domain crates directly via trait objects)                │
└─────────────┬───────────────────────────────────────────────────┘
              │
              ├──→ code_analysis::CodeAnalysisContainer::new()
              │    └── .aggregate() → ICodeAnalysisAggregate
              │         └── .run_code_analysis(path) → LintResultList
              │
              ├──→ import_rules::ImportContainer::new(source_parser)
              │    └── .orchestrator() → IImportRunnerAggregate
              │         └── .run_audit(&FilePath) → Vec<LintResult>
              │
              ├──→ naming_rules::NamingContainer::new()
              │    └── .orchestrator() → INamingRunnerAggregate
              │         └── .run_audit(&FilePath) → Vec<LintResult>
              │
              ├──→ role_rules::RoleContainer::new()
              │    └── .orchestrator() → IRoleRunnerAggregate
              │         └── .run_audit(&FilePath) → Vec<LintResult>
              │
              ├──→ external_lint::ExternalLintContainer::new()
              │    └── .aggregate() → IExternalLintAggregate
              │         └── .scan_all(&FilePath) → LintResultList
              │
              ├──→ orphan_detector::OrphanContainer::new()
              │    └── .analyzer() → IOrphanAggregate
              │         └── .check_orphans(detector, files, root) → Vec<LintResult>
              │
              └──→ file_watch::WatchContainer::new()
                   └── .aggregate() → IWatchAggregate
                        └── .run(config, running)
```

### Key Insight: TUI Needs Its Own DI Container

The TUI crate must create its own instances of domain containers. This means:
1. `crates/tui/Cargo.toml` must depend on: `code_analysis`, `import_rules`, `naming_rules`, `role_rules`, `external_lint`, `orphan_detector`, `file_watch`
2. Each domain container must have a `new()` or `new_default()` constructor
3. The TUI's `root_tui_container.rs` wires everything together

### TUI Cargo.toml Dependencies

```toml
[package]
name = "tui-lint-arwaky"
version = "1.10.43"
edition = "2021"

[dependencies]
shared.workspace = true
code_analysis.workspace = true
import_rules.workspace = true
naming_rules.workspace = true
role_rules.workspace = true
external_lint.workspace = true
orphan_detector.workspace = true
file_watch.workspace = true
ratatui = "0.29"
crossterm = "0.28"
tokio.workspace = true
```

---

**Document prepared for handoff to implementing AI agent.**
**All source code included. Ready for implementation.**
