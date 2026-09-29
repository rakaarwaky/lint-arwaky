use shared::filesystem::contract_filesystem_protocol::IFileSystemIOProtocol;
use shared::maintenance::contract_maintenance_protocol::IAdapterHealthProtocol;
use shared::maintenance::taxonomy_maintenance_vo::{HealthCheckAdapterVO, HealthCheckResult};
use std::sync::Arc;

use shared::maintenance::utility_maintenance_helpers;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct AdapterHealthChecker {
    _io: Arc<dyn IFileSystemIOProtocol>,
}

// ─── Block 2: Protocol Implementation ─────────────────────
impl IAdapterHealthProtocol for AdapterHealthChecker {
    fn health_check(&self) -> HealthCheckResult {
        let adapters = vec![
            HealthCheckAdapterVO {
                name: "clippy".to_string(),
                language: "Rust".to_string(),
                available: utility_maintenance_helpers::check_tool(
                    "cargo",
                    &["clippy", "--version"],
                    false,
                )
                .status
                    == "OK",
            },
            HealthCheckAdapterVO {
                name: "rustfmt".to_string(),
                language: "Rust".to_string(),
                available: utility_maintenance_helpers::check_tool(
                    "rustfmt",
                    &["--version"],
                    false,
                )
                .status
                    == "OK",
            },
            HealthCheckAdapterVO {
                name: "cargo-audit".to_string(),
                language: "Rust".to_string(),
                available: utility_maintenance_helpers::check_tool(
                    "cargo",
                    &["audit", "--version"],
                    false,
                )
                .status
                    == "OK",
            },
            HealthCheckAdapterVO {
                name: "ruff".to_string(),
                language: "Python".to_string(),
                available: utility_maintenance_helpers::check_tool("ruff", &["--version"], false)
                    .status
                    == "OK",
            },
            HealthCheckAdapterVO {
                name: "mypy".to_string(),
                language: "Python".to_string(),
                available: utility_maintenance_helpers::check_tool("mypy", &["--version"], false)
                    .status
                    == "OK",
            },
            HealthCheckAdapterVO {
                name: "bandit".to_string(),
                language: "Python".to_string(),
                available: utility_maintenance_helpers::check_tool("bandit", &["--version"], false)
                    .status
                    == "OK",
            },
            HealthCheckAdapterVO {
                name: "eslint".to_string(),
                language: "JS/TS".to_string(),
                available: utility_maintenance_helpers::check_tool("eslint", &["--version"], false)
                    .status
                    == "OK",
            },
            HealthCheckAdapterVO {
                name: "prettier".to_string(),
                language: "JS/TS".to_string(),
                available: utility_maintenance_helpers::check_tool(
                    "prettier",
                    &["--version"],
                    false,
                )
                .status
                    == "OK",
            },
            HealthCheckAdapterVO {
                name: "tsc".to_string(),
                language: "JS/TS".to_string(),
                available: utility_maintenance_helpers::check_tool("tsc", &["--version"], false)
                    .status
                    == "OK",
            },
            HealthCheckAdapterVO {
                name: "markdownlint-cli".to_string(),
                language: "Markdown".to_string(),
                available: utility_maintenance_helpers::check_tool(
                    "markdownlint-cli",
                    &["--version"],
                    false,
                )
                .status
                    == "OK",
            },
        ];
        HealthCheckResult { adapters }
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
impl AdapterHealthChecker {
    pub fn new(io: Arc<dyn IFileSystemIOProtocol>) -> Self {
        Self { _io: io }
    }
}
