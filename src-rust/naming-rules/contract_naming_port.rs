// PURPOSE: INamingProviderPort — port trait for retrieving naming convention variants for a symbol

use crate::naming_rules::taxonomy_name_vo::NameVariants;
use crate::naming_rules::taxonomy_name_vo::SymbolName;

pub trait INamingProviderPort: Send + Sync {
    fn get_variants(&self, name: &SymbolName) -> NameVariants;
}
