"""calculator-domain aggregate contract (AES101 `_aggregate`).

The single entry point over the calculator feature. Consumers pass a
CalculatorRequest; the agent behind the aggregate dispatches to the rich
protocol classes in `contract_calculator_protocol.py`.
"""

from abc import ABC, abstractmethod

from .taxonomy_calculator_request import CalculatorRequest
from .taxonomy_calculator_response import CalculatorResponse


class ICalculatorAggregate(ABC):
    """Aggregate trait — the single entry point over the calculator feature."""

    @abstractmethod
    def execute(self, request: CalculatorRequest) -> CalculatorResponse:
        """Execute a calculator request; return the corresponding response."""
        ...


__all__ = ["ICalculatorAggregate"]

_layer_symbols = {"ICalculatorAggregate": ICalculatorAggregate}
