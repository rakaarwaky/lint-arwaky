use std::sync::Arc;

use shared::<domain>::taxonomy_<name>_vo::<VO>;
use shared::<domain>::contract_<name>_port::I<Name>Port;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct Infrastructure<Name> {
    // DI fields use Arc<dyn Trait>
    // Value fields use shared VOs
}

// ─── Block 2: Public Contract (domain port ONLY) ──────────
impl I<Name>Port for Infrastructure<Name> {
    // port methods only
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for Infrastructure<Name> {
    fn default() -> Self {
        Self
    }
}

impl Infrastructure<Name> {
    pub fn new(/* DI params */) -> Self {
        Self { /* ... */ }
    }
}
