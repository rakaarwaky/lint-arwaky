import click
import os
from typing import Any

from ..taxonomy import JobId, FilePath
from ..contract import MaintenanceCommandsAggregate, ServiceContainerAggregate


class MaintenanceCommandsSurface:
    """Surface for maintenance-related CLI commands."""

    cli: Any = None
    container: ServiceContainerAggregate | None = None

    def __init__(self, cli, container: ServiceContainerAggregate | None = None):
        self.cli = cli
        self.container = container

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Register all maintenance commands."""
        self.container = container

        @self.cli.command("stats")
        @click.argument("path", type=click.Path(exists=True), default=".")
        def stats_cmd(path):
            """Show statistics dashboard."""
            self.stats(FilePath(value=os.path.abspath(path)))

        @self.cli.command("clean")
        def clean_cmd():
            """Cleanup cache and temporary files."""
            self.clean()

        @self.cli.command("update")
        def update_cmd():
            """Update linter adapters to latest versions."""
            self.update()

        @self.cli.command("doctor")
        def doctor_cmd():
            """Diagnose common issues."""
            self.doctor()

        @self.cli.command("cancel")
        @click.argument("job_id")
        def cancel_cmd(job_id):
            """Cancel a running lint job."""
            self.cancel(JobId(value=job_id))

    @click.argument("path", type=click.Path(exists=True), default=".")
    def stats(self, project_path: FilePath) -> None:
        """Show statistics dashboard."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        orchestrator = self.container.get(MaintenanceCommandsAggregate)
        stats = orchestrator.stats(project_path)

        click.echo(f" Auto-Linter Statistics for {stats.project_path.value}")
        click.echo("=" * 50)
        click.echo(f" Python files: {stats.python_files}")
        click.echo(f" Test files: {stats.test_files}")
        click.echo(f" Test ratio: {stats.test_ratio:.1f}%")
        click.echo("=" * 50)

    def clean(self) -> None:
        """Cleanup cache and temporary files."""
        click.echo(" Cleaning cache...")
        if self.container is None:
            raise RuntimeError("Container not initialized")
        orchestrator = self.container.get(MaintenanceCommandsAggregate)
        orchestrator.clean()
        click.echo(" Cleanup complete.")

    def update(self) -> None:
        """Update linter adapters to latest versions."""
        click.echo(" Updating adapters...")
        if self.container is None:
            raise RuntimeError("Container not initialized")
        orchestrator = self.container.get(MaintenanceCommandsAggregate)
        orchestrator.update()
        click.echo("\n Update complete")

    def doctor(self) -> None:
        """Diagnose common issues."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        orchestrator = self.container.get(MaintenanceCommandsAggregate)
        result = orchestrator.doctor()

        click.echo(" Auto-Linter Doctor")
        click.echo("=" * 50)
        click.echo(f" Python: {result.python_version}")
        click.echo(
            f" Status: {'Installed' if result.is_installed else 'NOT INSTALLED'}"
        )

        if result.config_found:
            click.echo(f" Config: {', '.join(result.config_found)}")
        else:
            click.echo(" Config: NOT FOUND")

        click.echo("\n Adapters:")
        for adapter, status in result.adapter_statuses.items():
            click.echo(f"  - {adapter}: {status}")

        click.echo("=" * 50)
        if result.issues:
            click.echo(f"\n Found {len(result.issues)} issue(s):")
            for issue in result.issues:
                click.echo(f" - {issue}")
        else:
            click.echo("\n All systems healthy!")

    @click.argument("job_id")
    def cancel(self, job_id: JobId) -> None:
        """Cancel a running lint job."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        orchestrator = self.container.get(MaintenanceCommandsAggregate)
        orchestrator.cancel(job_id)
        click.echo(f"Request to cancel job {job_id.value} sent.")


def register_maintenance_commands(cli, container: ServiceContainerAggregate) -> None:
    """Factory function for backward compatibility."""
    surface = MaintenanceCommandsSurface(cli, container)
    surface.register_all(container)
