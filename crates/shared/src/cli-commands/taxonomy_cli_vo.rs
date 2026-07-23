// PURPOSE: Cli + Commands enums — clap-based CLI definition with all subcommands
use clap::{Parser, Subcommand};

use crate::cli_commands::taxonomy_format_vo::Format;

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
pub enum ScanSubcommands {
    /// Run code-quality analysis only (AES101-AES306)
    Quality {
        /// Path to scan
        path: Option<String>,
    },

    /// Run import-rule checks only (AES201-AES299)
    Import {
        /// Path to scan
        path: Option<String>,
    },

    /// Run naming-rule checks only (AES401-AES406)
    Naming {
        /// Path to scan
        path: Option<String>,
    },

    /// Run role-rule checks only (AES301-AES399)
    Role {
        /// Path to scan
        path: Option<String>,
    },

    /// Check orphan: file path -> check single file, directory path -> scan all files in directory
    Orphan {
        /// File or directory path to check
        path: Option<String>,
        /// Scan only a specific workspace member by name
        #[arg(long)]
        member: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run all linters and calculate score.
    /// Supports subcommands: quality, import, naming, role, orphan (e.g. `scan quality [path]`)
    #[command(alias = "check")]
    Scan {
        #[command(subcommand)]
        subcommand: Option<ScanSubcommands>,

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
    },

    /// Run code-quality analysis only (AES101-AES306)
    ScanQuality {
        /// Path to scan
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Run import-rule checks only (AES201-AES299)
    ScanImport {
        /// Path to scan
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Run naming-rule checks only (AES401-AES406)
    ScanNaming {
        /// Path to scan
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Run role-rule checks only (AES301-AES399)
    ScanRole {
        /// Path to scan
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Run external linter checks only (Clippy, Ruff, ESLint, etc.)
    ScanExternal {
        /// Path to scan
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Check orphan: file path -> check single file, directory path -> scan all files in directory
    ScanOrphan {
        /// File or directory path to check
        path: Option<String>,
        /// Output format: text, json, sarif, junit
        #[arg(long, default_value_t = Format::Text)]
        format: Format,
    },

    /// Run 6 linter subcommands as parallel Rust subprocesses and measure timing
    ScanParallelSubprocess {
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
