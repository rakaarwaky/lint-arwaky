from shared.src.contract_operation_log_protocol import IOperationLogProtocol
from shared.src.taxonomy_operation_vo import OperationVO
from shared.src.taxonomy_result_vo import ResultVO

# ─── Block 1: Struct Definition ─────────────────────────────

class MultiplicationLog(IOperationLogProtocol):
    """Records the results the multiplication feature produced, and only
    those."""

    def __init__(self):
        self._entries: list[ResultVO] = []

    # ─── Block 2: Protocol Trait Implementation ────────────────

    def record(self, result: ResultVO) -> None:
        if not self._is_multiplication_result(result):
            return
        self._entries.append(result)

    def entries(self) -> list[ResultVO]:
        """Every multiplication result recorded so far, oldest first."""
        return list(self._entries)

    # ─── Block 3: Constructors, Std Traits, Helpers ─────────────

    def _is_multiplication_result(self, result: ResultVO) -> bool:
        """True when `result` was produced by this feature."""
        return OperationVO.MULTIPLY.value in result.expression
