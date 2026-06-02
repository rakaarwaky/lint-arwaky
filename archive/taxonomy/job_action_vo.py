"""job_action_vo — Job and action identifier value objects."""

from pydantic import BaseModel, ConfigDict, model_validator


# ── Single Source of Truth for pipeline action validation ──────────────
# All dispatch tables and validators MUST reference this set.
VALID_PIPELINE_ACTIONS: frozenset[str] = frozenset(
    {
        # Core analysis
        "check",
        "scan",
        "fix",
        "report",
        # Specialized analysis
        "security",
        "complexity",
        "duplicates",
        "trends",
        # System info
        "version",
        "adapters",
        # Hook management
        "install-hook",
        "install_hook",
        "uninstall-hook",
        "uninstall_hook",
        # Multi-project & batch
        "batch",
        "multi_project",
        # Maintenance
        "doctor",
        "cancel",
    }
)


class JobId(BaseModel):
    """Job identifier."""

    model_config = ConfigDict(frozen=True)

    value: str

    def __str__(self) -> str:
        return self.value

    def __hash__(self) -> int:
        return hash(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, JobId):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented


class ActionName(BaseModel):
    """Pipeline action identifier."""

    model_config = ConfigDict(frozen=True)

    value: str

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, str):
            return {"value": data}
        return data

    def __str__(self) -> str:
        return self.value

    def __hash__(self) -> int:
        return hash(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, ActionName):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented
