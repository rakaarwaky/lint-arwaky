//! Port trait for providing naming variants.
//!
//! Defines the outbound interface for generating naming
//! convention variants from a given symbol name.

use crate::naming_rules::taxonomy_name_vo::NameVariants;
use crate::naming_rules::taxonomy_name_vo::SymbolName;

pub trait INamingProviderPort: Send + Sync {
    fn get_variants(&self, name: &SymbolName) -> NameVariants;
}
