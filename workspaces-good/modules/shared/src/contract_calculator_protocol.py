"""calculator-domain capability contracts (AES102 `_protocol`).

One file for the calculator feature. Each class below is one capability
seam: a class carries every method that capability implements, with one
concrete return type each, so a capability implements its class outright
and never carries stubs.
"""

from abc import ABC, abstractmethod

from .taxonomy_expression_vo import ExpressionVO
from .taxonomy_result_vo import ResultVO


class ICalculatorProtocol(ABC):
    """Arithmetic evaluation capability: evaluates one expression using this
    capability's own operation."""

    @abstractmethod
    def evaluate(self, expr: ExpressionVO) -> ResultVO | None:
        """Evaluate a single arithmetic expression. Return the result, or
        `None` when the operation does not apply (e.g. division by zero)."""
        ...


__all__ = ["ICalculatorProtocol"]

_layer_symbols = {"ICalculatorProtocol": ICalculatorProtocol}
