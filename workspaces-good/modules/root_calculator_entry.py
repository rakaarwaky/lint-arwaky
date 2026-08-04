import sys
import os
import argparse

sys.path.insert(0, os.path.dirname(__file__))

from root_calculator_container import CalculatorContainer
from cli.src.surface_calculator_command import run


def main():
    parser = argparse.ArgumentParser(description="Calculator REPL")
    subparsers = parser.add_subparsers(dest="command")

    run_parser = subparsers.add_parser("run", help="Run the calculator REPL")
    run_parser.add_argument("--path", type=str, default=".", help="Working path")

    args = parser.parse_args()

    if args.command is None or args.command == "run":
        container = CalculatorContainer()
        run(container.orchestrator())
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()
