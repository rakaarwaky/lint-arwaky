// PURPOSE: MaintenanceChecker — IMaintenanceCheckerProtocol for maintenance: toolchain diagnostics, security scan, dependency report
// Zero I/O: all external tool execution and file reading delegated to ports via DI.
use std::sync::Arc;

use shared::common::taxonomy_path_vo::FilePath;
use shared::project_setup::contract_filesystem_maintenance_port::IFileSystemMaintenancePort;
use shared::project_setup::contract_maintenance_protocol::IMaintenanceCheckerProtocol;
use shared::project_setup::contract_tool_executor_port::IToolExecutorPort;
use shared::project_setup::taxonomy_dependency_parser_utility::{
    parse_bandit_output, parse_cargo_audit, parse_cargo_lock, parse_cargo_toml_direct_deps,
    parse_pyproject, parse_requirements,
};
use shared::project_setup::taxonomy_doctor_vo::{
    DependencyReport, SecurityScanReport, ToolStatus, ToolchainDiagnostics,
};

// ─── Block 1: Struct Definition ───────────────────────────
pub struct MaintenanceChecker {
    tool_executor: Arc<dyn IToolExecutorPort>,
    fs: Arc<dyn IFileSystemMaintenancePort>,
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
#[async_trait::async_trait]
impl IMaintenanceCheckerProtocol for MaintenanceChecker {
    async fn diagnose_toolchain(&self) -> ToolchainDiagnostics {
        let mut rust_tools = vec![self.check_tool_status("cargo", true).await];
        let mut clippy_status = self.check_tool_status("cargo", true).await;
        clippy_status.name = "clippy".to_string();
        rust_tools.push(clippy_status);
        rust_tools.push(self.check_tool_status("rustfmt", true).await);

        let python_tools = vec![
            self.check_tool_status("python3", false).await,
            self.check_tool_status("ruff", false).await,
            self.check_tool_status("mypy", false).await,
            self.check_tool_status("bandit", false).await,
        ];

        let mut js_tools = vec![self.check_tool_status("node", false).await];

        let eslint_local = self
            .fs
            .file_exists(&FilePath::new("node_modules/.bin/eslint").unwrap_or_default())
            .await;
        let eslint_status = if eslint_local {
            ToolStatus {
                name: "eslint (local)".to_string(),
                status: "OK".to_string(),
                version: "local".to_string(),
            }
        } else {
            let mut st = self.check_tool_status("eslint", false).await;
            st.name = "eslint".to_string();
            st
        };
        js_tools.push(eslint_status);

        let prettier_local = self
            .fs
            .file_exists(&FilePath::new("node_modules/.bin/prettier").unwrap_or_default())
            .await;
        let prettier_status = if prettier_local {
            ToolStatus {
                name: "prettier (local)".to_string(),
                status: "OK".to_string(),
                version: "local".to_string(),
            }
        } else {
            let mut st = self.check_tool_status("prettier", false).await;
            st.name = "prettier".to_string();
            st
        };
        js_tools.push(prettier_status);

        let tsc_local = self
            .fs
            .file_exists(&FilePath::new("node_modules/.bin/tsc").unwrap_or_default())
            .await;
        let tsc_status = if tsc_local {
            ToolStatus {
                name: "tsc (local)".to_string(),
                status: "OK".to_string(),
                version: "local".to_string(),
            }
        } else {
            let mut st = self.check_tool_status("tsc", false).await;
            st.name = "tsc".to_string();
            st
        };
        js_tools.push(tsc_status);

        let vcs_tools = vec![
            self.check_tool_status("git", true).await,
            self.check_tool_status("jj", false).await,
        ];

        let binary_path = self
            .tool_executor
            .get_binary_path()
            .await
            .value()
            .to_string();

        ToolchainDiagnostics {
            rust_tools,
            python_tools,
            js_tools,
            vcs_tools,
            binary_path,
        }
    }

    async fn run_security_scan(&self, project_path: &FilePath) -> SecurityScanReport {
        let cargo_lock_fp =
            FilePath::new(format!("{}/Cargo.lock", project_path.value())).unwrap_or_default();

        if self.fs.file_exists(&cargo_lock_fp).await {
            let output = self
                .tool_executor
                .run_tool_in_dir("cargo", &["audit", "--json"], project_path)
                .await;
            if !output.success {
                return SecurityScanReport {
                    language: "Rust".to_string(),
                    tool_name: "cargo-audit".to_string(),
                    findings: Vec::new(),
                    tool_installed: false,
                };
            }
            return parse_cargo_audit(&output.stdout);
        }

        let output = self
            .tool_executor
            .run_tool_in_dir("bandit", &["-r", "--format", "json", "."], project_path)
            .await;
        if !output.success && output.stdout.is_empty() {
            return SecurityScanReport {
                language: "Python".to_string(),
                tool_name: "bandit".to_string(),
                findings: Vec::new(),
                tool_installed: false,
            };
        }
        parse_bandit_output(&output.stdout)
    }

    async fn run_dependency_report(
        &self,
        project_path: &FilePath,
    ) -> Result<DependencyReport, String> {
        let base = project_path.value();

        let cargo_lock = FilePath::new(format!("{}/Cargo.lock", base)).unwrap_or_default();
        if self.fs.file_exists(&cargo_lock).await {
            let content = self.fs.read_file(&cargo_lock).await?;
            let cargo_toml = FilePath::new(format!("{}/Cargo.toml", base)).unwrap_or_default();
            let direct_deps = if self.fs.file_exists(&cargo_toml).await {
                let toml_content = self.fs.read_file(&cargo_toml).await.unwrap_or_default();
                parse_cargo_toml_direct_deps(&toml_content)
            } else {
                std::collections::HashSet::new()
            };
            return Ok(parse_cargo_lock(&content, &direct_deps));
        }

        let pyproject = FilePath::new(format!("{}/pyproject.toml", base)).unwrap_or_default();
        if self.fs.file_exists(&pyproject).await {
            let content = self.fs.read_file(&pyproject).await?;
            return Ok(parse_pyproject(&content));
        }

        let reqs = FilePath::new(format!("{}/requirements.txt", base)).unwrap_or_default();
        if self.fs.file_exists(&reqs).await {
            let content = self.fs.read_file(&reqs).await?;
            return Ok(parse_requirements(&content));
        }

        Err("No dependency files found (Cargo.lock, pyproject.toml, requirements.txt)".to_string())
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl MaintenanceChecker {
    pub fn new(
        tool_executor: Arc<dyn IToolExecutorPort>,
        fs: Arc<dyn IFileSystemMaintenancePort>,
    ) -> Self {
        Self { tool_executor, fs }
    }

    async fn check_tool_status(&self, name: &str, required: bool) -> ToolStatus {
        let output = self.tool_executor.run_tool(name, &["--version"]).await;
        let (status, version) = if output.success {
            let ver = output
                .stdout
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            ("OK".to_string(), ver)
        } else if required {
            ("FAIL".to_string(), "NOT FOUND".to_string())
        } else {
            ("WARN".to_string(), "NOT FOUND".to_string())
        };
        ToolStatus {
            name: name.to_string(),
            status,
            version,
        }
    }
}
