// PURPOSE: Test AES0305 — agent non-stateless, any type, infra imports, single goal
use std::path::Path;

pub struct AgentStatefulViolations {
    state: Option<String>,
}

impl AgentStatefulViolations {
    pub fn run(&mut self, data: Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {  // AES0305: any type
        self.state = Some("modified".to_string());  // AES0305: state assignment
        data
    }

    fn process(&self, path: String) -> String {
        fs::read_to_string(path).unwrap_or_default()  // AES0305: infra import usage
    }
}

pub struct AgentSingleGoal;
impl AgentSingleGoal {
    pub fn execute(&self) {}  // AES0305: single execution goal
}