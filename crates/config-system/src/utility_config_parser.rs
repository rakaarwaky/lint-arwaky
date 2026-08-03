use shared::common::taxonomy_common_vo::BooleanVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;

/// Parse the score threshold from raw YAML config content.
/// Checks `project.thresholds.score` then `thresholds.score`.
/// Returns the value if found, otherwise None.
pub fn parse_score_threshold(yaml_str: &str) -> Option<f64> {
    let raw: serde_yaml_ng::Value = serde_yaml_ng::from_str(yaml_str).ok()?;
    raw.get("project")
        .and_then(|p| p.get("thresholds"))
        .and_then(|t| t.get("score"))
        .and_then(|s| s.as_f64())
        .or_else(|| {
            raw.get("thresholds")
                .and_then(|t| t.get("score"))
                .and_then(|s| s.as_f64())
        })
}

/// Returns names of adapters whose status is "enabled" (default).
/// Ignores special entries like "architecture" (internal analysis).
/// Supports both `status` (string) and `enabled` (bool) fields per Appendix A.
pub fn parse_adapter_names_from_yaml(yaml_str: &str) -> Vec<String> {
    let raw: serde_yaml_ng::Value = match serde_yaml_ng::from_str(yaml_str) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let Some(adapters) = raw.get("adapters").and_then(|a| a.as_sequence()) else {
        return Vec::new();
    };
    adapters
        .iter()
        .filter_map(|entry| {
            let name = entry.get("name")?.as_str()?;
            if name == "architecture" {
                return None;
            }
            // Support both "enabled" (bool) and "status" (string) fields
            let enabled = entry
                .get("enabled")
                .and_then(|v| v.as_bool())
                .or_else(|| {
                    entry
                        .get("status")
                        .and_then(|s| s.as_str())
                        .map(|s| s != "disabled")
                })
                .unwrap_or(true);
            if enabled {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect()
}

pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig {
    parse_config_yaml_with_warnings(yaml_str).0
}

pub fn parse_config_yaml_with_warnings(yaml_str: &str) -> (ArchitectureConfig, Vec<String>) {
    let mut warnings = Vec::new();

    let raw: serde_yaml_ng::Value = match serde_yaml_ng::from_str(yaml_str) {
        Ok(v) => v,
        Err(e) => {
            warnings.push(format!("Failed to parse YAML: {}; using defaults", e));
            return (ArchitectureConfig::default(), warnings);
        }
    };
    if let Some(arch_val) = raw.get("architecture") {
        let mut arch_json: serde_json::Value = serde_json::to_value(arch_val).unwrap_or_default();
        if arch_json.get("layers").is_none()
            && let Some(rules_obj) = arch_json.get_mut("rules").and_then(|r| r.as_object_mut())
        {
            for (_rule_code, rule_val) in rules_obj.iter_mut() {
                if let Some(layers) = rule_val.get_mut("layers") {
                    let layers = std::mem::take(layers);
                    arch_json["layers"] = layers;
                    break;
                }
            }
        }
        let mut json = arch_json;
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
        remove_nulls(&mut json);
        if let Some(arr) = json.get("ignored_paths").and_then(|v| v.as_array()) {
            json["ignored_paths"] = serde_json::json!({"values": arr});
        }
        if let Some(layers_obj) = json.get_mut("layers")
            && let Some(obj) = layers_obj.as_object_mut()
        {
            let mut suffix_updates: Vec<(
                String,
                Option<String>,
                serde_json::Value,
                serde_json::Value,
            )> = Vec::new();
            for (layer_name, layer) in obj.iter() {
                if let Some(suffix_val) = layer.get("suffix")
                    && let Some(arr) = suffix_val.as_array()
                {
                    let mut policy: Option<String> = None;
                    let mut allowed = serde_json::Value::Array(Vec::new());
                    let mut forbidden = serde_json::Value::Array(Vec::new());
                    for entry in arr {
                        if let Some(entry_obj) = entry.as_object() {
                            for (pkey, plist) in entry_obj {
                                match pkey.as_str() {
                                    "strict" | "flexible" => {
                                        policy = Some(pkey.clone());
                                        if let Some(list) = plist.as_array() {
                                            allowed = serde_json::json!(list);
                                        }
                                    }
                                    "forbidden" => {
                                        if let Some(list) = plist.as_array() {
                                            forbidden = serde_json::json!(list);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    suffix_updates.push((layer_name.clone(), policy, allowed, forbidden));
                }
            }
            for (name, policy, allowed, forbidden) in suffix_updates {
                if let Some(layer) = obj.get_mut(&name)
                    && let Some(layer_obj) = layer.as_object_mut()
                {
                    // Build the naming sub-object for LayerDefinition.naming: LayerNamingConfig
                    let mut naming_obj = serde_json::Map::new();
                    if let Some(ref p) = policy {
                        naming_obj.insert("suffix_policy".to_string(), serde_json::json!(p));
                    }
                    naming_obj.insert("allowed_suffix".to_string(), allowed);
                    if let Some(arr) = forbidden.as_array()
                        && !arr.is_empty()
                    {
                        naming_obj.insert("forbidden_suffix".to_string(), forbidden);
                    }
                    layer_obj.insert("naming".to_string(), serde_json::Value::Object(naming_obj));
                    layer_obj.remove("suffix");
                }
            }
        }
        if let Some(rules_obj) = json.get_mut("rules")
            && let Some(obj) = rules_obj.as_object_mut()
        {
            let mut flat = serde_json::Value::Array(Vec::new());
            for (code, rule_val) in obj.iter() {
                if let Some(rule_obj) = rule_val.as_object() {
                    let mut base = rule_obj.clone();
                    base.insert("name".to_string(), serde_json::json!(code));
                    if let Some(scope_arr) = base.get("scope").and_then(|s| s.as_array()) {
                        if !base.contains_key("conditions") && scope_arr.len() > 1 {
                            for scope_val in scope_arr {
                                if let Some(s) = scope_val.as_str() {
                                    let mut entry = base.clone();
                                    entry.insert("scope".to_string(), serde_json::json!(s));
                                    if let Some(arr) = flat.as_array_mut() {
                                        arr.push(serde_json::Value::Object(entry));
                                    }
                                }
                            }
                            continue;
                        } else if let Some(first) = scope_arr.first().and_then(|v| v.as_str()) {
                            base.insert("scope".to_string(), serde_json::json!(first));
                        }
                    }
                    if let Some(conditions) = base.remove("conditions") {
                        let mut pushed = false;
                        if let Some(conds) = conditions.as_array() {
                            if conds.is_empty() {
                                if let Some(arr) = flat.as_array_mut() {
                                    arr.push(serde_json::Value::Object(base.clone()));
                                }
                                pushed = true;
                            } else {
                                for cond in conds {
                                    if let Some(cond_obj) = cond.as_object() {
                                        let mut entry = base.clone();
                                        for (k, v) in cond_obj {
                                            entry.insert(k.clone(), v.clone());
                                        }
                                        if let Some(arr) = flat.as_array_mut() {
                                            arr.push(serde_json::Value::Object(entry));
                                        }
                                        pushed = true;
                                    }
                                }
                            }
                        }
                        if !pushed && let Some(arr) = flat.as_array_mut() {
                            arr.push(serde_json::Value::Object(base));
                        }
                    } else {
                        if let Some(arr) = flat.as_array_mut() {
                            arr.push(serde_json::Value::Object(base));
                        }
                    }
                }
            }
            *rules_obj = flat;
        }
        let mut config = match serde_json::from_value::<ArchitectureConfig>(json) {
            Ok(c) => c,
            Err(e) => {
                warnings.push(format!("Failed to deserialize ArchitectureConfig: {:?}", e));
                warnings.push(
                    "Falling back to default config. Check your YAML syntax and field types."
                        .to_string(),
                );
                ArchitectureConfig::default()
            }
        };
        // Default orphan.check_orphan to true for all layers when not explicitly set.
        // BooleanVO defaults to false, but orphan detection should be enabled by default.
        for def in config.layers.values_mut() {
            if !def.orphan.check_orphan.value && def.orphan.exceptions.values.is_empty() {
                def.orphan.check_orphan = BooleanVO::new(true);
            }
        }
        if config.ignored_paths.values.is_empty()
            && let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence())
        {
            let paths: Vec<_> = arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                .collect();
            if !paths.is_empty() {
                config.ignored_paths = FilePathList::new(paths);
            }
        }
        (config, warnings)
    } else {
        let mut config = ArchitectureConfig::default();
        if let Some(arr) = raw.get("ignored_paths").and_then(|v| v.as_sequence()) {
            let paths: Vec<_> = arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                .collect();
            if !paths.is_empty() {
                config.ignored_paths = FilePathList::new(paths);
            }
        }
        (config, warnings)
    }
}

/// Parse adapter entries from YAML, returning `Vec<AdapterEntry>` with name,
/// status, weight, and timeout fields. Skips disabled adapters.
///
/// Supports the Appendix A YAML schema:
/// ```yaml
/// adapters:
///   - name: "clippy"
///     enabled: true
///     weight: 1.0
///     timeout: 120
/// ```
pub fn parse_adapter_entries_from_yaml(
    yaml_str: &str,
) -> Vec<shared::config_system::taxonomy_setting_vo::AdapterEntry> {
    use shared::common::taxonomy_adapter_name_vo::AdapterName;
    use shared::config_system::taxonomy_setting_vo::{AdapterEntry, AdapterStatus};
    let raw: serde_yaml_ng::Value = match serde_yaml_ng::from_str(yaml_str) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let Some(adapters) = raw.get("adapters").and_then(|a| a.as_sequence()) else {
        return Vec::new();
    };
    adapters
        .iter()
        .filter_map(|entry| {
            let name = entry.get("name")?.as_str()?;
            if name == "architecture" {
                return None;
            }
            // Support both "status" (existing) and "enabled" (FRD Appendix A) fields
            let enabled = entry
                .get("enabled")
                .and_then(|v| v.as_bool())
                .or_else(|| {
                    entry
                        .get("status")
                        .and_then(|s| s.as_str())
                        .map(|s| s != "disabled")
                })
                .unwrap_or(true);
            if !enabled {
                return None;
            }
            let weight = entry.get("weight").and_then(|v| v.as_f64()).unwrap_or(1.0);
            let timeout = entry
                .get("timeout")
                .and_then(|v| v.as_f64())
                .unwrap_or(60.0);
            Some(AdapterEntry::with_timeout(
                AdapterName::raw(name.to_string()),
                AdapterStatus::Enabled,
                weight,
                timeout,
            ))
        })
        .collect()
}
