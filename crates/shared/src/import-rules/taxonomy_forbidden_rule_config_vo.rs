// PURPOSE: ForbiddenRuleConfigVO — Value object for forbidden import rule configuration
use crate::taxonomy_layer_vo::LayerNameVO;

pub struct ForbiddenRuleConfigVO<'a> {
    pub forbidden_list: &'a [String],
    pub source_layer: &'a LayerNameVO,
    pub allowed_values: &'a [String],
}
