from typing import Optional

from .taxonomy_expression_vo import ExpressionVO, create_expression
from .taxonomy_operation_vo import operation_from_symbol


def parse_operand(input_str: str) -> Optional[float]:
    try:
        return float(input_str.strip())
    except ValueError:
        return None


def parse_expression(input_str: str) -> Optional[ExpressionVO]:
    parts = input_str.strip().split()
    if len(parts) != 3:
        return None
    left = parse_operand(parts[0])
    op = operation_from_symbol(parts[1])
    right = parse_operand(parts[2])
    if left is None or op is None or right is None:
        return None
    return create_expression(left, op, right)
