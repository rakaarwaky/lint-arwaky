// PURPOSE: Utility helpers for project-setup — file writing, template loading,
//          pre-flight checks, and path queries.
//
// Stateless functions supporting the four business capabilities (MCP config,
// env generation, language detection, adapter installation).

use crate::taxonomy_project_setup_vo::{
    CreateConfigDirResult, EmbeddedSkillVO, PackageManagerStatus, PreFlightResult, SetupError,
    WriteConfigResult,
};
use shared_common::taxonomy_suggestion_vo::DescriptionVO;

/// Return the embedded config template for the given language.
pub fn get_config_template(language: &str) -> Result<&'static str, SetupError> {
    match language {
        "rust" | "python" | "javascript" | "typescript" | "all" => {
            Ok(include_str!("../../config/lint_arwaky.config.yaml"))
        }
        _ => Err(SetupError::unknown_language(
            "rust, python, javascript, typescript, all",
        )),
    }
}

/// Retrieve all embedded skill files compiled into the binary.
pub fn get_embedded_skills() -> &'static [EmbeddedSkillVO] {
    crate::EMBEDDED_SKILLS
}

/// Write a configuration file to disk and return a description.
pub fn write_config_file(filename: &str, content: &str) -> WriteConfigResult {
    let byte_count = content.len();
    std::fs::write(filename, content).map_err(|e| SetupError::io(e.to_string()))?;
    Ok(DescriptionVO::new(format!(
        "wrote {} ({} bytes)",
        filename, byte_count
    )))
}

/// Create the global XDG config directory and return its path.
pub fn create_global_config_dir() -> CreateConfigDirResult {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| SetupError::invalid_state("Could not determine XDG config directory"))?
        .join("lint-arwaky");
    std::fs::create_dir_all(&config_dir).map_err(|e| SetupError::io(e.to_string()))?;
    Ok(config_dir)
}

/// Check that pip and npm are available on the system.
pub fn pre_flight_check() -> PreFlightResult {
    let mut results = Vec::new();

    let pip_ok = std::process::Command::new("pip")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
        || std::process::Command::new("python3")
            .args(["-m", "pip", "--version"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
    results.push(PackageManagerStatus {
        tool: "pip".to_string(),
        status: if pip_ok {
            "ok".to_string()
        } else {
            "not_found".to_string()
        },
    });

    let npm_ok = std::process::Command::new("npm")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    results.push(PackageManagerStatus {
        tool: "npm".to_string(),
        status: if npm_ok {
            "ok".to_string()
        } else {
            "not_found".to_string()
        },
    });

    results
}

/// Check whether a file exists at the given path.
pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}
