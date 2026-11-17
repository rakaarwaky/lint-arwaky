import sys
from pathlib import Path

# Surface fixture: unified surface self-import — `surface` is allowed to import
# other `surface` files (row 10 of the 11-scope AES201 Import Matrix).
sys.path.insert(0, str(Path(__file__).parent.parent))
from cli.src.surface_calculator_command import run as command_run  # noqa: E402


def show(calc) -> None:
    command_run(calc)
