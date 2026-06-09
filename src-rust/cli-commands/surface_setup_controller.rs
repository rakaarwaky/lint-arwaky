use std::sync::Arc;

use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;

pub struct SetupManagementSurface {
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl Default for SetupManagementSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl SetupManagementSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    pub fn generate_env(&self, home: &str) -> String {
        // In real impl: container.setup_processor.generate_env(home)
        format!("# Lint Arwaky environment configuration\nHOME={home}\nMCP_LOG_LEVEL=INFO\n")
    }

    pub fn generate_mcp_config(&self) -> String {
        // In real impl: container.setup_processor.generate_mcp_config()
        r#"{
  "mcpServers": {
    "lint-arwaky": {
      "command": "lint-arwaky",
      "args": []
    }
  }
}"#
        .to_string()
    }

    pub fn mcp_config_claude(&self) -> String {
        self.generate_mcp_config()
    }

    pub fn mcp_config_hermes(&self) -> String {
        self.generate_mcp_config()
    }

    pub fn mcp_config_vscode(&self) -> String {
        let base = self.generate_mcp_config();
        format!("{{\"mcp\": {{\"servers\": {base}}}}}")
    }
}

// Lazy singleton
static INSTANCE: std::sync::Mutex<Option<SetupManagementSurface>> = std::sync::Mutex::new(None);

fn get_instance() -> std::sync::MutexGuard<'static, Option<SetupManagementSurface>> {
    let mut guard = INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
    if guard.is_none() {
        *guard = Some(SetupManagementSurface::new());
    }
    guard
}

pub fn register_setup_management(container: Arc<dyn ServiceContainerAggregate>) {
    let mut guard = INSTANCE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(ref mut s) = *guard {
        s.register_all(container);
    } else {
        let mut s = SetupManagementSurface::new();
        s.register_all(container);
        *guard = Some(s);
    }
}

pub fn generate_env(home: &str) -> String {
    let guard = get_instance();
    guard
        .as_ref()
        .map(|s| s.generate_env(home))
        .unwrap_or_default()
}

pub fn generate_mcp_config() -> String {
    let guard = get_instance();
    guard
        .as_ref()
        .map(|s| s.generate_mcp_config())
        .unwrap_or_default()
}

pub fn mcp_config_claude() -> String {
    let guard = get_instance();
    guard
        .as_ref()
        .map(|s| s.mcp_config_claude())
        .unwrap_or_default()
}

pub fn mcp_config_hermes() -> String {
    let guard = get_instance();
    guard
        .as_ref()
        .map(|s| s.mcp_config_hermes())
        .unwrap_or_default()
}

pub fn mcp_config_vscode() -> String {
    let guard = get_instance();
    guard
        .as_ref()
        .map(|s| s.mcp_config_vscode())
        .unwrap_or_default()
}

pub fn which_mcp_binary() -> String {
    let candidates = [
        std::env::current_exe()
            .ok()
            .and_then(|p| {
                p.parent()
                    .map(|d| d.join("lint-arwaky-mcp").to_string_lossy().to_string())
            })
            .unwrap_or_default(),
        format!(
            "{}/bin/lint-arwaky-mcp",
            std::env::var("CARGO_HOME").unwrap_or_else(|_| "~/.cargo".to_string())
        ),
        "lint-arwaky-mcp".to_string(),
    ];
    for c in &candidates {
        if !c.is_empty() && std::path::Path::new(c).exists() {
            return c.clone();
        }
    }
    std::process::Command::new("which")
        .arg("lint-arwaky-mcp")
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "lint-arwaky-mcp".to_string())
}
