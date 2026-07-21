// PURPOSE: IExternalLintSelectorProtocol — protocol for selecting adapters based on detected languages
use async_trait::async_trait;

#[async_trait]
pub trait IExternalLintSelectorProtocol: Send + Sync {
    /// Select which adapter names to run based on which languages are present.
    fn select_adapters(
        &self,
        has_rs: bool,
        has_py: bool,
        has_js: bool,
    ) -> Vec<String>;
}
