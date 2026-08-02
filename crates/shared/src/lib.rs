// PURPOSE: shared — all taxonomy types, contract traits
// No dependencies on other feature crates — this is the foundation layer.

// ── Layer 0: Foundation ─────────────────────────────────────
pub mod common;

// ── Layer 1: Core ───────────────────────────────────────────
pub mod config_system;
pub mod filesystem;

// ── Layer 2: Lint rules ─────────────────────────────────────
// pub mod naming_rules;
// pub mod import_rules;
// pub mod quality_rules;
// pub mod orphan_rules;
// pub mod role_rules;
// pub mod external_lint;

// ── Layer 3: Infrastructure ─────────────────────────────────
// pub mod auto_fix;
// pub mod git_hooks;
// pub mod file_watch;
// pub mod project_setup;
// pub mod maintenance;

// ── Layer 4: Surface (CLI/MCP/TUI) ─────────────────────────
// pub mod report_formatter;
// pub mod cli_commands;
// pub mod mcp_server;
// pub mod tui;
