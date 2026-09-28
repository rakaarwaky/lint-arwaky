"""operation-log capability contract (AES102 `_protocol`).

One file for the operation-log seam. Each operation feature wires a recorder
next to its evaluator, so a feature folder holds two capability seams and its
orchestrator coordinates both: the arithmetic result and the durable record
of that result.
"""

from abc import ABC, abstractmethod

from .taxonomy_result_vo import ResultVO


class IOperationLogProtocol(ABC):
    """Operation log capability: record the outcome of one evaluation."""

    @abstractmethod
    def record(self, result: ResultVO) -> None:
        """Record one evaluated result into the operation log."""
        ...


__all__ = ["IOperationLogProtocol"]

_layer_symbols = {"IOperationLogProtocol": IOperationLogProtocol}
