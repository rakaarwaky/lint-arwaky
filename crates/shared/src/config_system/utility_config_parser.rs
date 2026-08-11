// PURPOSE: Taxonomy parser — YAML → ArchitectureConfig factory functions
// These functions produce taxonomy VOs from raw YAML, so they belong in taxonomy layer.
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_paths_vo::FilePathList;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use std::sync::OnceLock;

static DEFAULT_CONFIG: OnceLock<ArchitectureConfig> = OnceLock::new();

/// Returns the full default AES config parsed from `lint_arwaky.config.yaml`.
pub fn default_aes_config() -> ArchitectureConfig {
    DEFAULT_CONFIG
        .get_or_init(|| parse_config_yaml(include_str!("../../config/lint_arwaky.config.yaml")))
        .clone()
}

/// Returns default config for a given language, or empty config for unknown languages.
pub fn default_config_for_language(language: &str) -> ArchitectureConfig {
    match language {
        "rust" | "python" | "javascript" | "typescript" => default_aes_config(),
        _ => {
            tracing::warn!(
                language = language,
                "unknown language, using empty default config"
            );
            ArchitectureConfig::default()
        }
    }
}

pub fn parse_config_yaml(yaml_str: &str) -> ArchitectureConfig {
    parse_config_yaml_with_warnings(yaml_str).0
}

/// Parse YAML into an `ArchitectureConfig`, collecting non-fatal warnings.
pub fn parse_config_yaml_with_warnings(yaml_str: &str) -> (ArchitectureConfig, Vec<String>) {
    let mut warnings = Vec::new();

    let raw: serde_yaml_ng::Value = match serde_yaml_ng::from_str(yaml_str) {
        Ok(v) => v,
        Err(e) => {
            warnings.push(format!("Failed to parse YAML: {}; using defaults", e));
            return (ArchitectureConfig::default(), warnings);
        }
    };

    // Fallback path: no "architecture" key — just extract ignored_paths if present.
    if raw.get("architecture").is_none() {
        return (config_with_ignored_paths(&raw), warnings);
    }

    // Main path: extract architecture section and build JSON.
    let arch_json = match raw.get("architecture") {
        Some(val) => serde_json::to_value(val).unwrap_or_default(),
        None => return (ArchitectureConfig::default(), warnings),
    };

    let mut config = deserialize_config(preprocess_json(arch_json), &mut warnings);
    config = enable_default_orphan_detection(config);
    config = apply_fallback_ignored_paths(config, &raw);
    (config, warnings)
}

/// Build a default config carrying only the raw `ignored_paths` entries.
fn config_with_ignored_paths(raw: &serde_yaml_ng::Value) -> ArchitectureConfig {
    let mut config = ArchitectureConfig::default();
    let paths = parse_ignored_paths(raw);
    if !paths.values.is_empty() {
        config.ignored_paths = paths;
    }
    config
}

/// Extract `ignored_paths` from raw YAML as a `FilePathList` (empty when absent).
fn parse_ignored_paths(raw: &serde_yaml_ng::Value) -> FilePathList {
    let paths: Vec<FilePath> = raw
        .get("ignored_paths")
        .and_then(|v| v.as_sequence())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| FilePath::new(s.to_string()).unwrap_or_default())
                .collect()
        })
        .unwrap_or_default();
    FilePathList::new(paths)
}

/// Preprocess the architecture JSON: migrate legacy layers field, strip nulls,
/// convert ignored_paths format, and apply layer suffix → naming migration.
fn preprocess_json(mut json: serde_json::Value) -> serde_json::Value {
    // Legacy: move `rules.*.layers` → top-level `layers` when no top-level layers exist.
    migrate_legacy_layers(&mut json);

    // Strip null values recursively.
    remove_nulls(&mut json);

    // Convert ignored_paths from array to {"values": [...]} wrapper.
    if let Some(arr) = json.get("ignored_paths").and_then(|v| v.as_array()) {
        json["ignored_paths"] = serde_json::json!({"values": arr});
    }

    // Migrate layer suffix arrays → naming config objects.
    if let Some(layers_obj) = json.get_mut("layers") {
        if let Some(obj) = layers_obj.as_object_mut() {
            migrate_layer_naming(obj);
        }
    }

    // Flatten rules with scope/conditions into a flat array.
    if let Some(rules_val) = json.get_mut("rules") {
        flatten_rules(rules_val);
    }

    json
}

/// Legacy: promote `rules.*.layers` to a top-level `layers` key when missing.
fn migrate_legacy_layers(json: &mut serde_json::Value) {
    if json.get("layers").is_some() {
        return;
    }
    let Some(rules_obj) = json.get_mut("rules").and_then(|r| r.as_object_mut()) else {
        return;
    };
    for (_code, rule_val) in rules_obj.iter_mut() {
        if let Some(layers) = rule_val.get_mut("layers") {
            let layers = std::mem::take(layers);
            json["layers"] = layers;
            break;
        }
    }
}

/// Recursively remove null values from a JSON value.
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

/// Migrate legacy `suffix` arrays into `naming` config objects per layer.
fn migrate_layer_naming(obj: &mut serde_json::Map<String, serde_json::Value>) {
    let mut suffix_updates: Vec<(String, Option<String>, serde_json::Value, serde_json::Value)> =
        Vec::new();

    for (layer_name, layer) in obj.iter() {
        if let Some(update) = extract_suffix_policy(layer) {
            suffix_updates.push((layer_name.clone(), update.0, update.1, update.2));
        }
    }

    for (name, policy, allowed, forbidden) in suffix_updates {
        apply_suffix_update(obj, &name, policy, allowed, forbidden);
    }
}

/// Extract `(policy, allowed, forbidden)` from a layer's legacy `suffix` array.
fn extract_suffix_policy(
    layer: &serde_json::Value,
) -> Option<(Option<String>, serde_json::Value, serde_json::Value)> {
    let suffix_val = layer.get("suffix")?;
    let arr = suffix_val.as_array()?;
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
    Some((policy, allowed, forbidden))
}

/// Replace a layer's `suffix` array with a `naming` object built from the
/// extracted policy.
fn apply_suffix_update(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    name: &str,
    policy: Option<String>,
    allowed: serde_json::Value,
    forbidden: serde_json::Value,
) {
    let Some(layer) = obj.get_mut(name) else { return };
    let Some(layer_obj) = layer.as_object_mut() else { return };

    let mut naming_obj = serde_json::Map::new();
    if let Some(ref p) = policy {
        naming_obj.insert("suffix_policy".to_string(), serde_json::json!(p));
    }
    naming_obj.insert("allowed_suffix".to_string(), allowed);
    if let Some(arr) = forbidden.as_array() && !arr.is_empty() {
        naming_obj.insert("forbidden_suffix".to_string(), forbidden);
    }
    layer_obj.insert("naming".to_string(), serde_json::Value::Object(naming_obj));
    layer_obj.remove("suffix");
}

/// Flatten rules from `{"code": {...}}` → flat array, expanding scope/conditions.
/// The `rules` value is replaced in place with the flat array.
fn flatten_rules(rules_val: &mut serde_json::Value) {
    let Some(obj) = rules_val.as_object_mut() else {
        return;
    };
    let mut flat = serde_json::Value::Array(Vec::new());

    for (code, rule_val) in obj.iter() {
        let Some(rule_obj) = rule_val.as_object() else {
            continue;
        };

        let mut base = rule_obj.clone();
        base.insert("name".to_string(), serde_json::json!(code));
        push_expanded_rule(&mut flat, serde_json::Value::Object(base));
    }

    *rules_val = flat;
}

/// Push a single rule into `flat`, expanding multi-scope rules and conditions.
fn push_expanded_rule(flat: &mut serde_json::Value, base: serde_json::Value) {
    let mut base = base;
    if expand_scopes(flat, &mut base) {
        return;
    }
    expand_conditions(flat, base);
}

/// Expand multi-scope rules into one entry per scope. Returns true when the
/// rule was pushed as scope-specific entries and needs no further handling.
fn expand_scopes(flat: &mut serde_json::Value, base: &mut serde_json::Value) -> bool {
    let scope_arr = match base.get("scope") {
        Some(s) => match s.as_array() {
            Some(a) => a,
            None => return false,
        },
        None => return false,
    };
    let scopes: Vec<String> = scope_arr
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    let has_conditions = base
        .as_object()
        .is_some_and(|o| o.contains_key("conditions"));

    if !has_conditions && scopes.len() > 1 {
        // No conditions: expand one entry per scope.
        for s in scopes {
            let mut entry = base.clone();
            if let Some(obj) = entry.as_object_mut() {
                obj.insert("scope".to_string(), serde_json::json!(s));
            }
            push_entry(flat, entry);
        }
        return true;
    }

    // Otherwise (single scope, or scopes combined with conditions): collapse
    // to the first scope on the base entry (original behavior).
    if let Some(first) = scopes.first() {
        if let Some(obj) = base.as_object_mut() {
            obj.insert("scope".to_string(), serde_json::json!(first));
        }
    }
    false
}

/// Expand `conditions` into individual entries, or push the base entry itself
/// when there are no conditions.
fn expand_conditions(flat: &mut serde_json::Value, mut base: serde_json::Value) {
    // Remove `conditions` so expanded entries do not carry the key (original behavior).
    let conditions = base
        .as_object_mut()
        .and_then(|obj| obj.remove("conditions"));
    let Some(conditions) = conditions else {
        push_entry(flat, base);
        return;
    };
    let Some(conds) = conditions.as_array() else {
        push_entry(flat, base);
        return;
    };
    if conds.is_empty() {
        push_entry(flat, base);
        return;
    }
    for cond in conds {
        if let Some(cond_obj) = cond.as_object() {
            let mut entry = base.clone();
            if let Some(entry_obj) = entry.as_object_mut() {
                for (k, v) in cond_obj {
                    entry_obj.insert(k.clone(), v.clone());
                }
            }
            push_entry(flat, entry);
        }
    }
}

/// Push one rule object onto the `flat` array.
fn push_entry(flat: &mut serde_json::Value, entry: serde_json::Value) {
    if let Some(arr) = flat.as_array_mut() {
        arr.push(entry);
    }
}

/// Deserialize JSON into ArchitectureConfig, collecting warnings on failure.
fn deserialize_config(json: serde_json::Value, warnings: &mut Vec<String>) -> ArchitectureConfig {
    match serde_json::from_value::<ArchitectureConfig>(json) {
        Ok(c) => c,
        Err(e) => {
            warnings.push(format!("Failed to deserialize ArchitectureConfig: {:?}", e));
            warnings.push(
                "Falling back to default config. Check your YAML syntax and field types."
                    .to_string(),
            );
            ArchitectureConfig::default()
        }
    }
}

/// Enable orphan detection by default for all layers (BooleanVO defaults to false).
fn enable_default_orphan_detection(mut config: ArchitectureConfig) -> ArchitectureConfig {
    for def in config.layers.values_mut() {
        if !def.orphan.check_orphan.value {
            def.orphan.check_orphan = BooleanVO::new(true);
        }
    }
    config
}

/// Apply fallback ignored_paths from raw YAML when the deserialized config has none.
fn apply_fallback_ignored_paths(
    mut config: ArchitectureConfig,
    raw: &serde_yaml_ng::Value,
) -> ArchitectureConfig {
    if config.ignored_paths.values.is_empty() {
        let paths = parse_ignored_paths(raw);
        if !paths.values.is_empty() {
            config.ignored_paths = paths;
        }
    }
    config
}
