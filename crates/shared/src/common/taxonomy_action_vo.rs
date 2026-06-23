// PURPOSE: ActionName, ActionArgs — value objects for pipeline job actions
// JobId is re-exported from common for backward compatibility
pub use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_suggestion_vo::MetadataVO;
use crate::string_value_object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionArgs {
    pub value: MetadataVO,
}

impl ActionArgs {
    pub fn new(value: MetadataVO) -> Self {
        Self { value }
    }
    pub fn value(&self) -> &MetadataVO {
        &self.value
    }
}

string_value_object!(ActionName);
