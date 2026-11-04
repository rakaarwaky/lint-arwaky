"""Calculator feature shared mirror (AES101 `_aggregate` / AES102 `_protocol`).

Mirror of the `modules/calculator` feature into the `modules/shared` layer.
Two FR requirements (FR-CALCULATOR-001, FR-CALCULATOR-002) map to two protocol
classes here.
"""

from abc import ABC, abstractmethod

from ..taxonomy_expression_vo import ExpressionVO
from ..taxonomy_result_vo import ResultVO


class IRouteProtocol(ABC):
    """Route request capability (FR-CALCULATOR-001)."""

    @abstractmethod
    def route(self, expr: ExpressionVO) -> ResultVO | None:
        """Route a request to the feature that owns the operation."""
        ...


class IMergeProtocol(ABC):
    """Merge history capability (FR-CALCULATOR-002)."""

    @abstractmethod
    def merge(self, expr: ExpressionVO) -> ResultVO | None:
        """Merge the operation logs."""
        ...


__all__ = ["IRouteProtocol", "IMergeProtocol"]

_layer_symbols = {"IRouteProtocol": IRouteProtocol, "IMergeProtocol": IMergeProtocol}
