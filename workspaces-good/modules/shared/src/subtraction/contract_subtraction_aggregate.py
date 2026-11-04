"""Subtraction aggregate contract (AES101 `_aggregate`)."""

from abc import ABC, abstractmethod

from ..taxonomy_calculator_request import CalculatorRequest
from ..taxonomy_calculator_response import CalculatorResponse


class ISubtractionAggregate(ABC):
    """Aggregate contract — the single entry point over the subtraction
    feature."""

    @abstractmethod
    def execute(self, request: CalculatorRequest) -> CalculatorResponse:
        """Execute a subtraction request and return the corresponding
        response."""
        ...


__all__ = ["ISubtractionAggregate"]

_layer_symbols = {"ISubtractionAggregate": ISubtractionAggregate}
