// AES030 — capability-method-not-found
// This capability file defines a dispatch catalog referencing methods that
// do NOT exist on the capability class. This causes runtime failures.
//
// The catalog references "nonexistent_method" and "phantom_action"
// but the struct only implements "real_method".

use crate::taxonomy::removal_types::RemovalType;
use std::collections::HashMap;

pub struct DispatchMismatchAnalyzer {
    pub name: String,
}

// Dispatch catalog references methods that don't exist (AES030)
pub fn dispatch_catalog() -> HashMap<String, String> {
    let mut catalog = HashMap::new();
    // These methods are listed in the catalog but NOT implemented below!
    catalog.insert(
        "nonexistent_method".to_string(),
        "Does not exist".to_string(),
    );
    catalog.insert(
        "phantom_action".to_string(),
        "Also does not exist".to_string(),
    );
    catalog.insert("ghost_command".to_string(), "Never implemented".to_string());
    catalog
}

impl DispatchMismatchAnalyzer {
    pub fn new() -> Self {
        Self {
            name: "mismatch".to_string(),
        }
    }

    // Only ONE real method — catalog references 3 methods that don't exist
    pub fn real_method(&self) -> bool {
        true
    }
}
