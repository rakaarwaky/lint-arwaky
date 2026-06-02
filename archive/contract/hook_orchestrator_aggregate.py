"""hook_orchestrator_aggregate - Aggregate contract for hook orchestrator."""
from abc import ABC, abstractmethod
from typing import Type
from pydantic import BaseModel, ConfigDict
from ..taxonomy import Identity
from .hook_manager_port import IHookManagerPort

class HookManagementOrchestratorAggregate(BaseModel, ABC):
    """AGGREGATE: Defines the hook management boundaries."""
    model_config = ConfigDict(arbitrary_types_allowed=True)

    @property
    @abstractmethod
    def _INTERFACE_HOOK_MANAGER(self) -> Type[IHookManagerPort]:
        """ARCHITECTURAL COMMITMENT: Required infrastructure for git hooks."""
        ...

    manager: IHookManagerPort | None = None

    @abstractmethod
    def get_hook_manager_identity(self) -> Identity:
        """Get the identity of the hook manager."""
        ...
