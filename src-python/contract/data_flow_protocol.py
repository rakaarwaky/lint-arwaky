"""data_flow_analysis_protocol — Protocol interface for data flow analysis.

Capabilities implement this (DataFlowAnalyzer). Infrastructure consume it via DI.
"""

from abc import ABC, abstractmethod
from ..taxonomy import FilePath, SymbolName, LineNumber, DataFlowList


class IDataFlowProtocol(ABC):
    """Protocol for tracking variable lifecycle in source files."""

    @abstractmethod
    def find_flow(
        self, file_path: FilePath, var_name: SymbolName, start_line: LineNumber
    ) -> DataFlowList:
        """Track the lifecycle of a variable within a file."""
        ...
