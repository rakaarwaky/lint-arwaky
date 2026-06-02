"""Entry point for the auto-lint CLI.

This module provides the :func:`main` entry point used by the
``auto-lint`` console script. It wires together all command groups
and exposes a Click-based CLI.

All imports are performed at module level after bootstrapping the
Python path to guarantee that the local ``agent``, ``contract`` and
other top-level packages are resolved correctly even when a global
installation with conflicting names exists.
"""

import logging
import sys
from typing import Any
from .cli_core_command import get_cli, get_surface
from .cli_analysis_command import register_analysis_commands
from .cli_maintenance_command import register_maintenance_commands
from .cli_watch_command import register_watch_command
from .cli_setup_command import get_setup
from .cli_output_controller import set_container
from ..contract import ServiceContainerAggregate


class MainHandlerSurface:
    """Main entry surface for the Auto-Linter CLI.

    This surface acts as the primary boundary between the external CLI
    and the internal Agent layer, ensuring all primitive inputs are
    mapped to Taxonomy VOs.
    """

    cli: Any = None
    container: Any = None
    container_cls: Any = None

    def __init__(self, container: ServiceContainerAggregate | None = None):
        self.cli = get_cli()
        self.container = container
        if container is not None:
            self._register_extensions()

    def _register_extensions(self) -> None:
        """Register all command extensions."""
        if self.container is not None:
            set_container(self.container)

        core_surface = get_surface()

        # 1. Register setup commands
        self.cli.add_command(get_setup())

        # 2. Register core command groups
        core_surface.register_all(self.container)

        # 3. Register specialized extensions
        register_analysis_commands(self.cli, self.container)
        register_maintenance_commands(self.cli, self.container)
        register_watch_command(self.cli, self.container)

    def execute(self) -> None:
        """Execute the CLI application."""
        logging.basicConfig(
            level=logging.ERROR,
            format="%(levelname)s: %(message)s",
            stream=sys.stderr,
        )
        self.cli()
