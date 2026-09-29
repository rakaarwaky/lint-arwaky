// PURPOSE: CLI value objects — clap-based Cli/Commands, transport endpoints/URLs,
//          command catalog, scan report, and lint-result aliases.
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::cli_commands::taxonomy_format_vo::Format;
use crate::common::taxonomy_action_vo::ActionName;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::common::taxonomy_suggestion_vo::Suggestion;

// ─── Cli and Commands (from taxonomy_cli_vo) ──────────────────────────

#[derive(Parser, Debug)]
#[command(name = "lint-arwaky")]
#[command(about = "Lint Arwaky CLI: Autonomous Code Quality Gatekeeper.", long_about = None)]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Show debug information
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Minimize output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Directory to save output reports (overrides config)
    #[arg(short, long, global = true)]
    pub output_dir: Option<String>,

    /// Filter output by AES rule code (e.g. AES101, AES102, AES301, AES303)
    #[arg(long, global = true)]
    pub filter: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run all linters and calculate score.
    #[command(alias = "check")]
    Scan {
        /// Path to scan
        path: Option<String>,
        /// Scan only a specific workspace member by name (e.g. "shared", "import-rules")
        #[arg(long)]
        member: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Apply safe automatic fixes
    Fix {
        /// Path to fix
        path: Option<String>,
        /// Preview changes without applying them
        #[arg(long)]
        dry_run: bool,
    },

    /// CI mode (exit 1 if score < threshold)
    Ci {
        /// Path to lint
        path: Option<String>,
        /// Minimum quality score to pass
        #[arg(long, default_value_t = 80)]
        threshold: u32,
    },

    /// Diagnose environment health
    Doctor,

    /// Check orphan: file path → check single file, directory path → scan all files in directory
    Orphan {
        /// File or directory path to check
        path: String,
        /// Scan only a specific workspace member by name (only for directory mode)
        #[arg(long)]
        member: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Run code-quality analysis only (AES101-AES306)
    #[command(name = "quality")]
    ScanQuality {
        /// Path to scan
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Run import-rule checks only (AES201-AES299)
    #[command(name = "import")]
    ScanImport {
        /// Path to scan
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Run naming-rule checks only (AES401-AES406)
    #[command(name = "naming")]
    ScanNaming {
        /// Path to scan
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Run role-rule checks only (AES301-AES399)
    #[command(name = "role")]
    ScanRole {
        /// Path to scan
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Run external linter checks only (Clippy, Ruff, ESLint, etc.)
    #[command(name = "external")]
    ScanExternal {
        /// Path to scan
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Scan for security vulnerabilities
    Security {
        /// Path to scan
        path: Option<String>,
    },

    /// Scan for library vulnerabilities
    Dependencies {
        /// Path to scan
        path: Option<String>,
    },

    /// Watch and lint on changes
    Watch {
        /// Path to watch
        path: Option<String>,
    },

    /// Run AES analysis on git-changed files
    GitDiff {
        /// Git base branch to diff against
        #[arg(long, default_value = "HEAD")]
        base: String,
        /// Path to scan
        path: Option<String>,
        /// Filter changed files by name
        #[arg(long)]
        filter: Option<String>,
    },

    /// Install git pre-commit hook
    InstallHook,

    /// Remove git pre-commit hook
    UninstallHook,

    /// Show version
    Version,

    /// List active linters/adapters
    Adapters,

    /// Create default config
    Init,

    /// Install linter adapter dependencies
    Install {
        /// Use sudo for npm global install
        #[arg(long)]
        sudo: bool,
    },

    /// Print MCP server config for clients
    McpConfig {
        /// Client type (claude, hermes, vscode, all)
        #[arg(long, default_value = "all")]
        client: String,
    },

    /// Show active configuration
    ConfigShow,
}

pub fn get_cli() -> Cli {
    Cli::parse()
}

// ─── Transport / protocol VOs (from taxonomy_protocol_vo) ─────────────

use crate::string_value_object;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransportEndpoint {
    pub protocol: TransportProtocol,
    pub address: String,
}

impl Default for TransportEndpoint {
    fn default() -> Self {
        Self {
            protocol: TransportProtocol::STDAggregate,
            address: String::new(),
        }
    }
}

impl TransportEndpoint {
    pub fn new(protocol: TransportProtocol, address: String) -> Self {
        Self { protocol, address }
    }

    pub fn display_name(&self) -> String {
        match self.protocol {
            TransportProtocol::HTTP => format!("HTTP({})", self.address),
            TransportProtocol::UnixSocket => format!("Socket({})", self.address),
            TransportProtocol::STDAggregate => "Stdio(direct)".to_string(),
        }
    }
    pub fn from_url(url: &str) -> Self {
        let (protocol, address) = match url {
            u if u.starts_with("http://") || u.starts_with("https://") => {
                (TransportProtocol::HTTP, u.to_string())
            }
            "stdio" => (TransportProtocol::STDAggregate, "stdio".to_string()),
            u if u.starts_with('/') || u.starts_with('.') => {
                (TransportProtocol::UnixSocket, u.to_string())
            }
            _ => (TransportProtocol::STDAggregate, "stdio".to_string()),
        };
        Self { protocol, address }
    }
}

impl std::fmt::Display for TransportEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.protocol, self.address)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransportProtocol {
    #[serde(rename = "HTTP")]
    HTTP,
    #[serde(rename = "UnixSocket")]
    UnixSocket,
    #[serde(rename = "Stdio")]
    STDAggregate,
}

impl std::fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportProtocol::HTTP => write!(f, "HTTP"),
            TransportProtocol::UnixSocket => write!(f, "UnixSocket"),
            TransportProtocol::STDAggregate => write!(f, "Stdio"),
        }
    }
}

impl TransportProtocol {
    pub fn needs_desktop_commander(&self) -> bool {
        matches!(
            self,
            TransportProtocol::HTTP | TransportProtocol::UnixSocket
        )
    }
}

string_value_object!(TransportUrlVO);

// ─── Command catalog (from taxonomy_command_catalog_vo) ───────────────

/// Mirrors the CLI binary subcommands in crates/root_cli_main_entry.rs (Command enum).
pub static COMMAND_CATALOG: &[(&str, &str, &str)] = &[
    (
        "check",
        "Run full architecture compliance analysis",
        "lint-arwaky-cli check /path",
    ),
    (
        "scan",
        "Deep directory scan (alias for check)",
        "lint-arwaky-cli scan ./src/",
    ),
    ("fix", "Apply safe fixes", "lint-arwaky-cli fix file.py"),
    (
        "ci",
        "CI-optimized with exit codes",
        "lint-arwaky-cli ci /path --threshold 80",
    ),
    (
        "quality",
        "Quality rules scan (single linter)",
        "lint-arwaky-cli quality ./src/",
    ),
    (
        "role",
        "Role rules scan (single linter)",
        "lint-arwaky-cli role ./src/",
    ),
    (
        "import",
        "Import rules scan (single linter)",
        "lint-arwaky-cli import ./src/",
    ),
    (
        "naming",
        "Naming rules scan (single linter)",
        "lint-arwaky-cli naming ./src/",
    ),
    (
        "orphan",
        "Orphan detection scan (single linter)",
        "lint-arwaky-cli orphan ./src/",
    ),
    (
        "external",
        "External lint scan (ruff, eslint, ...)",
        "lint-arwaky-cli external ./src/",
    ),
    (
        "doctor",
        "System health diagnostics",
        "lint-arwaky-cli doctor",
    ),
    (
        "config",
        "Show effective architecture config",
        "lint-arwaky-cli config",
    ),
    (
        "git",
        "Scan files changed since git base",
        "lint-arwaky-cli git --base develop",
    ),
    (
        "security",
        "Security vulnerability scan",
        "lint-arwaky-cli security ./src/",
    ),
    (
        "dependencies",
        "Dependency report",
        "lint-arwaky-cli dependencies ./src/",
    ),
    (
        "adapters",
        "List external lint adapters",
        "lint-arwaky-cli adapters",
    ),
    (
        "version",
        "Show version information",
        "lint-arwaky-cli version",
    ),
    (
        "init",
        "Initialize project configuration",
        "lint-arwaky-cli init",
    ),
    (
        "install",
        "Install adapter dependencies",
        "lint-arwaky-cli install",
    ),
    (
        "mcp-config",
        "Print MCP client configuration snippet",
        "lint-arwaky-cli mcp-config claude",
    ),
    (
        "install-hook",
        "Install git pre-commit hook",
        "lint-arwaky-cli install-hook",
    ),
    (
        "uninstall-hook",
        "Uninstall git pre-commit hook",
        "lint-arwaky-cli uninstall-hook",
    ),
    (
        "docs",
        "Doc invariants audit (AES601–AES607) over the document chain",
        "lint-arwaky-cli docs ./",
    ),
    (
        "watch",
        "Watch for file changes and lint",
        "lint-arwaky-cli watch ./src/",
    ),
];

/// Metadata attached to each catalog entry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandMetadataVO {
    pub description: DescriptionVO,
    pub example: Suggestion,
}

impl CommandMetadataVO {
    pub fn new(description: DescriptionVO, example: Suggestion) -> Self {
        Self {
            description,
            example,
        }
    }
}

impl std::fmt::Display for CommandMetadataVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description, self.example)
    }
}

/// Derive the full command catalog from COMMAND_CATALOG (single source of truth).
pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
    let mut catalog = HashMap::new();
    for (name, description, example) in COMMAND_CATALOG {
        catalog.insert(
            ActionName::from(*name),
            CommandMetadataVO::new(DescriptionVO::new(*description), Suggestion::new(*example)),
        );
    }
    catalog
}

// ─── Scan report (from taxonomy_scan_report_vo) ───────────────────────

/// Severity level for pipeline diagnostics.
#[derive(Debug, Clone)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// A diagnostic message from a pipeline subsystem.
pub struct PipelineDiagnostic {
    pub source: String,
    pub message: String,
    pub severity: DiagnosticSeverity,
}

impl PipelineDiagnostic {
    pub fn new(source: String, message: String, severity: DiagnosticSeverity) -> Self {
        Self {
            source,
            message,
            severity,
        }
    }
}

/// Error types that can occur during pipeline execution.
#[derive(Debug, Clone)]
pub enum PipelineError {
    PathNotFound(String),
    InvalidPath(String),
    WorkspaceDiscovery(String),
    Analysis(String),
    Io(String),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::PathNotFound(p) => write!(f, "path not found: {p}"),
            PipelineError::InvalidPath(p) => write!(f, "invalid path: {p}"),
            PipelineError::WorkspaceDiscovery(e) => write!(f, "workspace discovery failed: {e}"),
            PipelineError::Analysis(e) => write!(f, "analysis failed: {e}"),
            PipelineError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for PipelineError {}

/// Results of the full analysis pipeline.
pub struct ScanReport {
    pub results: Vec<LintResult>,
    pub diagnostics: Vec<PipelineDiagnostic>,
    pub score: Option<crate::common::taxonomy_common_vo::Score>,
}

impl ScanReport {
    pub fn new(results: Vec<LintResult>, diagnostics: Vec<PipelineDiagnostic>) -> Self {
        Self {
            results,
            diagnostics,
            score: None,
        }
    }

    /// Return the number of violations (results with severity > INFO).
    pub fn violation_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.severity != crate::common::taxonomy_severity_vo::Severity::INFO)
            .count()
    }

    /// Attach a score to the report.
    pub fn with_score(mut self, score: crate::common::taxonomy_common_vo::Score) -> Self {
        self.score = Some(score);
        self
    }
}
