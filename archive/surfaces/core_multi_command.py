"""Multi-project CLI commands for auto-linter."""

import os
import json
import click
from typing import Any
from pathlib import Path

from ..taxonomy import BooleanVO, FilePath, SuccessStatus
from ..contract import ServiceContainerAggregate, PipelineOutputAggregate
from ..contract import run_async


class MultiCommandsSurface:
    """Surface for multi-project CLI commands."""

    cli: Any = None
    container: ServiceContainerAggregate | None = None

    def __init__(self, cli: Any):
        self.cli = cli

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Register all multi-project commands."""
        self.container = container

        @self.cli.command("multi-project")
        @click.argument("paths", nargs=-1, type=click.Path(exists=True))
        @click.option(
            "--output-format", type=click.Choice(["text", "json"]), default="text"
        )
        @click.option("--config", "-c", help="Config file with multi-project paths")
        def multi_project_cmd(paths, output_format, config):
            """Run lint across multiple projects and aggregate results."""
            self.multi_project(paths, output_format, config)

    async def execute_multi_project(
        self, container: ServiceContainerAggregate
    ) -> PipelineOutputAggregate:
        """Requirement for MultiProjectAggregate, though primarily used via multi_project command."""
        # This can be used for direct agent-like execution if needed
        return PipelineOutputAggregate(
            status=SuccessStatus(value=BooleanVO(value=True)), message="Surface execution placeholder"
        )

    def multi_project(
        self, paths: Any, output_format="text", config: Any = None
    ) -> None:
        project_list = list(paths or [])
        if not project_list:
            # We use the container to resolve the orchestrator to load config
            # or we just use a helper if it's static.
            # Since load_config was a static method on MultiProjectOrchestrator,
            # and we can't import it, we should probably have it in the Aggregate or as a
            # utility.
            # For now, I'll assume we can use the container to get the orchestrator.
            orchestrator = self.container.multi_project if self.container else None
            if orchestrator:
                project_list = [
                    str(p)
                    for p in orchestrator.load_config(Path(config) if config else None)
                ]
            if not project_list:
                project_list = [os.getcwd()]

        async def _multi():
            report = await self.container.multi_project.scan_all_projects(
                [FilePath(value=p) for p in project_list]
            )
            if output_format == "json":
                click.echo(json.dumps(report.to_dict(), indent=2))
            else:
                click.echo(report.to_text())

        run_async(_multi())


def register_multi_commands(cli: Any, container: ServiceContainerAggregate) -> None:
    """Factory function for backward compatibility."""
    surface = MultiCommandsSurface(cli)
    surface.container = container
    surface.register_all(container)
