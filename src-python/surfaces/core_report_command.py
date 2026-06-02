"""Report and security CLI commands for auto-linter."""

import json
import click
import os
from typing import Any
from collections import defaultdict

from ..taxonomy import FilePath, GovernanceReport
from ..contract import ServiceContainerAggregate
from datetime import datetime
from pathlib import Path
from ..contract import run_async


class ReportCommandsSurface:
    """Surface for report and security CLI commands."""

    cli: Any = None
    container: ServiceContainerAggregate | None = None

    def __init__(self, cli=None):
        """Initialize surface."""
        self.cli = cli

    def register_reports(self, container: ServiceContainerAggregate) -> None:
        """Register all report commands."""
        self.container = container
        if self.cli is None:
            raise ValueError("CLI group must be provided in constructor")

        @self.cli.command("report")
        @click.argument("path", type=click.Path(exists=True))
        @click.option(
            "--output-format",
            type=click.Choice(["text", "json", "sarif", "junit"]),
            default="text",
        )
        def report_cmd(path, output_format):
            """Generate a detailed quality report."""
            self.report(path, output_format)

        @self.cli.command("security")
        @click.argument("path", type=click.Path(exists=True))
        def security_cmd(path):
            """Run security-focused scan (Bandit, etc.)."""
            self.security(path)

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Compatibility wrapper for CoreCommandsSurface."""
        self.register_reports(container)

    def _get_output_dir(self) -> Path | None:
        """Get the effective output directory from CLI flag or config."""
        ctx = click.get_current_context(silent=True)
        cli_output_dir = ctx.obj.get("output_dir") if ctx and ctx.obj else None

        if cli_output_dir:
            return Path(cli_output_dir)

        config = getattr(self.container, "config", None)
        if config and config.project and config.project.output_dir:
            return Path(config.project.output_dir)

        return None

    def _write_output(self, output, command, output_format="txt") -> Path | None:
        """Write output to a timestamped file in the output directory."""
        output_dir = self._get_output_dir()
        if not output_dir:
            return None

        output_dir = Path(output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)

        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        filename = f"{command}_{timestamp}.{output_format}"
        output_path = output_dir / filename

        output_path.write_text(output)
        return output_path

    def report(self, path, output_format) -> None:
        """Generate a detailed quality report."""
        abs_path = os.path.abspath(path)

        class TeeOutput:
            def __init__(self):
                self.lines = []

            def echo(self, msg=""):
                click.echo(msg)
                self.lines.append(str(msg))

            def get_output(self):
                return "\n".join(self.lines)

        tee = TeeOutput()

        async def _run_report() -> None:
            if self.container is None:
                raise RuntimeError("Container not initialized")
            container = self.container.get_for_path(abs_path)
            report_data: GovernanceReport = await container.run_analysis(
                FilePath(value=abs_path)
            )

            if output_format == "json":
                data = container.report_formatter.report_to_dict(report_data)
                result = json.dumps(data, indent=2)
                tee.echo(result)
            elif output_format == "sarif":
                result = container.report_formatter.to_sarif(report_data).value
                tee.echo(result)
            elif output_format == "junit":
                result = container.report_formatter.to_junit(report_data).value
                tee.echo(result)
            else:
                # Text format
                tee.echo(f"--- Quality Report for {abs_path} ---")
                tee.echo(
                    f"Architecture Compliance Score: {report_data.score.value:.1f}"
                )
                # Group results by source
                grouped = defaultdict(list)
                for r in report_data.results:
                    src = str(r.source or "unknown")
                    grouped[src].append(r)

                for source in sorted(grouped.keys()):
                    results = grouped[source]
                    status = " CLEAN" if not results else f" {len(results)} ISSUES"
                    tee.echo(f"[{source}] {status}")
                    for res in results:
                        tee.echo(
                            f" - {res.file.value}:{res.line.value} {res.code}: {str(res.message)}"
                        )

        run_async(_run_report())

        # Write to file if output_dir configured
        output_dir = self._get_output_dir()
        if output_dir:
            ext = output_format if output_format != "text" else "txt"
            self._write_output(tee.get_output(), "report", ext)

    def security(self, path) -> None:
        """Run security-focused scan (Bandit, etc.)."""
        abs_path = os.path.abspath(path)
        output_dir = self._get_output_dir()

        class TeeOutput:
            def __init__(self):
                self.lines = []

            def echo(self, msg=""):
                click.echo(msg)
                self.lines.append(str(msg))

            def get_output(self):
                return "\n".join(self.lines)

        tee = TeeOutput()

        async def _run_security() -> None:
            if self.container is None:
                raise RuntimeError("Container not initialized")
            container = self.container.get_for_path(abs_path)
            tee.echo(f" Running security scan on {abs_path}...")
            report_data: GovernanceReport = await container.run_analysis(
                FilePath(value=abs_path)
            )

            bandit_results = [
                r for r in report_data.results if str(r.source or "") == "bandit"
            ]
            if not bandit_results:
                tee.echo(" No security vulnerabilities found.")
            else:
                tee.echo(f" Found {len(bandit_results)} vulnerabilities.")
                for res in bandit_results:
                    tee.echo(
                        f" - {res.file.value}:{res.line.value} {res.code}: {str(res.message)} (Severity: {str(res.severity)})"
                    )

        run_async(_run_security())

        # Write to file if output_dir configured
        if output_dir:
            self._write_output(tee.get_output(), "security", "txt")


def register_report_commands(cli: Any, container: ServiceContainerAggregate) -> None:
    """Factory function for backward compatibility."""
    surface = ReportCommandsSurface(cli)
    surface.container = container
    surface.register_reports(container)


# Legacy exports removed as they break with click when wrapping bound methods.
