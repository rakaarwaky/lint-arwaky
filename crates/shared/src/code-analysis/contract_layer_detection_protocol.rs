// PURPOSE: ILayerDetectionProtocol — contract trait for layer detection (all methods)
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;

/// Full protocol for layer detection — used by all rule checkers and orchestrators.
/// Every fn in the impl file MUST have a corresponding fn here (1:1 fn-to-trait matching).
pub trait ILayerDetectionProtocol: Send + Sync {
    /// Construct a new LayerDetectionAnalyzer with merged rule configuration.
    fn new(config: ArchitectureConfig) -> Self;

    /// Detect layer from filename — exclusively via filename prefix (FRD v1.1).
    fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String>;

    /// Look up a LayerDefinition by its layer name string.
    fn get_layer_def(&self, layer: &str) -> Option<LayerDefinition>;

    /// Return known orphan entry point filenames.
    fn get_orphan_entry_points(&self) -> Vec<String>;

    /// Return the merged architecture configuration.
    fn config(&self) -> &ArchitectureConfig;

    /// Extract layer from filename prefix (FRD v1.1).
    fn extract_layer_from_prefix(&self, filename: &str) -> Option<String>;

    /// Resolve specialised sub-layer from file suffix (e.g., "capabilities(command").
    fn resolve_specialized_layer(&self, base_layer: &str, file_path: &str) -> String;

    /// Determine which architectural layer a module path belongs to.
    fn detect_module_layer(&self, module: &str) -> Option<String>;

    /// Refine a base layer to a specialised sub-layer from dotted module path.
    fn refine_module_layer(&self, base_name: &str, parts: &[&str]) -> String;
}
