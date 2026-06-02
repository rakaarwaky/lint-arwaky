"""naming_provider_error — Naming convention domain error types."""

from pydantic import BaseModel, ConfigDict

from .error_code_vo import ErrorCode
from .error_value_vo import (
    ErrorMessage,
    Cause,
)


class NamingError(BaseModel):
    """General failure during naming analysis."""

    model_config = ConfigDict(frozen=True)

    message: ErrorMessage
    error_code: ErrorCode | None = None
    cause: Cause | None = None

    def __str__(self):
        code = f" [{self.error_code}]" if self.error_code else ""
        return f"Naming Error{code}: {self.message}"
