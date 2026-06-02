// This capability file violates AES014 (bypass-comment-violation)
// because it contains direct unwrap() calls which are prohibited bypass markers.
use crate::taxonomy::removal_types::RemovalType;

pub struct BypassCheatAnalyzer {
    pub value: String,
}

impl BypassCheatAnalyzer {
    pub fn parse_value(&self) -> String {
        let opt: Option<String> = Some("data".to_string());
        // Unsafe raw unwrap violates AES014!
        opt.unwrap()
    }
}
