//! Port trait for providing naming variants.
//!
//! Defines the outbound interface for generating naming
//! convention variants from a given symbol name.

use crate::taxonomy::{NameVariants, SymbolName};



pub trait INamingProviderPort: Send + Sync {
    fn get_variants(&self, name: &SymbolName) -> NameVariants;
}
