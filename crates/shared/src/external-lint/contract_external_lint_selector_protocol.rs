// PURPOSE: IExternalLintSelectorProtocol — protocol for selecting adapters based on detected languages
use crate::common::taxonomy_common_vo::BooleanVO;
use async_trait::async_trait;

#[async_trait]
pub trait IExternalLintSelectorProtocol: Send + Sync {
    /// Select which adapter names to run based on which languages are present.
    fn select_adapters(
        &self,
        has_rs: BooleanVO,
        has_py: BooleanVO,
        has_js: BooleanVO,
    ) -> Vec<String>;
}
