use std::fs;
use std::path::Path;

use crate::taxonomy::{default_aes_config, ArchitectureConfig, LintResult};

pub fn collect_source_files(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    if !dir.exists() || !dir.is_dir() {
        return files;
    }
    collect_source_files_recursive(dir, &mut files);
    files
}

fn is_source_file(ext: &str) -> bool {
    matches!(ext, "rs" | "py" | "ts" | "js" | "tsx" | "jsx")
}

fn collect_source_files_recursive(dir: &Path, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                if dir_name == "target"
                    || dir_name == ".git"
                    || dir_name == ".opencode"
                    || dir_name == "node_modules"
                {
                    continue;
                }
                collect_source_files_recursive(&path, files);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if is_source_file(ext) {
                    if let Some(path_str) = path.to_str() {
                        files.push(path_str.to_string());
                    }
                }
            }
        }
    }
}

pub fn collect_rs_files(dir: &Path) -> Vec<String> {
    collect_source_files(dir)
        .into_iter()
        .filter(|f| f.ends_with(".rs"))
        .collect()
}

/// Walk up from `start` looking for the YAML config file.
/// Returns the parsed ArchitectureConfig if found, or None.
fn try_load_yaml_config(start: &Path) -> Option<ArchitectureConfig> {
    const CONFIG_NAMES: &[&str] = &["lint_arwaky.config.rust.yaml", "lint_arwaky.config.yaml"];
    // Remove null values recursively so #[serde(default)] kicks in
    fn remove_nulls(val: &mut serde_json::Value) {
        match val {
            serde_json::Value::Object(m) => {
                m.retain(|_, v| !v.is_null());
                for v in m.values_mut() {
                    remove_nulls(v);
                }
            }
            serde_json::Value::Array(arr) => {
                for v in arr.iter_mut() {
                    remove_nulls(v);
                }
            }
            _ => {}
        }
    }
    let mut dir = start;
    loop {
        for name in CONFIG_NAMES {
            let candidate = dir.join(name);
            if candidate.is_file() {
                if let Ok(content) = fs::read_to_string(&candidate) {
                    if let Ok(raw) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                        if let Some(arch_val) = raw.get("architecture") {
                            let mut json = match serde_json::to_value(arch_val) {
                                Ok(v) => v,
                                Err(_) => return None,
                            };
                            remove_nulls(&mut json);
                            // Convert YAML suffix format to struct fields
                            if let Some(layers_obj) = json.get_mut("layers") {
                                if let Some(obj) = layers_obj.as_object_mut() {
                                    let mut suffix_updates: Vec<(
                                        String,
                                        Option<String>,
                                        serde_json::Value,
                                        serde_json::Value,
                                    )> = Vec::new();
                                    for (layer_name, layer) in obj.iter() {
                                        if let Some(suffix_val) = layer.get("suffix") {
                                            if let Some(arr) = suffix_val.as_array() {
                                                let mut policy: Option<String> = None;
                                                let mut allowed =
                                                    serde_json::Value::Array(Vec::new());
                                                let mut forbidden =
                                                    serde_json::Value::Array(Vec::new());
                                                for entry in arr {
                                                    if let Some(entry_obj) = entry.as_object() {
                                                        for (pkey, plist) in entry_obj {
                                                            match pkey.as_str() {
                                                                "strict" | "flexible" => {
                                                                    policy = Some(pkey.clone());
                                                                    if let Some(list) =
                                                                        plist.as_array()
                                                                    {
                                                                        allowed =
                                                                            serde_json::json!(list);
                                                                    }
                                                                }
                                                                "forbidden" => {
                                                                    if let Some(list) =
                                                                        plist.as_array()
                                                                    {
                                                                        forbidden =
                                                                            serde_json::json!(list);
                                                                    }
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                }
                                                suffix_updates.push((
                                                    layer_name.clone(),
                                                    policy,
                                                    allowed,
                                                    forbidden,
                                                ));
                                            }
                                        }
                                    }
                                    for (name, policy, allowed, forbidden) in suffix_updates {
                                        if let Some(layer) = obj.get_mut(&name) {
                                            if let Some(layer_obj) = layer.as_object_mut() {
                                                if let Some(ref p) = policy {
                                                    layer_obj.insert(
                                                        "suffix_policy".to_string(),
                                                        serde_json::json!(p),
                                                    );
                                                }
                                                layer_obj
                                                    .insert("allowed_suffix".to_string(), allowed);
                                                if let Some(arr) = forbidden.as_array() {
                                                    if !arr.is_empty() {
                                                        layer_obj.insert(
                                                            "forbidden_suffix".to_string(),
                                                            forbidden,
                                                        );
                                                    }
                                                }
                                                layer_obj.remove("suffix");
                                            }
                                        }
                                    }
                                }
                            }
                            // Flatten nested rules (global/internal/external) into a single array.
                            if let Some(rules_obj) = json.get_mut("rules") {
                                if let Some(obj) = rules_obj.as_object_mut() {
                                    let mut flat = serde_json::Value::Array(Vec::new());
                                    for (_, v) in obj.iter() {
                                        if let Some(arr) = v.as_array() {
                                            for item in arr {
                                                if let Some(arr) = flat.as_array_mut() {
                                                    arr.push(item.clone());
                                                }
                                            }
                                        }
                                    }
                                    *rules_obj = flat;
                                }
                            }
                            if let Ok(cfg) = serde_json::from_value::<ArchitectureConfig>(json) {
                                return Some(cfg);
                            }
                        }
                    }
                }
            }
        }
        match dir.parent() {
            Some(p) if p != dir => dir = p,
            _ => break,
        }
    }
    None
}

pub fn load_config(project_root: Option<&Path>, src_dir: &Path) -> ArchitectureConfig {
    let search_start = project_root.unwrap_or_else(|| src_dir.parent().unwrap_or(src_dir));
    try_load_yaml_config(search_start).unwrap_or_else(default_aes_config)
}

pub fn format_report(results: &[LintResult], project_root: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push("=".repeat(60));
    lines.push("  AES Architecture Compliance Report (Self-Lint)".to_string());
    lines.push("=".repeat(60));
    lines.push(format!("  Project: {}", project_root));
    lines.push(format!("  Files scanned: {}", results.len()));
    lines.push("=".repeat(60));
    lines.push("".to_string());

    let mut critical = Vec::new();
    let mut high = Vec::new();
    let mut medium = Vec::new();
    let mut low = Vec::new();

    for r in results {
        match r.severity {
            crate::taxonomy::Severity::CRITICAL => critical.push(r),
            crate::taxonomy::Severity::HIGH => high.push(r),
            crate::taxonomy::Severity::MEDIUM => medium.push(r),
            crate::taxonomy::Severity::LOW => low.push(r),
            _ => medium.push(r),
        }
    }

    for (sev, items) in [
        ("CRITICAL", &critical),
        ("HIGH", &high),
        ("MEDIUM", &medium),
        ("LOW", &low),
    ] {
        if items.is_empty() {
            continue;
        }
        lines.push(format!("  [{}] {} violations", sev, items.len()));
        lines.push("-".repeat(60));
        for r in items.iter() {
            lines.push(format!("  [{}] {}", format!("{}", r.code), r.file.value));
            for msg_line in r.message.value.lines() {
                lines.push(format!("    {}", msg_line));
            }
        }
        lines.push("".to_string());
    }

    let total = results.len();
    let mut per_code: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for r in results {
        *per_code.entry(format!("{}", r.code)).or_insert(0) += 1;
    }
    let categories = per_code.len();

    lines.push("=".repeat(60));
    lines.push(format!("  Total AES Violations: {}", total));
    lines.push(format!("  Total Category AES Violations: {}", categories));
    if categories > 0 {
        lines.push("-".repeat(60));
        for (code, count) in &per_code {
            lines.push(format!("  {}: {}", code, count));
        }
    }
    lines.push("".to_string());
    if total == 0 {
        lines.push("  Status: PASS - No AES violations detected".to_string());
    } else {
        lines.push("  Status: FAIL - AES violations detected".to_string());
    }
    lines.push("=".repeat(60));
    lines.join("\n")
}
