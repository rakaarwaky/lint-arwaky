# AES502: orphan - not imported by any capabilities file
from abc import ABC, abstractmethod

class OrphanConfigProtocol(ABC):
    @abstractmethod
    def load(self) -> bool:
        pass
