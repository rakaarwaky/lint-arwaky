// PURPOSE: IExternalLintAggregate — single entry point over the external-lint domain
use crate::external_lint::taxonomy_external_lint_request_vo::{
    ExternalLintRequest, ExternalLintResponse,
};

/// Single entry point over external-lint; the agent dispatches internally.
pub trait IExternalLintAggregate: Send + Sync {
    fn execute(&self, request: ExternalLintRequest) -> ExternalLintResponse;
}
