// PURPOSE: PluginCommandsSurface — adapter/plugin listing business logic, no formatting.
use shared::common::AdapterNameList;
use shared::external_lint::IExternalLintAggregate;
use std::sync::Arc;

pub fn collect_adapters(external_lint: Arc<dyn IExternalLintAggregate>) -> AdapterNameList {
    external_lint.adapter_names()
}
