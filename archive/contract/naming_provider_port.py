"""naming_provider_port — Interface for generating naming variants."""

from abc import ABC, abstractmethod
from ..taxonomy import NameVariants, SymbolName, NamingError


class INamingProviderPort(ABC):
    """Port for generating naming variants of a symbol."""

    @abstractmethod
    def get_variants(self, name: SymbolName) -> NameVariants | NamingError:
        """Generate common naming variants (snake, camel, pascal, screaming, kebab)."""
        ...
