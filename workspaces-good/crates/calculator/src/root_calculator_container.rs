use crate::agent_calculator_orchestrator::{CalculatorOrchestrator, CalculatorOrchestratorDeps};
use crate::capabilities_addition_analyzer::AdditionAnalyzer;
use crate::capabilities_division_analyzer::DivisionAnalyzer;
use crate::capabilities_multiplication_analyzer::MultiplicationAnalyzer;
use crate::capabilities_subtraction_analyzer::SubtractionAnalyzer;
use calculator_shared::contract_calculator_aggregate::CalculatorAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CalculatorContainer {
    orchestrator: CalculatorOrchestrator,
}

// ─── Block 2: Wiring & Factory ────────────────────────────

impl CalculatorContainer {
    pub fn new() -> Self {
        let orchestrator = CalculatorOrchestrator::new(CalculatorOrchestratorDeps {
            addition: Box::new(AdditionAnalyzer),
            subtraction: Box::new(SubtractionAnalyzer),
            multiplication: Box::new(MultiplicationAnalyzer),
            division: Box::new(DivisionAnalyzer),
        });
        Self { orchestrator }
    }

    pub fn orchestrator(&mut self) -> &mut dyn CalculatorAggregate {
        &mut self.orchestrator
    }
}

impl Default for CalculatorContainer {
    fn default() -> Self {
        Self::new()
    }
}
