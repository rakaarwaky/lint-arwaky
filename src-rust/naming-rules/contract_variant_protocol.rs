//! Protocol for naming variant generation (capabilities side).
//!
//! Defines the inbound interface for computing naming variants
//! used by capability layer implementations.

use crate::naming_rules::taxonomy_name_vo::SymbolName;
use crate::naming_rules::taxonomy_naming_list_vo::SymbolNameList;

pub trait INamingVariantProtocol: Send + Sync {
    fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value;
    fn build_variants(&self, name: &SymbolName) -> SymbolNameList;
}
