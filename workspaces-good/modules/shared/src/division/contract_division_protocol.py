"""Division feature shared mirror (AES101 `_aggregate` / AES102 `_protocol`).

Mirror of the `modules/division` feature into the `modules/shared` kernel.
Two FR requirements (FR-DIVISION-001, FR-DIVISION-002) map to two protocol
classes here.
"""

from abc import ABC, abstractmethod

from ..taxonomy_expression_vo import ExpressionVO
from ..taxonomy_result_vo import ResultVO


class ICheckProtocol(ABC):
    """Check operands capability (FR-DIVISION-001)."""

    @abstractmethod
    def check(self, expr: ExpressionVO) -> ResultVO | None:
        """Reject a zero divisor."""
        ...


class IRunProtocol(ABC):
    """Run computation capability (FR-DIVISION-002)."""

    @abstractmethod
    def run(self, expr: ExpressionVO) -> ResultVO | None:
        """Return the quotient."""
        ...


__all__ = ["ICheckProtocol", "IRunProtocol"]

_layer_symbols = {"ICheckProtocol": ICheckProtocol, "IRunProtocol": IRunProtocol}
