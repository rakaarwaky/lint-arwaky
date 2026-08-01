# AES402: Contract protocol with forbidden primitive types in method signatures
from abc import ABC, abstractmethod

class PrimitiveProtocol(ABC):
    @abstractmethod
    def get_value(self) -> int:
        pass

    @abstractmethod
    def set_name(self, name: str) -> None:
        pass
