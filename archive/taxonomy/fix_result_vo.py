from pydantic import BaseModel, ConfigDict


class FixResult(BaseModel):
    """Result of applying automatic fixes."""

    model_config = ConfigDict(frozen=True)

    output: str
    error: str | None = None

    def is_success(self) -> bool:
        return self.error is None

    def __str__(self) -> str:
        return self.output or (self.error or "")
