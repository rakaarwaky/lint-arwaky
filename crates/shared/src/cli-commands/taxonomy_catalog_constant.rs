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
        "lint-arwaky ci /path --exit-zero",
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
        "duplicates",
        "Code duplication detection",
        "lint-arwaky duplicates /path",
    ),
    (
        "dependencies",
        "Dependency vulnerability scan",
        "lint-arwaky dependencies .",
    ),
    ("maintenance doctor", "Diagnose environment health", "lint-arwaky maintenance doctor"),
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
        "plugins",
        "List discovered and registered plugins",
        "lint-arwaky plugins",
    ),
    ("version", "Show version", "lint-arwaky version"),
];
