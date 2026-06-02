"""config_identifier_vo — Configuration key value object."""

from pydantic import BaseModel, ConfigDict, field_validator


class ConfigKey(BaseModel):
    """Immutable config key with dot-notation path support."""

    model_config = ConfigDict(frozen=True)

    value: str

    @field_validator("value")
    @classmethod
    def check_non_empty(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("Config key cannot be empty")
        return v.strip()

    def __str__(self) -> str:
        return self.value

    def __hash__(self) -> int:
        return hash(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, ConfigKey):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented

    @property
    def parts(self) -> list[str]:
        """Split key into dot-separated parts."""
        return self.value.split(".")

    @property
    def parent(self) -> str:
        """Return parent key path (everything except last segment)."""
        parts = self.parts
        return ".".join(parts[:-1]) if len(parts) > 1 else ""

    @property
    def leaf(self) -> str:
        """Return last segment of the key."""
        return self.parts[-1] if self.parts else self.value
