// PURPOSE: Port: Interface for Naming

use crate::naming_rules::taxonomy_name_vo::NameVariants;
use crate::naming_rules::taxonomy_name_vo::SymbolName;

pub trait INamingProviderPort: Send + Sync {
    fn get_variants(&self, name: &SymbolName) -> NameVariants;
}
