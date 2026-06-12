// PURPOSE: LayerDefinition, LayerMapVO, NamingConfig — VOs for AES layer definitions and naming policies
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::import_rules::taxonomy_import_rule_vo::MandatoryImportRuleVO;
use crate::naming_rules::taxonomy_suffix_vo::SuffixPolicyVO;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LayerDefinition {
    pub path: DirectoryPath,
    #[serde(default)]
    pub allowed: PatternList,
    #[serde(default)]
    pub forbidden: PatternList,
    #[serde(default)]
    pub mandatory: PatternList,
    #[serde(default)]
    pub word_count: Count,
    #[serde(default)]
    pub exceptions: PatternList,
    #[serde(default)]
    pub recursive: BooleanVO,

    #[serde(flatten)]
    pub naming: crate::naming_rules::taxonomy_naming_rule_vo::NamingRuleVO,
    #[serde(flatten)]
    pub code_analysis: crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO,
    #[serde(flatten)]
    pub role: crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO,
    #[serde(flatten)]
    pub orphan: crate::orphan_detector::taxonomy_orphan_rule_vo::OrphanRuleVO,
}

impl LayerDefinition {
    pub fn path_str(&self) -> String {
        self.path.value.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LayerMapVO {
    pub values: std::collections::HashMap<LayerNameVO, LayerDefinition>,
}

impl LayerMapVO {
    pub fn new(value: std::collections::HashMap<LayerNameVO, LayerDefinition>) -> Self {
        Self { values: value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct NamingConfig {
    pub word_count: Count,
}

impl NamingConfig {
    pub fn new(word_count: Count) -> Self {
        Self { word_count }
    }
}
