"""file_path_vo — File and directory path value objects."""

from pydantic import BaseModel, ConfigDict, field_validator, model_validator


class FilePath(BaseModel):
    """File path identifier."""

    model_config = ConfigDict(frozen=True)
    value: str

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, str):
            return {"value": data}
        if isinstance(data, dict) and "value" in data:
            val = data["value"]
            if hasattr(val, "value") and isinstance(val.value, str):
                return {"value": val.value}
        if hasattr(data, "value") and isinstance(data.value, str):
            return {"value": data.value}
        return data

    @field_validator("value")
    @classmethod
    def check_non_empty(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("File path cannot be empty")
        normalized = v.strip().replace("\\", "/").rstrip("/")
        if not normalized:
            if v.strip() == "/":
                return "/"
            return "."
        return normalized

    def __str__(self) -> str:
        return self.value

    def __hash__(self) -> int:
        return hash(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, FilePath):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented

    @property
    def extension(self) -> str:
        """File extension without dot."""
        special_files = {
            "Makefile",
            "Dockerfile",
            "Dockerfile.dev",
            "Dockerfile.prod",
            ".bashrc",
            ".profile",
            ".zshrc",
            ".gitignore",
            ".dockerignore",
        }
        if self.value in special_files or self.value.startswith("."):
            return ""
        parts = self.value.rsplit(".", 1)
        return parts[-1] if len(parts) > 1 else ""

    def has_extension(self, ext: str) -> bool:
        """Check if path has given extension (without dot)."""
        return self.extension.lower() == ext.lower()


class DirectoryPath(BaseModel):
    """Directory path identifier."""

    model_config = ConfigDict(frozen=True)
    value: str

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, str):
            return {"value": data}
        if isinstance(data, dict) and "value" in data:
            val = data["value"]
            if hasattr(val, "value") and isinstance(val.value, str):
                return {"value": val.value}
        if hasattr(data, "value") and isinstance(data.value, str):
            return {"value": data.value}
        return data

    @field_validator("value")
    @classmethod
    def check_non_empty(cls, v: str) -> str:
        if not v or not v.strip():
            raise ValueError("Directory path cannot be empty")
        return v.strip().replace("\\", "/").rstrip("/")

    def __str__(self) -> str:
        return self.value

    def __hash__(self) -> int:
        return hash(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, DirectoryPath):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented
