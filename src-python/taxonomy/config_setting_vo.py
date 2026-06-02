"""config_setting_vo — Value objects for configuration domain (Pydantic)."""

from enum import Enum
from pydantic import BaseModel, ConfigDict, Field, field_validator
from .path_collection_vo import PatternList, FilePathList
from .architecture_config_vo import ArchitectureConfig
from .file_path_vo import FilePath
from .score_format_vo import Score
from .message_status_vo import Count


class Thresholds(BaseModel):
    """Scoring thresholds."""

    model_config = ConfigDict(frozen=True)

    score: Score = Score(value=80.0)
    complexity: Count = Count(value=10)
    max_file_lines: Count = Count(value=500)


DEFAULT_THRESHOLDS = Thresholds()


class AdapterStatus(str, Enum):
    ENABLED = "enabled"
    DISABLED = "disabled"
    NOT_INSTALLED = "not_installed"


class AdapterEntry(BaseModel):
    """Single adapter configuration."""

    model_config = ConfigDict(frozen=True)

    name: str
    status: AdapterStatus = AdapterStatus.ENABLED
    weight: float = 1.0

    @property
    def is_active(self) -> bool:
        return self.status == AdapterStatus.ENABLED


class ProjectConfig(BaseModel):
    """Project configuration."""

    model_config = ConfigDict(frozen=True)

    project_name: str = "auto-linter"
    thresholds: Thresholds = Thresholds()
    adapters: list[AdapterEntry] = []
    ignored_paths: FilePathList = Field(default_factory=FilePathList)
    ignored_rules: PatternList = Field(default_factory=PatternList)
    governance_rules: list[dict[str, str]] = []
    layer_map: dict[str, str] = {}
    output_dir: str | None = None
    architecture: ArchitectureConfig = ArchitectureConfig()  # Full architecture config

    @field_validator("ignored_paths", mode="before")
    @classmethod
    def validate_ignored_paths(cls, v):
        if isinstance(v, list):
            return FilePathList(
                values=[FilePath(value=p) if isinstance(p, str) else p for p in v]
            )
        return v

    @field_validator("ignored_rules", mode="before")
    @classmethod
    def validate_ignored_rules(cls, v):
        if isinstance(v, list):
            return PatternList(values=v)
        return v

    @classmethod
    def defaults(cls) -> "ProjectConfig":
        """Return a ProjectConfig with all default values (ruff, mypy, bandit, radon enabled)."""
        return cls(
            adapters=[
                AdapterEntry(name="ruff", status=AdapterStatus.ENABLED, weight=1.0),
                AdapterEntry(name="mypy", status=AdapterStatus.ENABLED, weight=1.0),
                AdapterEntry(name="bandit", status=AdapterStatus.ENABLED, weight=1.0),
                AdapterEntry(name="radon", status=AdapterStatus.ENABLED, weight=1.0),
            ]
        )
