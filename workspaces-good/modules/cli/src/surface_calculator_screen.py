"""Surface fixture exercising unified surface self-import rule."""

from cli.src.surface_calculator_command import run as command_run


def show(calc) -> None:
    command_run(calc)
