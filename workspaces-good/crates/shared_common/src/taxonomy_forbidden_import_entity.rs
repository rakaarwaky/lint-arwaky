use crate::agent::agent_large_orchestrator::LargeOrchestrator;

pub struct ForbiddenImportEntity {
    pub name: String,
}

impl ForbiddenImportEntity {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
