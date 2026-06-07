//! Port trait for naming variant generation (infrastructure side).
//!
//! Provides the outbound interface for computing naming variants
//! used by infrastructure implementations.

use crate::taxonomy::SymbolName;
use crate::taxonomy::SymbolNameList;


pub trait INamingVariantPort: Send + Sync {
    fn get_variant_dict(&self, name: &SymbolName) -> serde_json::Value;
    fn build_variants(&self, name: &SymbolName) -> SymbolNameList;
}
