// PURPOSE: INamingVariantPort — port for language-specific naming variant generation
use crate::taxonomy_name_vo::SymbolName;
use crate::language_adapters::taxonomy_naming_list_vo::SymbolNameList;

pub trait INamingVariantPort: Send + Sync {
    fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value;
    fn build_variants(&self, name: &SymbolName) -> SymbolNameList;
}
