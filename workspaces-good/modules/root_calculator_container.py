from agent_calculator_orchestrator import CalculatorOrchestrator, CalculatorOrchestratorDeps
from addition.src.capability_addition_analyzer import AdditionAnalyzer
from subtraction.src.capability_subtraction_analyzer import SubtractionAnalyzer
from multiplication.src.capability_multiplication_analyzer import MultiplicationAnalyzer
from division.src.capability_division_analyzer import DivisionAnalyzer
from shared.src.contract_calculator_aggregate import CalculatorAggregate


# ─── Block 1: Struct Definition ───────────────────────────

class CalculatorContainer:
    def __init__(self):
        self._orchestrator = CalculatorOrchestrator(CalculatorOrchestratorDeps(
            addition=AdditionAnalyzer(),
            subtraction=SubtractionAnalyzer(),
            multiplication=MultiplicationAnalyzer(),
            division=DivisionAnalyzer(),
        ))

    def orchestrator(self) -> CalculatorAggregate:
        return self._orchestrator
