// AES010 — forbidden-suffix violation
// This file is in the infrastructure layer but uses the `_vo` suffix,
// which is forbidden for infrastructure files (VOs belong to taxonomy).
// Infrastructure allowed suffixes: _adapter, _provider, _scanner, _wrapper, _client, _lifespan, _schemas, _validator

use crate::taxonomy::removal_types::RemovalType;

pub struct ForbiddenSuffixVo {
    pub value: String,
    pub removal_type: RemovalType,
}

impl ForbiddenSuffixVo {
    pub fn new(value: String, removal_type: RemovalType) -> Self {
        Self {
            value,
            removal_type,
        }
    }
}
