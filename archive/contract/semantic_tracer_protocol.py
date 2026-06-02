"""semantic_tracer_protocol — Protocol interface for semantic analysis capabilities."""

from abc import ABC, abstractmethod


from ..taxonomy import (
    FilePath,
    SymbolName,
    SymbolNameList,
    DirectoryPath,
    ResponseData,
    Count,
    LineNumber,
    ScopeRef,
    CallChainList,
    DataFlowList,
    ResponseDataList,
)


class ISemanticTracerProtocol(ABC):
    """Protocol interface for semantic code analysis."""

    @abstractmethod
    def get_enclosing_scope(
        self, file_path: FilePath, line: LineNumber
    ) -> ScopeRef | None:
        """Return the name of the function or class enclosing the given line."""
        ...

    @abstractmethod
    def trace_call_chain(
        self, root_dir: DirectoryPath, target_name: SymbolName
    ) -> CallChainList:
        """Find all call sites for the target name within the project."""
        ...

    @abstractmethod
    def find_flow(
        self, file_path: FilePath, var_name: SymbolName, start_line: LineNumber
    ) -> DataFlowList:
        """Track the lifecycle of a variable within a file."""
        ...

    @abstractmethod
    def get_variant_dict(self, name: SymbolName) -> ResponseData:
        """Return naming variants (camelCase, snake_case, etc.) for a name."""
        ...

    @abstractmethod
    def project_wide_rename(
        self, root_dir: DirectoryPath, old_name: SymbolName, new_name: SymbolName
    ) -> Count:
        """Rename a symbol across all files in the project."""
        ...

    @abstractmethod
    def get_symbol_locations(
        self, file_path: FilePath, symbol: SymbolName
    ) -> ResponseDataList:
        """Return the locations of a symbol within a file."""
        ...

    @abstractmethod
    def build_variants(self, name: SymbolName) -> SymbolNameList:
        """Produce common naming variants for a given name."""
        ...
