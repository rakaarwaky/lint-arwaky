"""Plugin and adapter-related CLI commands for auto-linter."""

import click
from typing import Any
from ..contract import PluginCommandsAggregate, ServiceContainerAggregate


class PluginCommandsSurface:
    """Surface for plugin and adapter-related CLI commands."""

    cli: Any = None
    container: ServiceContainerAggregate | None = None

    @property
    def _INTERFACE(self) -> type[PluginCommandsAggregate]:
        """ARCHITECTURAL COMMITMENT: Required interface."""
        return PluginCommandsAggregate

    def __init__(self, cli=None):
        """Initialize surface."""
        super().__init__()
        self.cli = cli

    def register_all(self, container: ServiceContainerAggregate, cli: Any = None) -> None:
        """Register all plugin commands."""
        self.container = container
        target_cli = cli or self.cli
        if target_cli is None:
            raise ValueError(
                "CLI group must be provided to register_all or in constructor"
            )

        target_cli.command("adapters")(self.adapters)
        target_cli.command("plugins")(self.plugins)

    def adapters(self) -> None:
        """List enabled adapters."""
        click.echo("Enabled Adapters:")
        if self.container is None:
            raise RuntimeError("Container not initialized")
        # Use container's public adapters property
        for adapter in self.container.adapters:
            click.echo(f" - {adapter.name()}")

    def plugins(self) -> None:
        """List discovered and registered plugins."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        discovered = self.container.plugin_commands.get_discovered_plugins_info()
        if discovered:
            click.echo("Discovered Plugins:")
            for name, module_cls in discovered.items():
                click.echo(f"  {name}: {module_cls}")

        if self.container is None:
            raise RuntimeError("Container not initialized")
        custom = self.container.plugin_commands.get_custom_adapters_info()
        if custom:
            click.echo("\nRegistered Custom Adapters:")
            for info in custom:
                click.echo(f"  {info['name']}: {info['class']}")

        if not discovered and not custom:
            click.echo("No plugins or custom adapters found.")
            click.echo("\nTo register a plugin, add entry point in pyproject.toml:")
            click.echo('  [project.entry-points."auto_linter.adapters"]')
            click.echo("  my_adapter = my_package:MyAdapterClass")


def register_plugin_commands(cli: Any, container: ServiceContainerAggregate) -> None:
    """Factory function for backward compatibility."""
    surface = PluginCommandsSurface(cli)
    surface.register_all(container)


# No more legacy static exports
