// PURPOSE: role-rules shared constants — layer names, numeric limits, and token tables
// Consolidated from former taxonomy_role_rules_constant, taxonomy_role_rules_constant,
// and taxonomy_role_rules_constant into a single taxonomy_role_rules_constant.

// ── Layer name constants (formerly taxonomy_role_rules_constant) ──

pub const LAYER_AGENT: &str = "agent";
pub const LAYER_CAPABILITIES: &str = "capabilities";
pub const LAYER_CONTRACT: &str = "contract";
pub const LAYER_UTILITY: &str = "utility";
pub const LAYER_SURFACES: &str = "surfaces";
pub const LAYER_TAXONOMY: &str = "taxonomy";
pub const LAYER_ROOT: &str = "root";
pub const LAYER_GLOBAL: &str = "global";

// ── Role thresholds (formerly taxonomy_role_rules_constant) ──
// Used by: role-rules (SurfaceRustRoleAuditor, SurfacePythonRoleAuditor,
//          SurfaceTypeScriptRoleAuditor, utility_surface_role_checker)

/// Maximum public methods a passive/utility surface may expose.
pub const MAX_PUBLIC_METHODS: usize = 50;

/// Maximum control flow statements tolerated in a passive/utility surface.
pub const MAX_CONTROL_FLOW: usize = 50;

/// Maximum functions in a smart surface (`_command` / `_controller` / `_page`).
/// Smart surfaces orchestrate sub-commands, so this is generous.
pub const MAX_FN_COUNT_SMART: usize = 50;

/// Maximum functions in a utility surface (`_action` / `_store` / `_hook` /
/// `_screen` / `_router`). Utility surfaces are thin adapters.
pub const MAX_FN_COUNT_UTILITY: usize = 25;

/// Maximum functions in a passive surface (`_component` / `_view` / `_layout`).
/// Passive surfaces only render.
pub const MAX_FN_COUNT_PASSIVE: usize = 25;

// ── Token tables (formerly taxonomy_role_rules_constant) ──
// Used by: role-rules (AgentRustRoleAuditor, AgentPythonRoleAuditor, AgentTsRoleAuditor)
//
// The token tables live here rather than inline in each auditor because
// AES403 forbids file-level constants in a `capabilities_` file: a constant
// is a shared policy value, and a shared policy value belongs in taxonomy.
// The three auditors differ only in the tokens their language spells, so each
// has its own table below.

/// Operations an agent file may not perform in Rust.
///
/// An agent coordinates in-memory protocols; it does not touch the
/// filesystem, the network, the clock, or the process environment. Those
/// belong to the surface layer that the agent is wired behind.
pub const AGENT_FORBIDDEN_IO_RUST: &[(&str, &str)] = &[
    ("std::fs::", "filesystem access"),
    ("File::open", "filesystem access"),
    ("std::io::", "stream access"),
    ("std::env::", "environment access"),
    ("reqwest::", "network access"),
    ("hyper::", "network access"),
    ("sqlx::", "database access"),
    ("rusqlite::", "database access"),
    ("println!", "console output"),
    ("eprintln!", "console output"),
    ("dbg!", "debug output"),
];

/// Operations an agent file may not perform in Python.
pub const AGENT_FORBIDDEN_IO_PYTHON: &[(&str, &str)] = &[
    ("open(", "file I/O"),
    ("os.", "process I/O"),
    ("Path(", "filesystem I/O"),
    ("httpx.", "network I/O"),
    ("requests.", "network I/O"),
    ("sqlite3.", "database I/O"),
    ("asyncpg.", "database I/O"),
    ("print(", "console output"),
    ("subprocess.", "process I/O"),
    ("shutil.", "filesystem I/O"),
];

/// Operations an agent file may not perform in TypeScript / JavaScript.
pub const AGENT_FORBIDDEN_IO_TYPESCRIPT: &[(&str, &str)] = &[
    ("require(\"fs\")", "filesystem I/O"),
    ("require('fs')", "filesystem I/O"),
    ("import fs from", "filesystem I/O"),
    ("fs.", "filesystem I/O"),
    ("readFile", "filesystem I/O"),
    ("writeFile", "filesystem I/O"),
    ("fetch(", "network I/O"),
    ("axios", "network I/O"),
    ("console.", "console output"),
    ("process.", "process I/O"),
    ("sqlite", "database I/O"),
    ("node:fs", "filesystem I/O"),
];
