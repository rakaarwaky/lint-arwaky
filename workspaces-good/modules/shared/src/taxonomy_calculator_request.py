"""CalculatorRequest — request VOs for the calculator aggregate."""

from dataclasses import dataclass
from enum import Enum, auto

from .taxonomy_expression_vo import ExpressionVO


class CalculatorVerb(Enum):
    """Consumer verbs carried by the calculator aggregate's single entry point."""

    DELEGATE = auto()
    HISTORY = auto()


@dataclass
class CalculatorRequest:
    """A consumer verb plus its operand, if the verb needs one."""

    verb: CalculatorVerb
    expr: ExpressionVO | None = None

    @classmethod
    def delegate(cls, expr: ExpressionVO) -> "CalculatorRequest":
        return cls(verb=CalculatorVerb.DELEGATE, expr=expr)

    @classmethod
    def history(cls) -> "CalculatorRequest":
        return cls(verb=CalculatorVerb.HISTORY)


__all__ = ["CalculatorVerb", "CalculatorRequest"]
