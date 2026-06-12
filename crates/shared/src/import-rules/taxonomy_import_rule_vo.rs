// PURPOSE: CustomMessageVO, LegacyLayerRule, LegacyLayerRuleList, MandatoryImportRuleVO — VOs for AES rule definitions
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_layer_vo::LayerNameVO;
use crate::naming_rules::taxonomy_suffix_vo::SuffixVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomMessageVO {
    pub pattern: String,
    pub message: ErrorMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LegacyLayerRule {
    pub source_layer: LayerNameVO,
    pub forbidden_target: LayerNameVO,
    pub description: ErrorMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LegacyLayerRuleList {
    pub values: Vec<LegacyLayerRule>,
}

impl LegacyLayerRuleList {
    pub fn new(value: Vec<LegacyLayerRule>) -> Self {
        Self { values: value }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LegacyLayerRule> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LegacyLayerRule) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MandatoryImportRuleVO {
    pub suffix: SuffixVO,
    pub imports: PatternList,
}

impl CustomMessageVO {
    pub fn new(pattern: String, message: ErrorMessage) -> Self {
        Self { pattern, message }
    }
}

impl MandatoryImportRuleVO {
    pub fn new(suffix: SuffixVO, imports: PatternList) -> Self {
        Self { suffix, imports }
    }
}

impl LegacyLayerRule {
    pub fn new(
        source_layer: LayerNameVO,
        forbidden_target: LayerNameVO,
        description: ErrorMessage,
    ) -> Self {
        Self {
            source_layer,
            forbidden_target,
            description,
        }
    }
}
