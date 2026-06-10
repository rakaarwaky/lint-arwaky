// PURPOSE: Cli + Commands enums — clap-based CLI definition with all subcommands (check, scan, fix, dev, config, report, setup, etc.)

use clap::{Parser, Subcommand};

use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
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

    /// Filter output by AES rule code (e.g. AES014, AES010, AES0305)
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

    /// Generate quality reports
    Report {
        /// Path to report on
        path: Option<String>,
        /// Output format
        #[arg(long, default_value = "text", value_parser = ["text", "json", "sarif", "junit"])]
        output_format: String,
    },

    /// CI mode (exit 1 if score < threshold)
    Ci {
        /// Path to lint
        path: Option<String>,
        /// Minimum quality score to pass
        #[arg(long, default_value_t = 80)]
        threshold: u32,
    },

    /// Show files changed since base ref
    GitDiff {
        /// Base ref
        #[arg(long, default_value = "HEAD")]
        base: String,
    },

    /// Run lint across multiple projects
    MultiProject {
        /// Paths to lint
        #[arg(required = true)]
        paths: Vec<String>,
    },

    /// Scan for security vulnerabilities
    Security {
        /// Path to scan
        path: Option<String>,
    },

    /// Cyclomatic complexity analysis
    Complexity {
        /// Path to analyze
        path: Option<String>,
    },

    /// Detect code duplication
    Duplicates {
        /// Path to analyze
        path: Option<String>,
    },

    /// Monitor quality trends over time
    Trends {
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

    /// Cancel a running lint job
    Cancel {
        /// Job ID to cancel
        job_id: String,
    },

    /// Compare lint results between two versions
    Diff {
        /// First path
        path1: String,
        /// Second path
        path2: String,
    },

    /// Import configuration
    Import {
        /// Config file path
        config_file: String,
    },

    /// Export lint reports
    Export {
        /// Output format
        #[arg(value_parser = ["sarif", "junit", "json"])]
        format: String,
    },

    /// Watch and lint on changes
    Watch {
        /// Path to watch
        path: Option<String>,
    },

    /// Provide improvement suggestions
    Suggest {
        /// Path to analyze
        path: Option<String>,
        /// Use AI for suggestions
        #[arg(long)]
        ai: bool,
    },

    /// Install git pre-commit hook
    InstallHook,

    /// Remove git pre-commit hook
    UninstallHook,

    /// Show version
    Version,
}

#[derive(Subcommand, Debug)]
pub enum SetupCommands {
    /// Auto-configure lint-arwaky
    Init,
    /// Diagnose environment
    Doctor,
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

pub struct CoreCommandsSurface {
    pub container: Option<Box<dyn ServiceContainerAggregate>>,
}

impl Default for CoreCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl CoreCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: Box<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    pub fn version() {
        let ver = env!("CARGO_PKG_VERSION");
        println!("Lint Arwaky v{ver} (AES Semantic Builder)");
    }
}

pub fn get_cli() -> Cli {
    Cli::parse()
}

pub fn get_surface() -> CoreCommandsSurface {
    CoreCommandsSurface::new()
}
