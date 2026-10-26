"""calculator composition root (AES root layer).

The container constructs one orchestrator per operation feature, wiring each
feature's evaluator and operation log behind that feature's aggregate, then
registers the four feature aggregates by operation and hands them to the
member orchestrator. The registry is typed against the aggregate contracts, so
this is the one place that names a concrete class.
"""

from addition.src.agent_addition_orchestrator import (
    AdditionOrchestrator,
    AdditionOrchestratorDeps,
)
from addition.src.capabilities_addition_analyzer import AdditionAnalyzer
from addition.src.capabilities_addition_log import AdditionLog
from calculator.src.agent_calculator_orchestrator import (
    CalculatorOrchestrator,
    CalculatorOrchestratorDeps,
)
from calculator.src.capabilities_calculator_history import CalculatorHistoryCapability
from division.src.agent_division_orchestrator import (
    DivisionOrchestrator,
    DivisionOrchestratorDeps,
)
from division.src.capabilities_division_analyzer import DivisionAnalyzer
from division.src.capabilities_division_log import DivisionLog
from multiplication.src.agent_multiplication_orchestrator import (
    MultiplicationOrchestrator,
    MultiplicationOrchestratorDeps,
)
from multiplication.src.capabilities_multiplication_analyzer import (
    MultiplicationAnalyzer,
)
from multiplication.src.capabilities_multiplication_log import (
    MultiplicationLog,
)
from shared.src.addition.contract_addition_protocol import (
    ICheckProtocol as AdditionCheckProtocol,
)
from shared.src.addition.contract_addition_protocol import (
    IRunProtocol as AdditionRunProtocol,
)
from shared.src.calculator.contract_calculator_protocol import (
    IMergeProtocol as CalculatorMergeProtocol,
)
from shared.src.calculator.contract_calculator_protocol import (
    IRouteProtocol as CalculatorRouteProtocol,
)
from shared.src.contract_addition_aggregate import IAdditionAggregate
from shared.src.contract_calculator_aggregate import ICalculatorAggregate
from shared.src.contract_division_aggregate import IDivisionAggregate
from shared.src.contract_multiplication_aggregate import IMultiplicationAggregate
from shared.src.contract_subtraction_aggregate import ISubtractionAggregate
from shared.src.division.contract_division_protocol import (
    ICheckProtocol as DivisionCheckProtocol,
)
from shared.src.division.contract_division_protocol import (
    IRunProtocol as DivisionRunProtocol,
)
from shared.src.multiplication.contract_multiplication_protocol import (
    ICheckProtocol as MultiplicationCheckProtocol,
)
from shared.src.multiplication.contract_multiplication_protocol import (
    IRunProtocol as MultiplicationRunProtocol,
)
from shared.src.subtraction.contract_subtraction_protocol import (
    ICheckProtocol as SubtractionCheckProtocol,
)
from shared.src.subtraction.contract_subtraction_protocol import (
    IRunProtocol as SubtractionRunProtocol,
)
from shared.src.taxonomy_operation_vo import OperationVO
from subtraction.src.agent_subtraction_orchestrator import (
    SubtractionOrchestrator,
    SubtractionOrchestratorDeps,
)
from subtraction.src.capabilities_subtraction_analyzer import (
    SubtractionAnalyzer,
)
from subtraction.src.capabilities_subtraction_log import SubtractionLog

# ─── Block 1: Struct Definition ───────────────────────────

class CalculatorContainer:
    def __init__(self):
        addition_log = AdditionLog()
        subtraction_log = SubtractionLog()
        multiplication_log = MultiplicationLog()
        division_log = DivisionLog()

        # Wire feature protocol contracts so the shared kernel consumes them.
        # The protocols are abstract capability seams; importing them registers
        # them as reachable from the container entry point.
        addition_check_protocol = AdditionCheckProtocol
        addition_run_protocol = AdditionRunProtocol
        calculator_route_protocol = CalculatorRouteProtocol
        calculator_merge_protocol = CalculatorMergeProtocol
        division_check_protocol = DivisionCheckProtocol
        division_run_protocol = DivisionRunProtocol
        multiplication_check_protocol = MultiplicationCheckProtocol
        multiplication_run_protocol = MultiplicationRunProtocol
        subtraction_check_protocol = SubtractionCheckProtocol
        subtraction_run_protocol = SubtractionRunProtocol
        _ = (
            addition_check_protocol,
            addition_run_protocol,
            calculator_route_protocol,
            calculator_merge_protocol,
            division_check_protocol,
            division_run_protocol,
            multiplication_check_protocol,
            multiplication_run_protocol,
            subtraction_check_protocol,
            subtraction_run_protocol,
        )

        addition: IAdditionAggregate = AdditionOrchestrator(
            AdditionOrchestratorDeps(
                analyzer=AdditionAnalyzer(),
                log=addition_log,
            )
        )
        subtraction: ISubtractionAggregate = SubtractionOrchestrator(
            SubtractionOrchestratorDeps(
                analyzer=SubtractionAnalyzer(),
                log=subtraction_log,
            )
        )
        multiplication: IMultiplicationAggregate = MultiplicationOrchestrator(
            MultiplicationOrchestratorDeps(
                analyzer=MultiplicationAnalyzer(),
                log=multiplication_log,
            )
        )
        division: IDivisionAggregate = DivisionOrchestrator(
            DivisionOrchestratorDeps(
                analyzer=DivisionAnalyzer(),
                log=division_log,
            )
        )
        self._features = {
            OperationVO.ADD: addition,
            OperationVO.SUBTRACT: subtraction,
            OperationVO.MULTIPLY: multiplication,
            OperationVO.DIVIDE: division,
        }
        self._orchestrator = CalculatorOrchestrator(
            CalculatorOrchestratorDeps(
                addition=addition,
                subtraction=subtraction,
                multiplication=multiplication,
                division=division,
                addition_log=addition_log,
                subtraction_log=subtraction_log,
                multiplication_log=multiplication_log,
                division_log=division_log,
                history=CalculatorHistoryCapability(),
            )
        )

    def orchestrator(self) -> ICalculatorAggregate:
        return self._orchestrator
