from shared.src.contract_operation_log_protocol import IOperationLogProtocol
from shared.src.taxonomy_operation_vo import OperationVO
from shared.src.taxonomy_result_vo import ResultVO


class DivisionLog(IOperationLogProtocol):
    """Records the results the division feature produced, and only those."""

    def __init__(self):
        self._entries: list[ResultVO] = []

    def record(self, result: ResultVO) -> None:
        if OperationVO.DIVIDE.value not in result.expression:
            return
        self._entries.append(result)

    def entries(self) -> list[ResultVO]:
        """Every division result recorded so far, oldest first."""
        return list(self._entries)
