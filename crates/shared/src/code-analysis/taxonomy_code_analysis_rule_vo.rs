// PURPOSE: CodeAnalysisRuleVO — value object containing code analysis and line checker rule definitions
use crate::common::taxonomy_common_vo::bool;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::import_rules::taxonomy_import_rule_vo::MandatoryImportRuleVO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CodeAnalysisRuleVO {
    #[serde(default)]
    pub min_lines: Count,
    #[serde(default)]
    pub max_lines: Count,
    #[serde(default)]
    pub forbidden_bypass: PatternList,
    #[serde(default)]
    pub mandatory_class_definition: bool,
    #[serde(default)]
    pub dead_inheritance_bypass: bool,
    #[serde(default)]
    pub check_unused_mandatory_imports: bool,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
    #[serde(default)]
    pub forbid_any_type: bool,
    #[serde(default)]
    pub mandatory_imports: Vec<MandatoryImportRuleVO>,
    #[serde(default)]
    pub duplication_threshold: Option<f64>,
}
