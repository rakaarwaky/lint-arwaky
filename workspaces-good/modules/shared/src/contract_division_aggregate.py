"""Division aggregate contract (AES101 `_aggregate`).

The single entry point over the division feature. A consumer passes an
ExpressionVO; the agent behind this aggregate hands it to the division
capability and returns the result, or `None` when the expression cannot be
divided.
"""

from abc import ABC, abstractmethod

from .taxonomy_expression_vo import ExpressionVO
from .taxonomy_result_vo import ResultVO


class IDivisionAggregate(ABC):
    """Aggregate contract — the single entry point over the division
    feature."""

    @abstractmethod
    def evaluate(self, expr: ExpressionVO) -> ResultVO | None:
        """Evaluate one division expression; `None` when the operation does
        not apply."""
        ...


__all__ = ["IDivisionAggregate"]

_layer_symbols = {"IDivisionAggregate": IDivisionAggregate}
