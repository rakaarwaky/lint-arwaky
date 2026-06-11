use crate::agent::base_orchestrator::BaseOrchestrator;
pub struct PassiveBadView;
impl PassiveBadView {
    pub fn render(&self) -> &'static str { "view" }
}
