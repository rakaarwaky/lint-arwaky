// AES031 — single-capability-bottleneck
// This single capability class handles ALL dispatch routes.
// Every action (check, scan, fix, report, security, complexity, etc.)
// is routed to this one monolithic class — a severe anti-pattern.
//
// Dispatch should distribute across specialized capability classes.

use crate::taxonomy::removal_types::RemovalType;

/// The ONLY capability class — everything goes through here (AES031 violation)
pub struct GodCapabilityProcessor {
    pub name: String,
}

impl GodCapabilityProcessor {
    pub fn new() -> Self {
        Self {
            name: "god_processor".to_string(),
        }
    }

    // ALL actions are routed to this single class
    pub fn check(&self, path: &str) -> bool {
        !path.is_empty()
    }

    pub fn scan(&self, path: &str) -> Vec<String> {
        vec![path.to_string()]
    }

    pub fn fix(&self, path: &str) -> bool {
        !path.is_empty()
    }

    pub fn report(&self, path: &str, format: &str) -> String {
        format!("Report for {} in {}", path, format)
    }

    pub fn security(&self, path: &str) -> Vec<String> {
        vec![]
    }

    pub fn complexity(&self, path: &str) -> f64 {
        0.0
    }

    pub fn duplicates(&self, path: &str) -> Vec<String> {
        vec![]
    }

    pub fn trends(&self, path: &str) -> Vec<f64> {
        vec![100.0]
    }

    pub fn dependencies(&self, path: &str) -> Vec<String> {
        vec![]
    }
}
