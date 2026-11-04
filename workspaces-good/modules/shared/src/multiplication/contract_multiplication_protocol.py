"""Multiplication feature shared mirror (AES101 `_aggregate` / AES102 `_protocol`).

Mirror of the `modules/multiplication` feature into the `modules/shared` layer.
Two FR requirements (FR-MULTIPLICATION-001, FR-MULTIPLICATION-002) map to two protocol
classes here.
"""

from abc import ABC, abstractmethod

from ..taxonomy_expression_vo import ExpressionVO
from ..taxonomy_result_vo import ResultVO


class ICheckProtocol(ABC):
    """Check operands capability (FR-MULTIPLICATION-001)."""

    @abstractmethod
    def check(self, expr: ExpressionVO) -> ResultVO | None:
        """Validate the operands."""
        ...


class IRunProtocol(ABC):
    """Run computation capability (FR-MULTIPLICATION-002)."""

    @abstractmethod
    def run(self, expr: ExpressionVO) -> ResultVO | None:
        """Return the product."""
        ...


__all__ = ["ICheckProtocol", "IRunProtocol"]

_layer_symbols = {"ICheckProtocol": ICheckProtocol, "IRunProtocol": IRunProtocol}
