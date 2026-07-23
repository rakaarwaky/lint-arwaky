// PURPOSE: SuffixPolicyVO, SuffixVO — value objects for suffix naming rules
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(transparent)]
pub struct SuffixPolicyVO {
    pub value: String,
}

impl SuffixPolicyVO {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_empty(&self) -> bool {
        self.value.trim().is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SuffixVO {
    pub values: crate::common::taxonomy_common_vo::PatternList,
}

impl SuffixVO {
    pub fn new(values: crate::common::taxonomy_common_vo::PatternList) -> Self {
        Self { values }
    }

    pub fn is_empty(&self) -> bool {
        self.values.values.is_empty()
    }
}
