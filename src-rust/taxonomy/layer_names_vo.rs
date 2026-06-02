use super::*;

pub fn layer_agent() -> LayerNameVO { LayerNameVO::new("agent") }
pub fn layer_capabilities() -> LayerNameVO { LayerNameVO::new("capabilities") }
pub fn layer_taxonomy() -> LayerNameVO { LayerNameVO::new("taxonomy") }
pub fn layer_contract() -> LayerNameVO { LayerNameVO::new("contract") }
pub fn layer_infrastructure() -> LayerNameVO { LayerNameVO::new("infrastructure") }
pub fn layer_surfaces() -> LayerNameVO { LayerNameVO::new("surfaces") }
pub fn layer_root() -> LayerNameVO { LayerNameVO::new("root") }
pub fn layer_global() -> LayerNameVO { LayerNameVO::new("global") }

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
