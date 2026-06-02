"""layer_names — Standardized architectural layer names."""

from .layer_content_vo import LayerNameVO

# Core Layers
LAYER_AGENT = LayerNameVO(value="agent")
LAYER_CAPABILITIES = LayerNameVO(value="capabilities")
LAYER_TAXONOMY = LayerNameVO(value="taxonomy")
LAYER_CONTRACT = LayerNameVO(value="contract")
LAYER_INFRASTRUCTURE = LayerNameVO(value="infrastructure")
LAYER_SURFACES = LayerNameVO(value="surfaces")
LAYER_ROOT = LayerNameVO(value="root")

# Global Scope
LAYER_GLOBAL = LayerNameVO(value="global")

ALL_CORE_LAYERS = [
    LAYER_AGENT,
    LAYER_CAPABILITIES,
    LAYER_TAXONOMY,
    LAYER_CONTRACT,
    LAYER_INFRASTRUCTURE,
    LAYER_SURFACES,
    LAYER_ROOT,
]

CORE_LAYER_NAMES = {str(layer) for layer in ALL_CORE_LAYERS}
