import sys
import os
import argparse

sys.path.insert(0, os.path.dirname(__file__))

from shared.src.taxonomy_operation_vo import OperationVO
from addition.src.capability_addition_analyzer import AdditionAnalyzer
from subtraction.src.capability_subtraction_analyzer import SubtractionAnalyzer
from multiplication.src.capability_multiplication_analyzer import MultiplicationAnalyzer
from division.src.capability_division_analyzer import DivisionAnalyzer
from cli.src.surface_calculator_command import run


class CalculatorAggregate:
    def __init__(self):
        self._analyzers = {}
        self._history = []

    def register(self, op, analyzer):
        self._analyzers[op] = analyzer

    def delegate(self, expr):
        analyzer = self._analyzers.get(expr.op)
        if analyzer:
            result = analyzer.evaluate(expr)
            if result:
                self._history.append(result)
            return result
        return None

    def history(self):
        return list(self._history)


def main():
    parser = argparse.ArgumentParser(description="Calculator REPL")
    subparsers = parser.add_subparsers(dest="command")

    run_parser = subparsers.add_parser("run", help="Run the calculator REPL")
    run_parser.add_argument("--path", type=str, default=".", help="Working path")

    args = parser.parse_args()

    if args.command is None or args.command == "run":
        calc = CalculatorAggregate()
        calc.register(OperationVO.ADD, AdditionAnalyzer())
        calc.register(OperationVO.SUBTRACT, SubtractionAnalyzer())
        calc.register(OperationVO.MULTIPLY, MultiplicationAnalyzer())
        calc.register(OperationVO.DIVIDE, DivisionAnalyzer())
        run(calc)
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()
