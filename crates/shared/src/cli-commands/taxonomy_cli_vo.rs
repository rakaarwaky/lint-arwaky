// PURPOSE: Cli + Commands enums — clap-based CLI definition with all subcommands
use clap::{Parser, Subcommand};

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
    /// Run all linters and calculate score
    Check {
        /// Path to check
        path: Option<String>,
        /// Only check git diff
        #[arg(long)]
        git_diff: bool,
    },

    /// Alias for check (CI-friendly)
    Scan {
        /// Path to scan
        path: Option<String>,
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

    /// Maintenance operations
    Maintenance {
        #[command(subcommand)]
        command: MaintenanceCommands,
    },

    /// Show files changed since base ref
    GitDiff {
        /// Base ref
        #[arg(long, default_value = "HEAD")]
        base: String,
    },

    /// Check if a file is an orphan (AES030)
    Orphan {
        /// File path to check
        path: String,
    },

    /// Scan for security vulnerabilities
    Security {
        /// Path to scan
        path: Option<String>,
    },

    /// Detect code duplication
    Duplicates {
        /// Path to analyze
        path: Option<String>,
    },

    /// Scan for library vulnerabilities
    Dependencies {
        /// Path to scan
        path: Option<String>,
    },

    /// Setup and configuration
    Setup {
        #[command(subcommand)]
        command: SetupCommands,
    },

    /// List active linters/adapters
    Adapters,

    /// Show current configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
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

    /// Export graph JSON for VS Code extension
    VscodeGraph {
        /// Path to the codebase root
        path: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum MaintenanceCommands {
    /// Diagnose environment health
    Doctor,
}

#[derive(Subcommand, Debug)]
pub enum SetupCommands {
    /// Auto-configure lint-arwaky (project-local config)
    Init {
        /// Install default configs to ~/.config/lint-arwaky/ (XDG config dir)
        #[arg(long)]
        global: bool,
    },
    /// Install all linter adapter dependencies (ruff, mypy, bandit, eslint, prettier, tsc)
    Install {
        /// Use sudo for npm global install (will prompt for password)
        #[arg(long)]
        sudo: bool,
    },
    /// Print MCP server config for clients
    McpConfig {
        /// Client type (claude, hermes, vscode, all)
        #[arg(long, default_value = "all")]
        client: String,
    },
    /// Auto-install into Hermes Agent
    Hermes {
        /// Remove lint-arwaky from Hermes
        #[arg(long)]
        remove: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Show active configuration
    Show,
}

pub fn get_cli() -> Cli {
    Cli::parse()
}
