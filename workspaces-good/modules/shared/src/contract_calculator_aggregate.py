from abc import ABC, abstractmethod

from .taxonomy_expression_vo import ExpressionVO
from .taxonomy_result_vo import ResultVO


class CalculatorAggregate(ABC):
    @abstractmethod
    def delegate(self, expr: ExpressionVO) -> ResultVO | None:
        pass

    @abstractmethod
    def history(self) -> list[ResultVO]:
        pass
