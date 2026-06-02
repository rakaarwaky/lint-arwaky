"""naming_variant_protocol — Protocol interface for naming variant analysis.

Capabilities implement this (NamingVariantAnalyzer).
"""

from abc import ABC, abstractmethod
from ..taxonomy import SymbolName, SymbolNameList, ResponseData, NamingError


class INamingVariantPort(ABC):
    """Port for generating identifier naming variants in infrastructure."""

    @abstractmethod
    def get_variant_dict(self, name: SymbolName) -> ResponseData | NamingError:
        """Return naming variants (camelCase, snake_case, etc.) for a name."""
        ...

    @abstractmethod
    def build_variants(self, name: SymbolName) -> SymbolNameList | NamingError:
        """Produce common naming variants for a given name."""
        ...
