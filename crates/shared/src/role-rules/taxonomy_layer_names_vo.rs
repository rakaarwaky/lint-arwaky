// PURPOSE: LayerNames — value object for layer name collection and lookup
use crate::role_rules::taxonomy_layer_names_constant::LAYER_AGENT;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_CAPABILITIES;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_CONTRACT;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_GLOBAL;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_INFRASTRUCTURE;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_ROOT;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_SURFACES;
use crate::role_rules::taxonomy_layer_names_constant::LAYER_TAXONOMY;
use crate::common::taxonomy_layer_vo::LayerNameVO;

/// Value object holding the set of core layer names.
pub struct LayerNames {}

pub fn layer_agent() -> LayerNameVO {
    LayerNameVO::new(LAYER_AGENT)
}
pub fn layer_capabilities() -> LayerNameVO {
    LayerNameVO::new(LAYER_CAPABILITIES)
}
pub fn layer_taxonomy() -> LayerNameVO {
    LayerNameVO::new(LAYER_TAXONOMY)
}
pub fn layer_contract() -> LayerNameVO {
    LayerNameVO::new(LAYER_CONTRACT)
}
pub fn layer_infrastructure() -> LayerNameVO {
    LayerNameVO::new(LAYER_INFRASTRUCTURE)
}
pub fn layer_surfaces() -> LayerNameVO {
    LayerNameVO::new(LAYER_SURFACES)
}
pub fn layer_root() -> LayerNameVO {
    LayerNameVO::new(LAYER_ROOT)
}
pub fn layer_global() -> LayerNameVO {
    LayerNameVO::new(LAYER_GLOBAL)
}

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
