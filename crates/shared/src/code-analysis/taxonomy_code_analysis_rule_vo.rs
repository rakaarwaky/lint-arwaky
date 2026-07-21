// PURPOSE: CodeAnalysisRuleVO — value object containing code analysis and line checker rule definitions
use crate::common::taxonomy_common_vo::BooleanVO;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::PatternList;
use crate::import_rules::taxonomy_import_rule_vo::MandatoryImportRuleVO;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeAnalysisRuleVO {
    #[serde(default = "default_min_lines")]
    pub min_lines: Count,
    #[serde(default = "default_max_lines")]
    pub max_lines: Count,
    #[serde(default)]
    pub forbidden_bypass: PatternList,
    #[serde(default)]
    pub mandatory_class_definition: BooleanVO,
    #[serde(default)]
    pub dead_inheritance_bypass: BooleanVO,
    #[serde(default)]
    pub check_unused_mandatory_imports: BooleanVO,
    #[serde(default)]
    pub forbidden_inheritance: PatternList,
    #[serde(default)]
    pub forbid_any_type: BooleanVO,
    #[serde(default)]
    pub mandatory_imports: Vec<MandatoryImportRuleVO>,
    #[serde(default)]
    pub duplication_threshold: Option<f64>,
}

/// P0.1 fix: AES302 default threshold — FRD specifies 10 lines minimum.
fn default_min_lines() -> Count {
    Count::new(10)
}

/// AES301 default maximum file line count.
fn default_max_lines() -> Count {
    Count::new(1000)
}

impl Default for CodeAnalysisRuleVO {
    fn default() -> Self {
        Self {
            min_lines: default_min_lines(),
            max_lines: default_max_lines(),
            forbidden_bypass: PatternList::default(),
            mandatory_class_definition: BooleanVO::default(),
            dead_inheritance_bypass: BooleanVO::default(),
            check_unused_mandatory_imports: BooleanVO::default(),
            forbidden_inheritance: PatternList::default(),
            forbid_any_type: BooleanVO::default(),
            mandatory_imports: Vec::new(),
            duplication_threshold: None,
        }
    }
}
