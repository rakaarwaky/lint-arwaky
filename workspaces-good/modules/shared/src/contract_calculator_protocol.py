from abc import ABC, abstractmethod
from typing import Optional

from .taxonomy_expression_vo import ExpressionVO
from .taxonomy_result_vo import ResultVO


class CalculatorProtocol(ABC):
    @abstractmethod
    def evaluate(self, expr: ExpressionVO) -> Optional[ResultVO]:
        pass
