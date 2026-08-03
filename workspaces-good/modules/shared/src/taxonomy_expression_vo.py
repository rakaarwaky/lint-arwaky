from .taxonomy_operation_vo import OperationVO


class ExpressionVO:
    def __init__(self, left: float, op: OperationVO, right: float):
        self.left = left
        self.op = op
        self.right = right

    def __repr__(self) -> str:
        return f"{self.left} {self.op.value} {self.right}"


def create_expression(left: float, op: OperationVO, right: float) -> ExpressionVO:
    return ExpressionVO(left, op, right)
