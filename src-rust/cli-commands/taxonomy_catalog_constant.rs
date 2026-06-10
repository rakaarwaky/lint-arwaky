//! CLI/MCP command catalog constant.
//!
//! Static table of every public top-level command exposed by the CLI and
//! MCP server, paired with a short description and a usage example. The
//! catalog is the single source of truth for `list_commands` and the
//! dispatch routing checker (AES0303).
//!
//! AES_RULES: Follows the `_constant` taxonomy role (AES0301).

pub static COMMAND_CATALOG: &[(&str, &str, &str)] = &[
    (
        "check",
        "Run full architecture compliance analysis",
        "lint-arwaky check /path",
    ),
    (
        "scan",
        "Deep directory scan (alias for check)",
        "lint-arwaky scan ./src/",
    ),
    ("fix", "Apply safe fixes", "lint-arwaky fix file.py"),
    (
        "report",
        "Generate quality reports",
        "lint-arwaky report ./src --format json",
    ),
    (
        "ci",
        "CI-optimized with exit codes",
        "lint-arwaky ci /path --exit-zero",
    ),
    (
        "batch",
        "Check multiple paths",
        "lint-arwaky batch path1.py path2.js",
    ),
    (
        "watch",
        "Watch files for changes",
        "lint-arwaky watch ./src/",
    ),
    (
        "security",
        "Bandit vulnerability scanning",
        "lint-arwaky security /path",
    ),
    (
        "complexity",
        "Cyclomatic complexity",
        "lint-arwaky complexity ./src/",
    ),
    (
        "duplicates",
        "Code duplication detection",
        "lint-arwaky duplicates /path",
    ),
    ("trends", "Quality trend over time", "lint-arwaky trends ."),
    (
        "dependencies",
        "Dependency vulnerability scan",
        "lint-arwaky dependencies .",
    ),
    (
        "diff",
        "Compare two versions",
        "lint-arwaky diff v1.py v2.py",
    ),
    (
        "suggest",
        "AI-powered suggestions",
        "lint-arwaky suggest file.py",
    ),
    ("stats", "Statistics dashboard", "lint-arwaky stats ./src/"),
    ("init", "Initialize config", "lint-arwaky init /path"),
    (
        "config",
        "Edit configuration",
        "lint-arwaky config get thresholds",
    ),
    (
        "ignore",
        "Manage ignore rules",
        "lint-arwaky ignore add E501",
    ),
    (
        "import",
        "Import configurations",
        "lint-arwaky import config.json",
    ),
    (
        "export",
        "Export reports",
        "lint-arwaky export --format sarif",
    ),
    ("clean", "Cleanup cache", "lint-arwaky clean"),
    ("update", "Update adapters", "lint-arwaky update"),
    ("doctor", "Diagnose issues", "lint-arwaky doctor"),
    ("adapters", "List enabled adapters", "lint-arwaky adapters"),
    (
        "install-hook",
        "Install git pre-commit hook",
        "lint-arwaky install-hook",
    ),
    (
        "uninstall-hook",
        "Remove git pre-commit hook",
        "lint-arwaky uninstall-hook",
    ),
    (
        "cancel",
        "Cancel a running lint job",
        "lint-arwaky cancel <job_id>",
    ),
    (
        "plugins",
        "List discovered and registered plugins",
        "lint-arwaky plugins",
    ),
    (
        "multi-project",
        "Run lint across multiple projects",
        "lint-arwaky multi-project proj1/ proj2/",
    ),
    ("version", "Show version", "lint-arwaky version"),
];
