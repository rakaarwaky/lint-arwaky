"""error_code_vo — Error code value object."""

from pydantic import BaseModel, ConfigDict, model_validator


class ErrorCode(BaseModel):
    """Linter error code."""

    model_config = ConfigDict(frozen=True)
    code: str

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, str):
            return {"code": data}
        return data

    def __str__(self) -> str:
        return self.code

    @property
    def is_style(self) -> bool:
        return self.code.startswith(("E", "W", "D"))

    @property
    def is_logic(self) -> bool:
        return self.code.startswith(("F", "I"))

    @property
    def is_security(self) -> bool:
        return self.code.startswith("B")

    @property
    def is_architecture(self) -> bool:
        """Return True for architecture compliance codes (prefix AES)."""
        return self.code.startswith("AES")
