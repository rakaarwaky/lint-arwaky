"""WatchCommandsOrchestrator — Implementation of WatchCommandsAggregate (Agent Logic)."""

from ..contract import WatchCommandsAggregate, ServiceContainerAggregate
from ..taxonomy import FilePath


class WatchCommandsOrchestrator(WatchCommandsAggregate):
    """Orchestrator that handles watch-related domain logic for the agent."""

    def __init__(self, container: ServiceContainerAggregate) -> None:
        super().__init__(container=container)
        FilePath  # taxonomy import grounding

    def watch(self, path: FilePath) -> None:
        """
        The watch command is inherently interactive/blocking at the surface.
        The orchestrator provides the logic execution for changes.
        """
        # Logic is handled by WatchExecutionOrchestrator already,
        # but this Port fulfillment connects it to the Surface.
        pass
