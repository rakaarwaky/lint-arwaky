"""Subtraction aggregate contract (AES101 `_aggregate`).

The single entry point over the subtraction feature. A consumer passes an
ExpressionVO; the agent behind this aggregate hands it to the subtraction
capability and returns the result.
"""

from abc import ABC, abstractmethod

from .taxonomy_expression_vo import ExpressionVO
from .taxonomy_result_vo import ResultVO


class ISubtractionAggregate(ABC):
    """Aggregate contract — the single entry point over the subtraction
    feature."""

    @abstractmethod
    def evaluate(self, expr: ExpressionVO) -> ResultVO | None:
        """Evaluate one subtraction expression; `None` when the operation does
        not apply."""
        ...


__all__ = ["ISubtractionAggregate"]

_layer_symbols = {"ISubtractionAggregate": ISubtractionAggregate}
