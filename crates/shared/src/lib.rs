// PURPOSE: shared — all taxonomy types, contract traits
// No dependencies on other feature crates — this is the foundation layer.
//
// Dependency flow (top = no deps, bottom = depends on above):
//   common            ← foundation: taxonomy VOs, contract traits, utilities
//   filesystem        ← filesystem taxonomy + contracts (depends on common)
//   config_system     ← config types (depends on common)
//   code_analysis     ← analysis core (depends on common, config_system)
//   import_rules      ← import linting (depends on code_analysis, filesystem)
//   naming_rules      ← naming linting (depends on code_analysis, filesystem)
//   external_lint     ← external linter (depends on code_analysis)
//   orphan_detector   ← orphan detection (depends on code_analysis, filesystem)
//   auto_fix          ← auto-fixing (depends on code_analysis)
//   git_hooks         ← git integration (depends on auto_fix, config_system)
//   file_watch        ← file watching (depends on filesystem, config_system)
//   project_setup     ← project setup (depends on config_system)
//   maintenance       ← maintenance (depends on project_setup)
//   role_rules        ← role rules (depends on config_system)
//   report_formatter  ← reporting (depends on code_analysis)
//   cli_commands      ← CLI surface (depends on report_formatter, auto_fix)
//   mcp_server        ← MCP surface (depends on cli_commands)
//   tui               ← TUI surface (depends on cli_commands)

// ── Layer 0: Foundation ─────────────────────────────────────
pub mod common;

// ── Layer 1: Domain taxonomy ────────────────────────────────
pub mod filesystem;
// pub mod config_system;

// ── Layer 2: Analysis core ──────────────────────────────────
// pub mod code_analysis;

// ── Layer 3: Lint rules (depend on analysis + domain) ──────
// pub mod import_rules;
// pub mod naming_rules;
// pub mod external_lint;
// pub mod orphan_detector;

// ── Layer 4: Fixing & hooks ─────────────────────────────────
// pub mod auto_fix;
// pub mod git_hooks;

// ── Layer 5: Infrastructure ─────────────────────────────────
// pub mod file_watch;
// pub mod project_setup;
// pub mod maintenance;
// pub mod role_rules;

// ── Layer 6: Surface (CLI/MCP/TUI) ─────────────────────────
// pub mod report_formatter;
// pub mod cli_commands;
// pub mod mcp_server;
// pub mod tui;
