// PURPOSE: utils_dependency_parser — pure parsing functions for dependency files
use shared::project_setup::taxonomy_doctor_vo::{
    DependencyInfo, DependencyReport, SecurityFinding, SecurityScanReport,
};

pub fn parse_cargo_audit(stdout: &str) -> SecurityScanReport {
    let mut findings = Vec::new();
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(stdout) {
        if let Some(list) = json
            .get("vulnerabilities")
            .and_then(|v| v.get("list"))
            .and_then(|l| l.as_array())
        {
            for adv in list {
                let pkg = adv
                    .get("package")
                    .and_then(|p| p.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                let severity = adv
                    .get("severity")
                    .and_then(|s| s.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                let cve = adv
                    .get("advisory")
                    .and_then(|a| a.get("id"))
                    .and_then(|i| i.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                findings.push(SecurityFinding {
                    severity,
                    test_id: cve,
                    file: pkg,
                    line: 0,
                    issue: "Advisory vulnerability".to_string(),
                });
            }
        }
    }
    SecurityScanReport {
        language: "Rust".to_string(),
        tool_name: "cargo-audit".to_string(),
        findings,
        tool_installed: true,
    }
}

pub fn parse_bandit_output(stdout: &str) -> SecurityScanReport {
    let mut findings = Vec::new();
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(stdout) {
        if let Some(results) = json.get("results").and_then(|r| r.as_array()) {
            for r in results {
                let test_id = r
                    .get("test_id")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_string();
                let issue = r
                    .get("issue_text")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_string();
                let severity = r
                    .get("issue_severity")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let fname = r
                    .get("filename")
                    .and_then(|f| f.as_str())
                    .unwrap_or("")
                    .to_string();
                let line = r
                    .get("line_number")
                    .and_then(|l| l.as_u64())
                    .unwrap_or_default();
                findings.push(SecurityFinding {
                    severity,
                    test_id,
                    file: fname,
                    line,
                    issue,
                });
            }
        }
    }
    SecurityScanReport {
        language: "Python".to_string(),
        tool_name: "bandit".to_string(),
        findings,
        tool_installed: true,
    }
}

pub fn parse_cargo_toml_direct_deps(toml_content: &str) -> std::collections::HashSet<String> {
    let mut direct_deps = std::collections::HashSet::new();
    let mut in_deps = false;
    for line in toml_content.lines() {
        if line.trim().starts_with("[dependencies]") {
            in_deps = true;
            continue;
        }
        if in_deps && line.trim().starts_with('[') {
            in_deps = false;
        }
        if in_deps {
            if let Some(eq) = line.find('=') {
                let name = line[..eq].trim().to_string();
                if !name.is_empty() && !name.starts_with('#') {
                    direct_deps.insert(name);
                }
            }
        }
    }
    direct_deps
}

pub fn parse_cargo_lock(
    content: &str,
    direct_deps: &std::collections::HashSet<String>,
) -> DependencyReport {
    let mut in_package = false;
    let mut pkg_name = String::new();
    let mut pkg_version = String::new();
    let mut dependencies = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[[package]]" {
            if !pkg_name.is_empty() && !pkg_version.is_empty() {
                let dep_type = if direct_deps.contains(pkg_name.as_str()) {
                    "direct".to_string()
                } else {
                    "transitive".to_string()
                };
                dependencies.push(DependencyInfo {
                    name: pkg_name.clone(),
                    version: pkg_version.clone(),
                    dep_type,
                });
            }
            pkg_name.clear();
            pkg_version.clear();
            in_package = true;
            continue;
        }
        if in_package {
            if let Some(name_val) = trimmed.strip_prefix("name = ") {
                pkg_name = name_val.trim_matches('"').to_string();
            } else if let Some(ver_val) = trimmed.strip_prefix("version = ") {
                pkg_version = ver_val.trim_matches('"').to_string();
            }
        }
    }
    if !pkg_name.is_empty() && !pkg_version.is_empty() {
        let dep_type = if direct_deps.contains(pkg_name.as_str()) {
            "direct".to_string()
        } else {
            "transitive".to_string()
        };
        dependencies.push(DependencyInfo {
            name: pkg_name,
            version: pkg_version,
            dep_type,
        });
    }

    DependencyReport {
        language: "Rust".to_string(),
        dependencies,
    }
}

pub fn parse_pyproject(content: &str) -> DependencyReport {
    let mut dependencies = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if t.contains('=') && !t.starts_with('[') && !t.starts_with('#') {
            if let Some(eq) = t.find('=') {
                let name = t[..eq].trim().to_string();
                let version = t[eq + 1..]
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();
                dependencies.push(DependencyInfo {
                    name,
                    version,
                    dep_type: "python".to_string(),
                });
            }
        }
    }
    DependencyReport {
        language: "Python".to_string(),
        dependencies,
    }
}

pub fn parse_requirements(content: &str) -> DependencyReport {
    let mut dependencies = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if !t.is_empty() && !t.starts_with('#') {
            dependencies.push(DependencyInfo {
                name: t.to_string(),
                version: String::new(),
                dep_type: "python".to_string(),
            });
        }
    }
    DependencyReport {
        language: "Python".to_string(),
        dependencies,
    }
}
