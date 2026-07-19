from abc import ABC, abstractmethod
from shared.<domain>.taxonomy_<name>_vo import <VO>


class I<Name>Protocol(ABC):
    @abstractmethod
    def method_name(self, param: <VO>) -> None: ...
