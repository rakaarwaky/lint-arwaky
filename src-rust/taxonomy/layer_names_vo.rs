use super::*;

pub const LAYER_AGENT: &str = "agent";
pub const LAYER_CAPABILITIES: &str = "capabilities";
pub const LAYER_CONTRACT: &str = "contract";
pub const LAYER_INFRASTRUCTURE: &str = "infrastructure";
pub const LAYER_SURFACES: &str = "surfaces";
pub const LAYER_TAXONOMY: &str = "taxonomy";
pub const LAYER_ROOT: &str = "root";
pub const LAYER_GLOBAL: &str = "global";

pub fn layer_agent() -> LayerNameVO { LayerNameVO::new(LAYER_AGENT) }
pub fn layer_capabilities() -> LayerNameVO { LayerNameVO::new(LAYER_CAPABILITIES) }
pub fn layer_taxonomy() -> LayerNameVO { LayerNameVO::new(LAYER_TAXONOMY) }
pub fn layer_contract() -> LayerNameVO { LayerNameVO::new(LAYER_CONTRACT) }
pub fn layer_infrastructure() -> LayerNameVO { LayerNameVO::new(LAYER_INFRASTRUCTURE) }
pub fn layer_surfaces() -> LayerNameVO { LayerNameVO::new(LAYER_SURFACES) }
pub fn layer_root() -> LayerNameVO { LayerNameVO::new(LAYER_ROOT) }
pub fn layer_global() -> LayerNameVO { LayerNameVO::new(LAYER_GLOBAL) }

pub fn all_core_layers() -> Vec<LayerNameVO> {
    vec![
        layer_agent(),
        layer_capabilities(),
        layer_taxonomy(),
        layer_contract(),
        layer_infrastructure(),
        layer_surfaces(),
        layer_root(),
    ]
}

pub fn core_layer_names() -> std::collections::HashSet<String> {
    all_core_layers().iter().map(|l| l.value.clone()).collect()
}
