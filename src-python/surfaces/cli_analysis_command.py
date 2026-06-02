"""Analysis CLI commands: complexity, duplicates, trends, ci, batch, dependencies."""

from __future__ import annotations
from typing import Any

import click
import os
import sys

from ..taxonomy import FilePath, GovernanceReport
from ..contract import ServiceContainerAggregate
from .cli_output_controller import get_output_dir, write_output, tee_stdout
from ..contract import run_async


class AnalysisCommandsSurface:
    """Surface for analysis-related CLI commands."""

    cli: Any = None
    container: ServiceContainerAggregate | None = None

    def __init__(self, cli, container: ServiceContainerAggregate | None = None):
        self.cli = cli
        self.container = container

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Register all analysis commands."""
        self.container = container

        @self.cli.command("complexity")
        @click.argument("path", type=click.Path(exists=True))
        def complexity_cmd(path):
            """Run cyclomatic complexity analysis."""
            self.complexity(path)

        @self.cli.command("duplicates")
        @click.argument("path", type=click.Path(exists=True))
        def duplicates_cmd(path):
            """Find code duplication or oversized files."""
            self.duplicates(path)

        @self.cli.command("trends")
        @click.argument("path", type=click.Path(exists=True))
        def trends_cmd(path):
            """Show quality trends over time."""
            self.trends(path)

        @self.cli.command("ci")
        @click.argument("path", type=click.Path(exists=True))
        @click.option("--exit-zero", is_flag=True, help="Always return exit code 0")
        def ci_cmd(path, exit_zero):
            """CI-optimized scan with exit codes."""
            self.ci(path, exit_zero)

        @self.cli.command("batch")
        @click.argument("paths", nargs=-1, type=click.Path(exists=True))
        def batch_cmd(paths):
            """Run check on multiple paths."""
            self.batch(paths)

        @self.cli.command("dependencies")
        @click.argument("path", type=click.Path(exists=True))
        def dependencies_cmd(path):
            """Scan for dependency vulnerabilities (pip-audit)."""
            self.dependencies(path)

    @click.argument("path", type=click.Path(exists=True))
    def complexity(self, path) -> None:
        """Run cyclomatic complexity analysis."""
        # Use path-specific container for logic
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(path)
        abs_path = os.path.abspath(path)
        output_dir = get_output_dir()

        async def _complexity() -> object:
            with tee_stdout() as tee:
                click.echo(f" Analyzing complexity in {abs_path}...")
                report: GovernanceReport = (
                    await container.analysis_orchestrator.get_complexity(
                        FilePath(value=abs_path)
                    )
                )

                if not report.results:
                    click.echo(" Complexity is within healthy limits.")
                else:
                    click.echo(
                        f" Found {len(report.results)} high complexity functions."
                    )
                    for res in report.results:
                        click.echo(
                            f" - {res.file}:{res.line.value} {res.message.value}"
                        )

                return tee.getvalue()

        output = run_async(_complexity())
        if output_dir:
            write_output(output, "complexity", "txt")

    @click.argument("path", type=click.Path(exists=True))
    def duplicates(self, path) -> None:
        """Find code duplication or oversized files."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(path)
        abs_path = os.path.abspath(path)
        output_dir = get_output_dir()

        async def _duplicates() -> object:
            with tee_stdout() as tee:
                click.echo(f" Scanning for duplicates in {abs_path}...")
                report: GovernanceReport = (
                    await container.analysis_orchestrator.get_duplicates(
                        FilePath(value=abs_path)
                    )
                )
                if not report.results:
                    click.echo(" No major duplication issues detected.")
                else:
                    for res in report.results:
                        click.echo(
                            f" - {res.file}:{res.line.value} {res.message.value}"
                        )
                return tee.getvalue()

        output = run_async(_duplicates())
        if output_dir:
            write_output(output, "duplicates", "txt")

    @click.argument("path", type=click.Path(exists=True))
    def trends(self, path) -> None:
        """Show quality trends over time."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(path)
        abs_path = os.path.abspath(path)
        output_dir = get_output_dir()

        async def _trends() -> object:
            with tee_stdout() as tee:
                report: GovernanceReport = (
                    await container.analysis_orchestrator.get_trends(
                        FilePath(value=abs_path)
                    )
                )
                if not report.results:
                    click.echo(" Quality trend: STABLE or IMPROVING")
                else:
                    for res in report.results:
                        click.echo(f" {res.message.value}")
                return tee.getvalue()

        output = run_async(_trends())
        if output_dir:
            write_output(output, "trends", "txt")

    @click.argument("path", type=click.Path(exists=True))
    @click.option("--exit-zero", is_flag=True, help="Always return exit code 0")
    def ci(self, path, exit_zero) -> None:
        """CI-optimized scan with exit codes."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(path)
        abs_path = os.path.abspath(path)
        output_dir = get_output_dir()
        ci_failed = False

        async def _ci() -> object:
            nonlocal ci_failed
            with tee_stdout() as tee:
                report: GovernanceReport = await container.run_analysis(
                    FilePath(value=abs_path)
                )
                click.echo(
                    f"CI Scan: score={report.score.value:.1f}, passing={report.is_passing.value}"
                )
                if not report.is_passing.value and not exit_zero:
                    ci_failed = True
                return tee.getvalue()

        output = run_async(_ci())
        if output_dir:
            write_output(output, "ci", "txt")

        if ci_failed:
            sys.exit(1)

    @click.argument("paths", nargs=-1, type=click.Path(exists=True))
    def batch(self, paths) -> None:
        """Run check on multiple paths."""
        if not paths:
            click.echo("No paths provided.")
            return

        all_passing = True
        output_dir = get_output_dir()

        async def _batch() -> object:
            nonlocal all_passing
            with tee_stdout() as tee:
                for path in paths:
                    abs_path = os.path.abspath(path)
                    click.echo(f"Checking {abs_path}...")
                    if self.container is None:
                        raise RuntimeError("Container not initialized")
                    container = self.container.get_for_path(abs_path)
                    report: GovernanceReport = await container.run_analysis(
                        FilePath(value=abs_path)
                    )
                    if not report.is_passing.value:
                        all_passing = False
                        click.echo(f" FAILED: {abs_path}")
                    else:
                        click.echo(f" PASSED: {abs_path}")
                return tee.getvalue()

        output = run_async(_batch())
        if output_dir:
            write_output(output, "batch", "txt")

        if not all_passing:
            sys.exit(1)

    @click.argument("path", type=click.Path(exists=True))
    def dependencies(self, path) -> None:
        """Scan for dependency vulnerabilities (pip-audit)."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(path)
        abs_path = os.path.abspath(path)
        output_dir = get_output_dir()

        async def _dependencies() -> object:
            with tee_stdout() as tee:
                click.echo(f" Scanning for dependency vulnerabilities in {abs_path}...")
                report: GovernanceReport = (
                    await container.analysis_orchestrator.get_dependencies(
                        FilePath(value=abs_path)
                    )
                )
                if not report.results:
                    click.echo(" No dependency vulnerabilities found.")
                else:
                    click.echo(f" Found {len(report.results)} vulnerable packages.")
                    for res in report.results:
                        click.echo(
                            f" - {res.message.value} (Severity: {res.severity.value})"
                        )
                return tee.getvalue()

        output = run_async(_dependencies())
        if output_dir:
            write_output(output, "dependencies", "txt")


def register_analysis_commands(cli, container: ServiceContainerAggregate) -> None:
    """Factory function for analysis commands surface."""
    surface = AnalysisCommandsSurface(cli, container)
    surface.register_all(container)
