"""Fix CLI command - applies safe fixes automatically (Surface)."""

import click
from typing import Any
import os
from ..taxonomy import FilePath
from ..contract import ServiceContainerAggregate
from .cli_output_controller import get_output_dir, write_output, tee_stdout
from ..contract import run_async


class FixCommandsSurface:
    """Surface for lint fixing CLI commands."""

    cli: Any = None
    container: ServiceContainerAggregate | None = None

    def __init__(self, cli, container: ServiceContainerAggregate | None = None):
        self.cli = cli
        self.container = container

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Register all fix commands."""
        self.container = container

        @self.cli.command("fix")
        @click.argument("path", type=click.Path(exists=True), default=".")
        def fix_cmd(path):
            """Apply safe fixes automatically (Ruff, ESLint, Prettier)."""
            self.fix(FilePath(value=os.path.abspath(path)))

    def fix(self, project_path: FilePath) -> None:
        run_async(self._run_fix(project_path))

    async def _run_fix(self, project_path: FilePath) -> None:
        """Internal runner for fix command."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(project_path.value)
        output_dir = get_output_dir()

        with tee_stdout() as tee:
            click.echo(f" Applying safe fixes to {project_path}...")
            # Ensure fix_orchestrator exists on container
            results = await container.fix_orchestrator.execute(project_path)
            click.echo(results)

        if output_dir:
            write_output(tee.getvalue(), "fix", "txt")


def register_fix_commands(cli: Any, container: ServiceContainerAggregate) -> None:
    """Factory function for backward compatibility."""
    surface = FixCommandsSurface(cli, container)
    surface.register_all(container)
