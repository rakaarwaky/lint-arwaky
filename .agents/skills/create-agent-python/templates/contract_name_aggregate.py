from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<name>_vo import <RequestVO>, <ResultVO>

class I<Name>Aggregate(ABC):
    @abstractmethod
    def execute(self, request: <RequestVO>) -> list[<ResultVO>]:
        ...
