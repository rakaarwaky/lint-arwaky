# AES402: contract uses primitive types instead of taxonomy VOs
from abc import ABC, abstractmethod

class ConfigProtocol(ABC):
    @abstractmethod
    def load(self, id: int, name: str) -> bool:
        pass
    
    @abstractmethod
    def save(self, data: bytes) -> None:
        pass
