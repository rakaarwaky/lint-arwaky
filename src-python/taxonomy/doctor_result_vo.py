from pydantic import BaseModel, ConfigDict


class DoctorResultVO(BaseModel):
    """Value Object for doctor command results."""

    model_config = ConfigDict(frozen=True)

    python_version: str
    is_installed: bool
    config_found: list[str]
    adapter_statuses: dict[str, str]
    issues: list[str]
    healthy: bool
