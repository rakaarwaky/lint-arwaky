"""plugin_manager_port — Interface for plugin discovery and management."""

from abc import ABC, abstractmethod
from ..taxonomy import AdapterName, PluginGroup, AdapterMetadataList, AdapterClassMap, PluginError


class IPluginManagerPort(ABC):
    """Port for discovering and managing third-party adapters via entry points."""

    @abstractmethod
    def discover_plugins(self, group: PluginGroup) -> AdapterClassMap | PluginError:
        """Discover custom adapters via entry points and register them."""
        ...

    @abstractmethod
    def list_custom_adapters(self) -> AdapterMetadataList:
        """Return a list of all registered custom adapters with metadata."""
        ...

    @abstractmethod
    def register_custom_adapter(self, name: AdapterName, adapter_class: type) -> PluginError | None:
        """Manually register a custom adapter class."""
        ...
