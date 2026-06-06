use serde::{Deserialize, Serialize};

use crate::taxonomy::{
    ArchitectureRule, BooleanVO, Count, ErrorMessage, FilePathList, LayerDefinition,
    LayerNameVO, LegacyLayerRuleList, NamingConfig,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct ArchitectureConfig {
    pub enabled: BooleanVO,
    pub layers: std::collections::HashMap<LayerNameVO, LayerDefinition>,
    pub rules: Vec<ArchitectureRule>,
    pub governance_rules: LegacyLayerRuleList,
    pub naming: NamingConfig,
    pub ignored_paths: FilePathList,
    pub mandatory_import_violation_message: ErrorMessage,
    pub mandatory_class_definition: BooleanVO,
    pub mandatory_class_definition_violation_message: ErrorMessage,
}

impl ArchitectureConfig {
    pub fn new(
        enabled: BooleanVO,
        layers: std::collections::HashMap<LayerNameVO, LayerDefinition>,
        rules: Vec<ArchitectureRule>,
        governance_rules: LegacyLayerRuleList,
        naming: NamingConfig,
        ignored_paths: FilePathList,
        mandatory_import_violation_message: ErrorMessage,
        mandatory_class_definition: BooleanVO,
        mandatory_class_definition_violation_message: ErrorMessage,
    ) -> Self {
        Self {
            enabled,
            layers,
            rules,
            governance_rules,
            naming,
            ignored_paths,
            mandatory_import_violation_message,
            mandatory_class_definition,
            mandatory_class_definition_violation_message,
        }
    }
}

impl Default for ArchitectureConfig {
    fn default() -> Self {
        Self {
            enabled: BooleanVO::new(true),
            layers: HashMap::new(),
            rules: Vec::new(),
            governance_rules: LegacyLayerRuleList::new(vec![]),
            naming: NamingConfig::new(Count::new(3), ErrorMessage::new(String::new())),
            ignored_paths: FilePathList { values: vec![] },
            mandatory_import_violation_message: ErrorMessage::new(String::new()),
            mandatory_class_definition: BooleanVO::new(false),
            mandatory_class_definition_violation_message: ErrorMessage::new(String::new()),
        }
    }
}

/// Build default config by parsing the embedded YAML file at compile time.
/// The YAML (`lint_arwaky.config.rust.yaml`) is baked into the binary via `include_str!`.
/// Project-level `lint_arwaky.config.rust.yaml` at runtime will override this default.
pub fn default_aes_config() -> ArchitectureConfig {
    let yaml_str = include_str!("../../lint_arwaky.config.rust.yaml");
    let raw: serde_yaml::Value = serde_yaml::from_str(yaml_str).unwrap_or_default();
    if let Some(arch_val) = raw.get("architecture") {
        let mut json = serde_json::to_value(arch_val).unwrap_or_default();
        // Remove null values so #[serde(default)] kicks in for missing fields
        fn remove_nulls(val: &mut serde_json::Value) {
            match val {
                serde_json::Value::Object(m) => {
                    m.retain(|_, v| !v.is_null());
                    for v in m.values_mut() { remove_nulls(v); }
                }
                serde_json::Value::Array(arr) => {
                    for v in arr.iter_mut() { remove_nulls(v); }
                }
                _ => {}
            }
        }
        remove_nulls(&mut json);
        // Convert YAML suffix format to struct fields
        if let Some(layers_obj) = json.get_mut("layers") {
            if let Some(obj) = layers_obj.as_object_mut() {
                let mut suffix_updates: Vec<(String, Option<String>, serde_json::Value, serde_json::Value)> = Vec::new();
                for (layer_name, layer) in obj.iter() {
                    if let Some(suffix_val) = layer.get("suffix") {
                        if let Some(arr) = suffix_val.as_array() {
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
                }
                for (name, policy, allowed, forbidden) in suffix_updates {
                    if let Some(layer) = obj.get_mut(&name) {
                        if let Some(layer_obj) = layer.as_object_mut() {
                            if let Some(ref p) = policy {
                                layer_obj.insert("suffix_policy".to_string(), serde_json::json!(p));
                            }
                            layer_obj.insert("allowed_suffix".to_string(), allowed);
                            if let Some(arr) = forbidden.as_array() {
                                if !arr.is_empty() {
                                    layer_obj.insert("forbidden_suffix".to_string(), forbidden);
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
                            flat.as_array_mut().unwrap().push(item.clone());
                        }
                    }
                }
                *rules_obj = flat;
            }
        }
        serde_json::from_value::<ArchitectureConfig>(json).unwrap_or_default()
    } else {
        ArchitectureConfig::default()
    }
}
