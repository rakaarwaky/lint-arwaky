// AES203: capabilities with unused import (entity is imported but never used)
use crate::taxonomy::taxonomy_massive_domain_entity::MassiveDomainEntity;

pub struct UnusedImportChecker;

impl UnusedImportChecker {
    pub fn check(&self) -> bool {
        // MassiveDomainEntity is imported but never used
        true
    }
}
