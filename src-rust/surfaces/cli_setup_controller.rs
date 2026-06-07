use crate::contract::service_container_aggregate::ServiceContainerAggregate;
use std::sync::Arc;

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
    let mut guard = INSTANCE.lock().unwrap();
    if guard.is_none() {
        *guard = Some(SetupManagementSurface::new());
    }
    guard
}

pub fn register_setup_management(container: Arc<dyn ServiceContainerAggregate>) {
    let mut guard = INSTANCE.lock().unwrap();
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
