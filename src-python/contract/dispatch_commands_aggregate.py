"""dispatch_commands_aggregate — Command catalog moved from surfaces to contract aggregate.

This is the source of truth for available CLI/MCP commands. Both
infrastructure (for tool schema generation) and surfaces (for dispatch)
import from here.
"""

from ..taxonomy import ActionName, CommandMetadataVO

COMMAND_CATALOG: dict[ActionName, CommandMetadataVO] = {
    ActionName(value="check"): {
        "description": "Run full architecture compliance analysis",
        "example": "auto-lint check /path",
    },
    ActionName(value="scan"): {
        "description": "Deep directory scan (alias for check)",
        "example": "auto-lint scan ./src/",
    },
    ActionName(value="fix"): {"description": "Apply safe fixes", "example": "auto-lint fix file.py"},
    ActionName(value="report"): {
        "description": "Generate quality reports",
        "example": "auto-lint report ./src --format json",
    },
    ActionName(value="ci"): {
        "description": "CI-optimized with exit codes",
        "example": "auto-lint ci /path --exit-zero",
    },
    ActionName(value="batch"): {
        "description": "Check multiple paths",
        "example": "auto-lint batch path1.py path2.js",
    },
    ActionName(value="watch"): {
        "description": "Watch files for changes",
        "example": "auto-lint watch ./src/",
    },
    ActionName(value="security"): {
        "description": "Bandit vulnerability scanning",
        "example": "auto-lint security /path",
    },
    ActionName(value="complexity"): {
        "description": "Cyclomatic complexity",
        "example": "auto-lint complexity ./src/",
    },
    ActionName(value="duplicates"): {
        "description": "Code duplication detection",
        "example": "auto-lint duplicates /path",
    },
    ActionName(value="trends"): {
        "description": "Quality trend over time",
        "example": "auto-lint trends .",
    },
    ActionName(value="dependencies"): {
        "description": "Dependency vulnerability scan",
        "example": "auto-lint dependencies .",
    },
    ActionName(value="diff"): {
        "description": "Compare two versions",
        "example": "auto-lint diff v1.py v2.py",
    },
    ActionName(value="suggest"): {
        "description": "AI-powered suggestions",
        "example": "auto-lint suggest file.py",
    },
    ActionName(value="stats"): {
        "description": "Statistics dashboard",
        "example": "auto-lint stats ./src/",
    },
    ActionName(value="init"): {"description": "Initialize config", "example": "auto-lint init /path"},
    ActionName(value="config"): {
        "description": "Edit configuration",
        "example": "auto-lint config get thresholds",
    },
    ActionName(value="ignore"): {
        "description": "Manage ignore rules",
        "example": "auto-lint ignore add E501",
    },
    ActionName(value="import"): {
        "description": "Import configurations",
        "example": "auto-lint import config.json",
    },
    ActionName(value="export"): {
        "description": "Export reports",
        "example": "auto-lint export --format sarif",
    },
    ActionName(value="clean"): {"description": "Cleanup cache", "example": "auto-lint clean"},
    ActionName(value="update"): {"description": "Update adapters", "example": "auto-lint update"},
    ActionName(value="doctor"): {"description": "Diagnose issues", "example": "auto-lint doctor"},
    ActionName(value="adapters"): {
        "description": "List enabled adapters",
        "example": "auto-lint adapters",
    },
    ActionName(value="install-hook"): {
        "description": "Install git pre-commit hook",
        "example": "auto-lint install-hook",
    },
    ActionName(value="uninstall-hook"): {
        "description": "Remove git pre-commit hook",
        "example": "auto-lint uninstall-hook",
    },
    ActionName(value="cancel"): {
        "description": "Cancel a running lint job",
        "example": "auto-lint cancel <job_id>",
    },
    ActionName(value="plugins"): {
        "description": "List discovered and registered plugins",
        "example": "auto-lint plugins",
    },
    ActionName(value="multi-project"): {
        "description": "Run lint across multiple projects",
        "example": "auto-lint multi-project proj1/ proj2/",
    },
    ActionName(value="version"): {"description": "Show version", "example": "auto-lint version"},
}
