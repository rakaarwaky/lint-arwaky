# PURPOSE: Test AES022 — bypass comments in Python
# noqa: this line should trigger AES022
from typing import Any  # type: ignore
# pylint: disable=all

def check_something():
    x: Any = 1  # type: ignore[assignment]
    return x
