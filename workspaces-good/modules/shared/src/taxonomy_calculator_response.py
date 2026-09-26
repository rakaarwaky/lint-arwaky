"""CalculatorResponse — response VOs for the calculator aggregate."""

from dataclasses import dataclass, field

from .taxonomy_result_vo import ResultVO


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


__all__ = ["CalculatorResponse"]
