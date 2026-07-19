from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<name>_vo import <VO>


class I<Name>Aggregate(ABC):
    @abstractmethod
    def execute(self, request: ScanRequest) -> list[LintResult]: ...
