"""code_transformation_protocol — Protocols for code refactoring and renaming operations."""

from abc import ABC, abstractmethod

from ..taxonomy import DirectoryPath, SymbolName, Count  # mandatory


class ISymbolRenamerProtocol(ABC):
    """Protocol for project-wide symbol renaming across a codebase."""

    @abstractmethod
    def rename_symbol(
        self, root_dir: DirectoryPath, old_name: SymbolName, new_name: SymbolName
    ) -> Count:
        """Rename a symbol throughout the project, respecting comments and strings."""
        ...
