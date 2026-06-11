// PURPOSE: INamingVariantPort — port trait for building naming variant dictionaries

use crate::language_adapters::taxonomy_naming_list_vo::SymbolNameList;
use crate::shared_common::taxonomy_name_vo::SymbolName;

pub trait INamingVariantPort: Send + Sync {
    fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value;
    fn build_variants(&self, name: &SymbolName) -> SymbolNameList;
}
