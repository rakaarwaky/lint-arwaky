// PURPOSE: INamingProviderPort — port for symbol naming and variant generation
use crate::taxonomy_name_vo::{NameVariants, SymbolName};

pub trait INamingProviderPort: Send + Sync {
    fn get_variants(&self, name: &SymbolName) -> NameVariants;
}
