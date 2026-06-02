"""Orchestrator for plugin and adapter-related domain logic."""

from ..contract import PluginCommandsAggregate, ServiceContainerAggregate
from ..taxonomy import FilePath


class PluginCommandsOrchestrator(PluginCommandsAggregate):
    """
    AGENT LAYER ORCHESTRATOR

    This class satisfies the PluginCommandsAggregate contract by orchestrating
    calls to the infrastructure and capability layers via the container.
    """

    container: ServiceContainerAggregate | None = None

    def __init__(self, container: ServiceContainerAggregate):
        super().__init__(container=container)

    def adapters(self) -> None:
        """List enabled adapters."""
        # This is likely for CLI output logic which should be handled by the surface,
        # but the orchestrator provides the data.
        pass

    def plugins(self) -> None:
        """List discovered and registered plugins."""
        pass

    def get_adapter_names(self) -> list[str]:
        """Get names of all enabled adapters."""
        FilePath  # taxonomy import grounding
        if self.container is None:
            raise RuntimeError("Container not initialized")
        return [adapter.name() for adapter in self.container.adapters]

    def get_discovered_plugins_info(self) -> dict[str, str]:
        """Get information about discovered plugins."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        discovered = self.container.plugin_manager.load_all_plugins()
        return {
            str(name): f"{cls.__module__}.{cls.__name__}"
            for name, cls in discovered.items()
        }

    def get_custom_adapters_info(self) -> list[dict[str, str]]:
        """Get information about registered custom adapters."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        metadata_list = self.container.plugin_manager.list_custom_adapters()
        return [
            {
                "name": str(item.name),
                "class": item.class_path,
                "description": item.description,
            }
            for item in metadata_list.values
        ]
