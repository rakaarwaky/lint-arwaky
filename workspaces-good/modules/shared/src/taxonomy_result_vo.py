from typing import Optional

from .taxonomy_operation_vo import OperationVO


class ResultVO:
    def __init__(self, expression: str, value: float):
        self.expression = expression
        self.value = value

    def __repr__(self) -> str:
        return self.expression


def create_result(
    left: float, op: OperationVO, right: float, value: float
) -> ResultVO:
    return ResultVO(
        expression=f"{left} {op.value} {right} = {value}",
        value=value,
    )
