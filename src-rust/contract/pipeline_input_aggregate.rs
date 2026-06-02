use crate::taxonomy::{FilePath, ContentString};

#[derive(Debug, Clone, Default)]
pub struct PipelineInputAggregate {
    pub root_path: Option<FilePath>,
    pub action: ContentString,
    pub path: Option<FilePath>,
    pub args: Option<serde_json::Value>,
    pub rules: Vec<String>,
    pub use_retry: Option<bool>,
}
