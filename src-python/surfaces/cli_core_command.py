"""Core CLI commands: cli, check, scan, fix, report, version, adapters, security."""

import click
import os
from typing import Any
import importlib.metadata
from .core_git_command import register_git_commands
from .core_multi_command import register_multi_commands
from .core_plugin_command import register_plugin_commands
from .core_report_command import register_report_commands
from .cli_fix_command import register_fix_commands
from .cli_check_command import register_check_commands
from .cli_setup_command import register_setup_commands
from .cli_dev_command import register_dev_commands
from ..contract import ServiceContainerAggregate


class CoreCommandsSurface:
    """Surface for core CLI command structure."""

    cli: Any = None
    container: ServiceContainerAggregate | None = None

    def __init__(self):
        self.cli = self._build_cli()
        # No longer registering subcommands in __init__ to wait for container

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Core command registration fulfillment."""
        self.container = container
        self._register_subcommands(container)

    def _build_cli(self) -> click.Group:
        """Build the main CLI group."""

        @click.group()
        @click.option("--verbose", "-v", is_flag=True, help="Show debug information")
        @click.option("--quiet", "-q", is_flag=True, help="Minimize output")
        @click.option(
            "--output-dir",
            "-o",
            help="Directory to save output reports (overrides config)",
        )
        def cli(verbose, quiet, output_dir):
            """Auto-Linter CLI: Autonomous Code Quality Gatekeeper."""
            if verbose:
                os.environ["MCP_LOG_LEVEL"] = "DEBUG"
            elif quiet:
                os.environ["MCP_LOG_LEVEL"] = "WARNING"

            # Store output_dir in context for subcommands
            ctx = click.get_current_context()
            ctx.ensure_object(dict)
            ctx.obj["output_dir"] = output_dir

        return cli

    def _register_subcommands(self, container: ServiceContainerAggregate) -> None:
        """Register all subcommands to the main group."""

        @self.cli.command()
        def version():
            """Show version information."""
            try:
                ver = importlib.metadata.version("auto-linter")
            except importlib.metadata.PackageNotFoundError:
                ver = "1.6.3"
            click.echo(f"Auto-Linter v{ver} (AES Semantic Builder)")

        # Register check and scan commands
        register_check_commands(self.cli, container)

        # Register report and security commands
        register_report_commands(self.cli, container)

        # Register fix command
        register_fix_commands(self.cli, container)

        # Register git commands
        register_git_commands(self.cli, container)

        # Register plugin-related commands
        register_plugin_commands(self.cli, container)

        # Register multi-project commands
        register_multi_commands(self.cli, container)

        # Register setup commands
        register_setup_commands(self.cli, container)

        # Register dev commands (diff, suggest, config, export)
        register_dev_commands(self.cli, container)


# Lazy singleton — created on first call to avoid import-time side effects
_Instance = None


def get_cli():
    global _Instance
    if _Instance is None:
        _Instance = CoreCommandsSurface()
    return _Instance.cli


def get_surface():
    global _Instance
    if _Instance is None:
        _Instance = CoreCommandsSurface()
    return _Instance


cli = None  # placeholder; use get_cli() instead
