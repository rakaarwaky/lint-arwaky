"""Git-related CLI commands for auto-linter."""

from __future__ import annotations

import json
import logging
import os

import click

from ..taxonomy import FilePath, GitRef
from ..contract import ServiceContainerAggregate, GitCommandsAggregate, GitDiffResultAggregate


class GitCommandsSurface:
    """Surface for git-related CLI commands."""

    logger: object = None
    container: ServiceContainerAggregate | None = None

    def __init__(self):
        super().__init__()
        self.logger = logging.getLogger(__name__)

    def _print_section(self, title, items, item_fmt) -> None:
        """Print a section of items with a title (Helper)."""
        if items:
            click.echo(f"  {title} ({len(items)}):")
            for item in items:
                item_fmt(item)

    def _print_diff_text(self, diff: GitDiffResultAggregate, base_ref: GitRef) -> None:
        """Pretty-print diff summary in text format."""
        if not diff.all_files:
            click.echo(" No changed files detected.")
            return
        click.echo(f" Changed files since {base_ref}:")
        self._print_section(
            "Added", list(diff.added), lambda f: click.echo(f"    + {f}")
        )
        self._print_section(
            "Modified", list(diff.modified), lambda f: click.echo(f"    ~ {f}")
        )
        self._print_section(
            "Deleted", list(diff.deleted), lambda f: click.echo(f"    - {f}")
        )
        self._print_section(
            "Renamed",
            list(diff.renamed),
            lambda r: click.echo(f"    {r.old_path} -> {r.new_path}"),
        )
        click.echo(f"\n Lintable files: {len(diff.lintable_files)}")

    async def run_git_diff_check(
        self, container: ServiceContainerAggregate, path: FilePath, tee=None
    ):
        """Execute a lint check only on files changed in git."""
        orchestrator = container.get_aggregate(GitCommandsAggregate)
        await orchestrator.run_git_diff_check(container, path, tee)

    def register_all(
        self, container: ServiceContainerAggregate, cli: click.Group | None = None
    ) -> None:
        """Register git commands to the provided click group."""
        self.container = container
        target_cli = cli or getattr(self, "cli", None)
        if target_cli is None:
            return

        @target_cli.command("git-diff")
        @click.option(
            "--base", default="HEAD", help="Git ref to compare from (default: HEAD)"
        )
        @click.option(
            "--output-format", type=click.Choice(["text", "json"]), default="text"
        )
        def git_diff_command(base, output_format):
            """Show files changed since base ref (git diff awareness)."""
            base_ref = GitRef(value=base)
            if self.container is None:
                raise RuntimeError("Container not initialized")
            diff = self.container.git_commands.get_diff(FilePath(value=os.getcwd()))
            if diff is None:
                click.echo(" Not a git repository or git not available.")
                return

            if output_format == "json":
                click.echo(
                    json.dumps(
                        {
                            "added": list(diff.added),
                            "modified": list(diff.modified),
                            "deleted": list(diff.deleted),
                            "renamed": [
                                {"old": r.old_path, "new": r.new_path}
                                for r in diff.renamed
                            ],
                            "lintable_files": list(diff.lintable_files),
                            "all_files": list(diff.all_files),
                            "total_changed": diff.total_changed,
                        },
                        indent=2,
                    )
                )
            else:
                self._print_diff_text(diff, base_ref)


def register_git_commands(cli: click.Group, container: ServiceContainerAggregate) -> None:
    """Factory function for backward compatibility."""
    surface = GitCommandsSurface()
    surface.register_all(container, cli)


# Singleton for module-level access
