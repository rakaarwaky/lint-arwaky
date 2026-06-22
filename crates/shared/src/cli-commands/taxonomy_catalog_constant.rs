// PURPOSE: COMMAND_CATALOG — static table of every public CLI/MCP command with description and usage example

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
        "ci",
        "CI-optimized with exit codes",
        "lint-arwaky ci /path --threshold 80",
    ),
    (
        "doctor",
        "Diagnose environment health",
        "lint-arwaky doctor",
    ),
    (
        "orphan",
        "Check if file is dead/unreachable code",
        "lint-arwaky orphan <path>",
    ),
    (
        "security",
        "Vulnerability scanning",
        "lint-arwaky security /path",
    ),
    (
        "duplicates",
        "Code duplication detection",
        "lint-arwaky duplicates /path",
    ),
    (
        "dependencies",
        "Dependency vulnerability scan",
        "lint-arwaky dependencies .",
    ),
    (
        "watch",
        "Watch files for changes",
        "lint-arwaky watch ./src/",
    ),
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
    ("adapters", "List enabled adapters", "lint-arwaky adapters"),
    ("version", "Show version", "lint-arwaky version"),
    ("init", "Create default config", "lint-arwaky init"),
    (
        "install",
        "Install adapter dependencies",
        "lint-arwaky install",
    ),
    (
        "mcp-config",
        "Print MCP server config",
        "lint-arwaky mcp-config --client claude",
    ),
    (
        "config-show",
        "Show active configuration",
        "lint-arwaky config-show",
    ),
];
