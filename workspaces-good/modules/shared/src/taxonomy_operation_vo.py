from enum import Enum


class OperationVO(Enum):
    ADD = "+"
    SUBTRACT = "-"
    MULTIPLY = "*"
    DIVIDE = "/"

    def symbol(self) -> str:
        return self.value


def operation_from_symbol(s: str) -> "OperationVO | None":
    mapping = {
        "+": OperationVO.ADD,
        "-": OperationVO.SUBTRACT,
        "*": OperationVO.MULTIPLY,
        "x": OperationVO.MULTIPLY,
        "/": OperationVO.DIVIDE,
    }
    return mapping.get(s)
