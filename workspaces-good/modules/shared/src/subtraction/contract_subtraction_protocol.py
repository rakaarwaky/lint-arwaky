"""Subtraction feature shared mirror (AES101 `_aggregate` / AES102 `_protocol`).

Mirror of the `modules/subtraction` feature into the `modules/shared` kernel.
Two FR requirements (FR-SUBTRACTION-001, FR-SUBTRACTION-002) map to two protocol
classes here.
"""

from abc import ABC, abstractmethod

from ..taxonomy_expression_vo import ExpressionVO
from ..taxonomy_result_vo import ResultVO


class ICheckProtocol(ABC):
    """Check operands capability (FR-SUBTRACTION-001)."""

    @abstractmethod
    def check(self, expr: ExpressionVO) -> ResultVO | None:
        """Validate the operands."""
        ...


class IRunProtocol(ABC):
    """Run computation capability (FR-SUBTRACTION-002)."""

    @abstractmethod
    def run(self, expr: ExpressionVO) -> ResultVO | None:
        """Return the difference."""
        ...


__all__ = ["ICheckProtocol", "IRunProtocol"]

_layer_symbols = {"ICheckProtocol": ICheckProtocol, "IRunProtocol": IRunProtocol}
