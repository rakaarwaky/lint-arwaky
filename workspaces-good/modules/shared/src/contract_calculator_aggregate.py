from abc import ABC, abstractmethod
from typing import List, Optional

from .taxonomy_expression_vo import ExpressionVO
from .taxonomy_result_vo import ResultVO


class CalculatorAggregate(ABC):
    @abstractmethod
    def delegate(self, expr: ExpressionVO) -> Optional[ResultVO]:
        pass

    @abstractmethod
    def history(self) -> List[ResultVO]:
        pass
