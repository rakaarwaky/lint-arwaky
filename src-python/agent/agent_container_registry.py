"""agent_container_registry — Registry and provider for project-specific DI containers."""
from __future__ import annotations
import os
import logging
import threading
from typing import TYPE_CHECKING, MutableMapping
from ..contract import ServiceContainerAggregate, ContainerRegistryAggregate
from ..taxonomy import DirectoryPath

if TYPE_CHECKING:
    from .dependency_injection_container import Container

logger = logging.getLogger("auto_linter.agent")


class AgentContainerRegistry(ContainerRegistryAggregate):
    """Registry for the Agent DI containers.
    
    Manages a mapping of project roots to their respective Container instances.
    This enables the system to handle multiple projects with distinct configurations
    simultaneously.
    """

    _containers: MutableMapping[str, "Container"] = {}
    _default_root: DirectoryPath = DirectoryPath(value=os.getcwd())
    _lock: threading.Lock = threading.Lock()

    @staticmethod
    def get_container(project_root: DirectoryPath | None = None) -> ServiceContainerAggregate:
        """Get or create a container for a specific project root.
        
        Args:
            project_root: The absolute path to the project root. If None, uses the 
                         current working directory as the default project root.
                        
        Returns:
            A ProjectAggregateFacade wrapping the Container instance for the specified project root.
        """
        from .dependency_injection_container import Container, ProjectAggregateFacade

        # Normalize to absolute path for consistent registry lookup
        root = DirectoryPath(value=os.path.abspath(str(project_root))) if project_root else AgentContainerRegistry._default_root

        with AgentContainerRegistry._lock:
            if root not in AgentContainerRegistry._containers:
                logger.info(f"Initializing new container for project root: {root}")
                AgentContainerRegistry._containers[root] = Container(project_root=str(root))

            return ProjectAggregateFacade(AgentContainerRegistry._containers[root])

    @staticmethod
    def reset_container(project_root: DirectoryPath | None = None) -> None:
        """Reset container(s) in the registry."""
        with AgentContainerRegistry._lock:
            if project_root:
                root = DirectoryPath(value=os.path.abspath(str(project_root)))
                if root in AgentContainerRegistry._containers:
                    logger.info(f"Resetting container for project root: {root}")
                    del AgentContainerRegistry._containers[root]
            else:
                logger.info("Clearing all containers from registry")
                AgentContainerRegistry._containers.clear()


get_container = AgentContainerRegistry.get_container
reset_container = AgentContainerRegistry.reset_container
