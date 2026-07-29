# AES014 — forbidden bypass comments
# noqa: this entire file is a violation
from typing import Any  # type: ignore

x: Any = 1  # type: ignore[assignment]
y = "test"  # noqa: F841
