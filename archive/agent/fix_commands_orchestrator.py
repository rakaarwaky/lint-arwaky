"""FixCommandsOrchestrator — Implementation of FixCommandsAggregate (Agent Logic)."""

from ..contract import FixCommandsAggregate, ServiceContainerAggregate
from ..taxonomy import FilePath


class FixCommandsOrchestrator(FixCommandsAggregate):
    """Orchestrator that handles fix-related domain logic for the agent."""

    container: ServiceContainerAggregate | None = None

    def __init__(self, container: ServiceContainerAggregate) -> None:
        super().__init__(container=container)
        self.container = container

    async def fix(self, project_path: FilePath) -> None:
        """Logic: Apply safe fixes automatically."""
        # Delegates to the internal fix_orchestrator
        # We can use the container to get or create the project-specific container if
        # needed,
        # but usually the coordinator is already within the correct context.
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(project_path.value)
        await container.fix_orchestrator.execute(project_path)
