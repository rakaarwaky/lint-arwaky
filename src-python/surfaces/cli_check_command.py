"""CLI check and scan commands (Surface)."""

import click
from typing import Any
from ..taxonomy import FilePath, GovernanceReport, BooleanVO, Count, AdapterName
from .cli_output_controller import get_output_dir, write_output, tee_stdout


from ..contract import ServiceContainerAggregate
from typing import TYPE_CHECKING
from ..contract import run_async

if TYPE_CHECKING:
    pass


class CheckCommandsSurface:
    """Surface for check-related CLI commands."""

    cli: Any = None
    container: ServiceContainerAggregate | None = None

    def __init__(self, cli=None):
        self.cli = cli

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Register all check commands."""
        self.container = container
        if self.cli is None:
            raise ValueError("CLI group must be provided in constructor")

        @self.cli.command("check")
        @click.argument("path", type=click.Path(exists=True), default=".")
        @click.option(
            "--git-diff",
            is_flag=True,
            help="Only lint files changed in this branch (vs main/master)",
        )
        def check_cmd(path: FilePath, git_diff: BooleanVO):
            """Run all linters and check architecture compliance score."""
            # Coerce primitives from CLI to VOs
            path_vo = FilePath(value=str(path))
            diff_vo = BooleanVO(value=bool(git_diff))
            self.check(path_vo, diff_vo)

        @self.cli.command("scan")
        @click.argument("path", type=click.Path(exists=True))
        def scan_cmd(path: FilePath):
            """Full deep scan of a directory (alias for check)."""
            path_vo = FilePath(value=str(path))
            self.scan(path_vo)

    def check(self, path: FilePath, git_diff: BooleanVO) -> None:
        run_async(self._run_check(path, git_diff))

    def scan(self, path: FilePath) -> None:
        run_async(self._run_check(path, BooleanVO(value=False)))

    async def _handle_git_diff(self, container, project_path: FilePath, tee) -> None:
        """Run check in git-diff mode."""
        report = await container.analysis_orchestrator.run(project_path)

        report_text = self._format_report(report)
        click.echo(report_text)

    def _aggregate_source_counts(self, reports) -> dict:
        """Count issues per source from git diff reports."""
        counts: dict = {}
        for _, report in reports:
            for res in report.results:
                source = res.source or AdapterName(value="unknown")
                current = counts.get(source, Count(value=0))
                counts[source] = Count(value=int(current) + 1)
        return counts

    def _print_source_summary(self, source_counts) -> None:
        """Print one-line summary per source."""
        for source, count in source_counts.items():
            status = " CLEAN" if int(count) == 0 else f" {int(count)} ISSUES"
            click.echo(f"[{source}] {status}")

    def _format_report(self, report: GovernanceReport) -> str:
        """Format governance report into a human-readable string."""
        lines = []

        # Group results by source
        source_results: dict = {}
        for res in report.results:
            src = res.source or AdapterName(value="unknown")
            source_results.setdefault(src, []).append(res)

        # Build issue list
        for source, results in source_results.items():
            status = " CLEAN" if not results else f" {len(results)} ISSUES"
            lines.append(f"[{source}] {status}")
            for res in results:
                lines.append(
                    f" - {res.file.value}:{int(res.line.value)} {res.code}: {res.message}"
                )

        # Add summary at the end
        lines.append("-" * 40)
        lines.append(f"total issues :  {len(report.results)}")
        lines.append(f"total score  :  {report.score.value:.1f}/100.0")
        lines.append("-" * 40)

        return "\n".join(lines)

    async def _handle_full_analysis(
        self, container, project_path: FilePath, tee
    ) -> None:
        """Run check in full directory analysis mode."""
        click.echo(f" Running analysis on {project_path}...")
        report: GovernanceReport = await container.run_analysis(project_path)

        report_text = self._format_report(report)
        click.echo(report_text)

    async def _run_check(self, project_path: FilePath, git_diff: BooleanVO) -> None:
        """Internal runner for check command."""
        if not self.container:
            raise RuntimeError("CheckCommandsSurface not initialized with container")

        container = self.container.get_for_path(str(project_path))
        output_dir = get_output_dir()

        with tee_stdout() as tee:
            if bool(git_diff):
                await self._handle_git_diff(container, project_path, tee)
            else:
                await self._handle_full_analysis(container, project_path, tee)

        if output_dir:
            write_output(tee.getvalue(), "check", "txt")


def register_check_commands(cli, container: ServiceContainerAggregate) -> None:
    """Factory function for check commands surface."""
    surface = CheckCommandsSurface(cli)
    surface.register_all(container)
