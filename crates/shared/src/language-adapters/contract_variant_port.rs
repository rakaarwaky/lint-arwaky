// PURPOSE: INamingVariantPort — port for language-specific naming variant generation
use crate::taxonomy_name_vo::SymbolName;

pub trait INamingVariantPort: Send + Sync {
    fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value;
}
