use crate::taxonomy::{SymbolName, SymbolNameList};

pub trait INamingVariantProtocol: Send + Sync {
    fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value;
    fn build_variants(&self, name: &SymbolName) -> SymbolNameList;
}
