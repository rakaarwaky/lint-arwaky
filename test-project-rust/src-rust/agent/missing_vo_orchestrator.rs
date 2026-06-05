// AES032 — missing-vo-construction
// This agent orchestrator calls capability methods WITHOUT passing
// the required Value Object parameters. Capabilities expect typed VOs
// (like RemovalRequest, RemovalResult), but this file passes raw primitives.

use crate::capabilities::removal_usecase_handler::RemovalUsecase;
use crate::taxonomy::removal_types::{RemovalRequest, RemovalResult, RemovalType};

pub struct MissingVoOrchestrator {
    pub usecase: RemovalUsecase,
}

impl MissingVoOrchestrator {
    pub fn new() -> Self {
        Self {
            usecase: RemovalUsecase,
        }
    }

    // AES032: Calls execute() but does NOT construct or pass a RemovalRequest VO.
    // Instead passes raw primitives (String, bool) bypassing domain validation.
    pub fn run_without_vo(&self, path: String, force: bool) -> bool {
        // WRONG: should construct RemovalRequest VO and pass it
        // let request = RemovalRequest { image_path: path, removal_type: RemovalType::Background };
        // self.usecase.execute(request);

        // Bypasses VO construction entirely — passes raw primitives
        let _path_str = path;
        let _force_flag = force;
        true
    }

    // AES032: Another call site missing VO parameter
    pub fn process_batch(&self, paths: Vec<String>) -> Vec<bool> {
        paths
            .iter()
            .map(|p| {
                // No RemovalRequest constructed — raw string passed
                let _raw = p.clone();
                true
            })
            .collect()
    }
}
