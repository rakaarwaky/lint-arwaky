"""CalculatorRequest / CalculatorResponse — aggregate request/response VOs for
the calculator domain. The aggregate's single `execute()` entry point takes a
CalculatorRequest variant and returns the matching CalculatorResponse variant;
each variant carries VO-wrapped values, not raw primitives.
"""

from dataclasses import dataclass, field
from enum import Enum, auto

from .taxonomy_expression_vo import ExpressionVO
from .taxonomy_result_vo import ResultVO


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


@dataclass
class CalculatorResponse:
    """Results of a calculator aggregate request.

    `result` holds the outcome of a DELEGATE verb; `results` holds the full
    calculation history for a HISTORY verb.
    """

    result: ResultVO | None = None
    results: list[ResultVO] = field(default_factory=list)

    @classmethod
    def delegation(cls, result: ResultVO | None) -> "CalculatorResponse":
        return cls(result=result)

    @classmethod
    def history(cls, results: list[ResultVO]) -> "CalculatorResponse":
        return cls(results=list(results))


__all__ = ["CalculatorVerb", "CalculatorRequest", "CalculatorResponse"]

_layer_symbols = {
    "CalculatorVerb": CalculatorVerb,
    "CalculatorRequest": CalculatorRequest,
    "CalculatorResponse": CalculatorResponse,
}
