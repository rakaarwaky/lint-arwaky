/// mcp_server_resources — Resource handling for auto-linter.

#[derive(Debug, Clone)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub mime_type: String,
}

impl McpResource {
    pub fn new(uri: String, name: String, description: String, mime_type: String) -> Self {
        Self { uri, name, description, mime_type }
    }
}

pub fn build_resources(project_root: &str) -> Vec<McpResource> {
    let mut resources = Vec::new();
    let root = std::path::Path::new(project_root);

    let docs_dir = root.join("docs");
    if docs_dir.exists() && docs_dir.is_dir() {
        if let Ok(entries) = std::fs::read_dir(&docs_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                    resources.push(McpResource::new(
                        format!("auto-linter://rules/{}.md", name),
                        format!("Auto-Linter Rules: {}", name),
                        format!("Rule definitions from {}.md", name),
                        "text/markdown".to_string(),
                    ));
                }
            }
        }
    }

    for config_name in &["auto_linter.config.python.yaml", "auto_linter.config.json", "pyproject.toml"] {
        let config_path = root.join(config_name);
        if config_path.exists() {
            let mime = if config_name.ends_with(".yaml") { "application/x-yaml" } else if config_name.ends_with(".toml") { "application/toml" } else { "application/json" };
            resources.push(McpResource::new(
                format!("auto-linter://config/{}", config_name),
                format!("Auto-Linter Config: {}", config_name),
                format!("Configuration from {}", config_name),
                mime.to_string(),
            ));
        }
    }
    resources
}

pub fn read_resource(uri: &str, project_root: &str) -> Result<String, String> {
    if !uri.starts_with("auto-linter://") {
        return Err(format!("Unknown resource URI scheme: {}", uri));
    }
    let trimmed = uri.trim_start_matches("auto-linter://");
    let parts: Vec<&str> = trimmed.splitn(2, '/').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid resource URI: {}", uri));
    }
    let resource_type = parts[0];
    let filename = parts[1];
    let root = std::path::Path::new(project_root);
    let file_path = if resource_type == "rules" {
        root.join("docs").join(filename)
    } else if resource_type == "config" {
        root.join(filename)
    } else {
        return Err(format!("Unknown resource type: {}", resource_type));
    };
    if !file_path.exists() {
        return Err(format!("Resource not found: {}", filename));
    }
    std::fs::read_to_string(&file_path).map_err(|e| format!("Failed to read resource: {}", e))
}
