from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from .service_container_aggregate import ServiceContainerAggregate
from ..taxonomy import Duration, ResponseData, AgentStatusVO, BooleanVO

class AgentLifecycleAggregate(BaseModel, ABC):
    """AGGREGATE: Track agent lifecycle state and aggregate system health."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    container: ServiceContainerAggregate
    status: AgentStatusVO = AgentStatusVO(value="init")
    started: BooleanVO = BooleanVO(value=False)

    @property
    @abstractmethod
    def uptime(self) -> Duration:
        """ARCHITECTURAL COMMITMENT: Uptime tracking."""
        ...

    @abstractmethod
    def mark_started(self) -> None:
        """State transition: started."""
        ...

    @abstractmethod
    async def get_health(self) -> ResponseData:
        """AGGREGATOR: Gather system health data."""
        ...

    @abstractmethod
    def mark_stopped(self) -> None:
        """State transition: stopped."""
        ...

    @abstractmethod
    def mark_degraded(self) -> None:
        """State transition: degraded."""
        ...
