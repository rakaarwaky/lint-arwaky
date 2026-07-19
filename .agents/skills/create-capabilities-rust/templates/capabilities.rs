use std::sync::Arc;

use shared::<domain>::taxonomy_<name>_vo::<VO>;
use shared::<domain>::contract_<name>_protocol::I<Name>Protocol;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct Capabilities<Name> {
    // DI fields use Arc<dyn Trait>
    // Value fields use shared VOs
}

// ─── Block 2: Public Contract (domain protocol ONLY) ──────
impl I<Name>Protocol for Capabilities<Name> {
    // public contract methods only
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for Capabilities<Name> {
    fn default() -> Self {
        Self
    }
}

impl Capabilities<Name> {
    pub fn new(/* DI params */) -> Self {
        Self {
            // ...
        }
    }

    // private helpers here
}
