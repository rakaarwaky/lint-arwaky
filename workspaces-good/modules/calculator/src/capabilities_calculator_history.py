from shared.src.contract_operation_log_protocol import IOperationLogProtocol
from shared.src.taxonomy_operation_log_vo import OperationLogVO

# ─── Block 1: Struct Definition ─────────────────────────────

class CalculatorHistoryCapability(IOperationLogProtocol):
    """Merge the four operation logs into the history the caller reads."""

    # ─── Block 2: Protocol Trait Implementation ────────────────

    def record(self, result: OperationLogVO) -> None:
        """Record one evaluated result into the merged history."""
        self.entries.append(result)

    def merge(self, logs) -> list:
        """Return every operation log entry, newest first."""
        entries = []
        for log in logs:
            entries.extend(log.entries)
        return sorted(entries, key=lambda entry: entry.when, reverse=True)

    # ─── Block 3: Constructors, Std Traits, Helpers ─────────────

    def __init__(self) -> None:
        self.entries = []

    def __repr__(self) -> str:
        return "CalculatorHistoryCapability()"
