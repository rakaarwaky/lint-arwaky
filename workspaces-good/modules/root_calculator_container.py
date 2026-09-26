from addition.src.capability_addition_analyzer import AdditionAnalyzer
from agent_calculator_orchestrator import (
    CalculatorOrchestrator,
    CalculatorOrchestratorDeps,
)
from division.src.capability_division_analyzer import DivisionAnalyzer
from multiplication.src.capability_multiplication_analyzer import (
    MultiplicationAnalyzer,
)
from shared.src.contract_calculator_aggregate import ICalculatorAggregate
from subtraction.src.capability_subtraction_analyzer import SubtractionAnalyzer

# ─── Block 1: Struct Definition ───────────────────────────

class CalculatorContainer:
    def __init__(self):
        self._orchestrator = CalculatorOrchestrator(CalculatorOrchestratorDeps(
            addition=AdditionAnalyzer(),
            subtraction=SubtractionAnalyzer(),
            multiplication=MultiplicationAnalyzer(),
            division=DivisionAnalyzer(),
        ))

    def orchestrator(self) -> ICalculatorAggregate:
        return self._orchestrator
