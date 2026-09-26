// PURPOSE: IImportRunnerAggregate — single entry point over the import-rules domain
// The agent behind the aggregate dispatches each ImportRequest to the rich
// import-protocol operations. Consumers never see the protocols.
use crate::import_rules::taxonomy_import_request_vo::{ImportRequest, ImportResponse};

/// Single entry point over import-rules; the agent dispatches internally.
pub trait IImportRunnerAggregate: Send + Sync {
    fn execute(&self, request: ImportRequest) -> ImportResponse;
}
