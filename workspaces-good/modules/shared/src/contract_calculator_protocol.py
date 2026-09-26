from abc import ABC, abstractmethod

from .taxonomy_expression_vo import ExpressionVO
from .taxonomy_result_vo import ResultVO


class CalculatorProtocol(ABC):
    @abstractmethod
    def evaluate(self, expr: ExpressionVO) -> ResultVO | None:
        pass
