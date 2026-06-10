// PURPOSE: INamingProviderPort — port trait for retrieving naming convention variants for a symbol

use crate::shared_common::taxonomy_name_vo::NameVariants;
use crate::shared_common::taxonomy_name_vo::SymbolName;

pub trait INamingProviderPort: Send + Sync {
    fn get_variants(&self, name: &SymbolName) -> NameVariants;
}
