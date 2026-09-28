"""Multiplication aggregate contract (AES101 `_aggregate`).

The single entry point over the multiplication feature. A consumer passes an
ExpressionVO; the agent behind this aggregate hands it to the multiplication
capability and returns the result.
"""

from abc import ABC, abstractmethod

from .taxonomy_expression_vo import ExpressionVO
from .taxonomy_result_vo import ResultVO


class IMultiplicationAggregate(ABC):
    """Aggregate contract — the single entry point over the multiplication
    feature."""

    @abstractmethod
    def evaluate(self, expr: ExpressionVO) -> ResultVO | None:
        """Evaluate one multiplication expression; `None` when the operation
        does not apply."""
        ...


__all__ = ["IMultiplicationAggregate"]

_layer_symbols = {"IMultiplicationAggregate": IMultiplicationAggregate}
