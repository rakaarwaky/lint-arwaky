"""javascript_flow_port — Port for JS variable flow tracking."""

from abc import ABC, abstractmethod
from ..taxonomy import FilePath, SymbolName, LineNumber, DataFlowList, SemanticError


class IJSFlowTracerPort(ABC):
    """Port for tracking variable flow in JS/TS."""

    @abstractmethod
    def find_flow(
        self, file_path: FilePath, var_name: SymbolName, start_line: LineNumber | None
    ) -> DataFlowList | SemanticError:
        """Track lifecycle of a variable."""
        ...

    @abstractmethod
    def trace_flow(self, path: FilePath) -> DataFlowList | SemanticError:
        """Trace data flow in a Javascript file."""
        ...
