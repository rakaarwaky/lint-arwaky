// PURPOSE: Port: Interface for Variant

use crate::naming_rules::taxonomy_name_vo::SymbolName;
use crate::naming_rules::taxonomy_naming_list_vo::SymbolNameList;

pub trait INamingVariantPort: Send + Sync {
    fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value;
    fn build_variants(&self, name: &SymbolName) -> SymbolNameList;
}
