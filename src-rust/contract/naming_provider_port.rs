use crate::taxonomy::{NameVariants, SymbolName};



pub trait INamingProviderPort: Send + Sync {
    fn get_variants(&self, name: &SymbolName) -> NameVariants;
}
