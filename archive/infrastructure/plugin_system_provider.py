"""plugin_system — Entry point discovery and loading for custom adapters.

Allows third-party adapters to be discovered via Python entry points.

Usage in pyproject.toml of a plugin package:
    [project.entry-points."auto_linter.adapters"]
    my_custom_adapter = my_plugin:MyCustomAdapter

Then auto-linter will auto-discover it.
"""

from __future__ import annotations

from ..taxonomy import AdapterMetadata, AdapterMetadataList, AdapterName, PluginGroup

import importlib.metadata
import logging
from typing import Type
from ..contract import IPluginManagerPort


logger = logging.getLogger(__name__)


class PluginSystemProvider(IPluginManagerPort):
    """Discovery and registration system for adapter plugins."""

    def __init__(self) -> None:
        # Registry of discovered custom adapters
        self._custom_adapters: dict = {}

    def discover_plugins(
        self, group: PluginGroup = PluginGroup(value="auto_linter.adapters")
    ):
        """Discover custom adapters via entry points.

        Scans for entry points registered under the given group.

        Args:
            group: Entry point group name (default: PluginGroup("auto_linter.adapters"))

        Returns:
            Dict mapping adapter name to adapter class.
        """
        discovered = {}

        try:
            eps = importlib.metadata.entry_points()
            # Python 3.10+ returns SelectableGroups, 3.9 returns dict
            if hasattr(eps, "select"):
                group_eps = list(eps.select(group=str(group)))
            elif isinstance(eps, dict):
                group_eps = eps.get(group, [])
            else:
                group_eps = []

            for ep in group_eps:
                try:
                    adapter_class = ep.load()
                    discovered[AdapterName(value=ep.name)] = adapter_class
                    logger.info(f"Discovered plugin: {ep.name} -> {adapter_class}")
                except Exception as e:
                    logger.warning(f"Failed to load plugin {ep.name}: {e}")
        except Exception as e:
            logger.warning(f"Entry point discovery failed: {e}")

        return discovered

    def register_custom_adapter(self, name: AdapterName, adapter_class: Type):
        """Manually register a custom adapter."""
        self._custom_adapters[name] = adapter_class
        logger.info(f"Manually registered adapter: {name}")

    def unregister_custom_adapter(self, name: AdapterName) -> Type | None:
        """Unregister a custom adapter.

        Args:
            name: Adapter name to unregister.

        Returns:
            The unregistered class, or None if not found.
        """
        return self._custom_adapters.pop(name, None)

    def get_custom_adapter(self, name: AdapterName) -> Type | None:
        """Get a registered custom adapter by name.

        Args:
            name: Adapter name.

        Returns:
            The adapter class, or None if not found.
        """
        return self._custom_adapters.get(name)

    def list_custom_adapters(self) -> AdapterMetadataList:
        """List all registered custom adapters.

        Returns:
            List of dicts with adapter info.
        """
        vals = []
        for name, cls in self._custom_adapters.items():
            vals.append(
                AdapterMetadata(
                    name=name,
                    class_path=f"{cls.__module__}.{cls.__name__}",
                    description=cls.__doc__ or "",
                )
            )
        return AdapterMetadataList(values=vals)

    def load_all_plugins(self):
        """Discover and register all plugins from entry points.

        Returns:
            Dict of all discovered adapter classes.
        """
        discovered = self.discover_plugins()
        for name, cls in discovered.items():
            self.register_custom_adapter(name, cls)
        return self._custom_adapters.copy()
