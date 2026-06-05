use super::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
            enabled: BooleanVO::new(false),
            layers: std::collections::HashMap::new(),
            rules: Vec::new(),
            governance_rules: LegacyLayerRuleList::new(vec![]),
            naming: NamingConfig::new(Count::new(0), ErrorMessage::new(String::new())),
            ignored_paths: FilePathList { values: vec![] },
            mandatory_import_violation_message: ErrorMessage::new(String::new()),
            mandatory_class_definition: BooleanVO::new(false),
            mandatory_class_definition_violation_message: ErrorMessage::new(String::new()),
        }
    }
}
