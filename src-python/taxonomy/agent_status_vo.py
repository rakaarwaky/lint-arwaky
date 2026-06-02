"""agent_status_vo — Agent lifecycle status value objects."""

from enum import Enum
from pydantic import BaseModel, ConfigDict, model_validator


class AgentStatus(str, Enum):
    """Lifecycle status of an agent."""

    INIT = "init"
    STARTED = "started"
    STOPPED = "stopped"
    DEGRADED = "degraded"


class AgentStatusVO(BaseModel):
    """Value object wrapper for agent status."""

    model_config = ConfigDict(frozen=True)
    value: AgentStatus

    @model_validator(mode="before")
    @classmethod
    def coerce_from_primitive(cls, data: object) -> object:
        if isinstance(data, (str, AgentStatus)):
            return {"value": data}
        return data

    def __str__(self) -> str:
        return self.value.value

    def __eq__(self, other: object) -> bool:
        if isinstance(other, AgentStatusVO):
            return self.value == other.value
        if isinstance(other, (str, AgentStatus)):
            return self.value == other
        return NotImplemented
