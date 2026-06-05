use crate::taxonomy::{NameVariants, SymbolName};
use super::*;


pub trait INamingProviderPort: Send + Sync {
    fn get_variants(&self, name: &SymbolName) -> NameVariants;
}
