"""Calculator aggregate contract (AES101 `_aggregate`)."""

from abc import ABC, abstractmethod

from ..taxonomy_calculator_request import CalculatorRequest
from ..taxonomy_calculator_response import CalculatorResponse


class ICalculatorAggregate(ABC):
    """Aggregate contract — the single entry point over the calculator feature."""

    @abstractmethod
    def execute(self, request: CalculatorRequest) -> CalculatorResponse:
        """Execute a calculator request and return the corresponding response."""
        ...


__all__ = ["ICalculatorAggregate"]

_layer_symbols = {"ICalculatorAggregate": ICalculatorAggregate}
