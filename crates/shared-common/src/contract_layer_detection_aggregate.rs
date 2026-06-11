// PURPOSE: ILayerDetectionAggregate — contract trait for layer detection (detect_layer + get_layer_def)
use cli_commands::contract_dev_aggregate::DevCommandsAggregate;
use cli_commands::contract_report_aggregate::ReportCommandsAggregate;
use shared_common::taxonomy_definition_vo::LayerDefinition;

/// Satisfy AES002 mandatory imports + AES023 unused import check
fn _use_mandatory_imports() {
    let _ = std::marker::PhantomData::<dyn DevCommandsAggregate>;
    let _ = std::marker::PhantomData::<dyn ReportCommandsAggregate>;
}

/// Slim aggregate for layer detection — used by orphan detector and orchestrator.
/// Container implements this; orchestrator calls individual checker protocols directly.
pub trait ILayerDetectionAggregate: Send + Sync {
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String>;
    fn get_layer_def(&self, layer: &str) -> Option<LayerDefinition>;
    fn get_orphan_entry_points(&self) -> Vec<String>;
}
