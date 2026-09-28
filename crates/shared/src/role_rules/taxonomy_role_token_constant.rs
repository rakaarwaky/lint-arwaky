// PURPOSE: role-rules token tables — shared constants for AES405 agent role checks
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
