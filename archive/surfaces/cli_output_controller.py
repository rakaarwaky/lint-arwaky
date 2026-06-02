"""CLI output management utilities."""

import sys
from contextlib import contextmanager
from typing import Iterator, Any
from ..taxonomy import FilePath, LogOutput, Identity, FileFormat
from ..contract import OutputClientAggregate, ServiceContainerAggregate
import click


class OutputControllerSurface:
    """Surface for CLI output management (Controller)."""

    container: ServiceContainerAggregate | None = None

    @property
    def _INTERFACE(self) -> object:
        """ARCHITECTURAL COMMITMENT: Required interface."""
        return ServiceContainerAggregate

    def get_output_dir(self) -> FilePath | None:
        """Get the effective output directory from CLI flag or config."""
        ctx = click.get_current_context(silent=True)
        cli_output_dir = ctx.obj.get("output_dir") if ctx and ctx.obj else None

        if cli_output_dir:
            return FilePath(value=cli_output_dir)

        if self.container:
            # Try to resolve via orchestrator if container available
            client = self.container.get(OutputClientAggregate)
            if client:
                return client.get_output_dir()

        return None

    def write_output(
        self, output: LogOutput, command: Identity, output_format: FileFormat | None = None
    ) -> FilePath | None:
        """Passive delegation to the Agent output client."""
        if not self.container:
            return None

        client = self.container.get(OutputClientAggregate)
        if not client:
            return None

        # Resolve output dir from CLI context if possible, otherwise let client handle it
        cli_dir = self.get_output_dir()
        old_dir = None
        if cli_dir and hasattr(client, "_default_output_dir"):
            old_dir = getattr(client, "_default_output_dir")
            setattr(client, "_default_output_dir", cli_dir)

        try:
            return client.write_output(output, command, output_format)
        finally:
            if cli_dir and hasattr(client, "_default_output_dir"):
                setattr(client, "_default_output_dir", old_dir)

    @contextmanager
    def tee_stdout(self) -> Iterator[Any]:
        """Passive delegation to the Agent output client."""
        if not self.container:
            yield sys.stdout
            return

        client = self.container.get(OutputClientAggregate)
        if not client:
            yield sys.stdout
            return

        with client.tee_stdout() as buffer:
            yield buffer


# Lazy singleton — created on first call to avoid import-time side effects
_Instance = None


def _get_instance():
    global _Instance
    if _Instance is None:
        _Instance = OutputControllerSurface()
    return _Instance


def get_output_dir(*args, **kwargs):
    return _get_instance().get_output_dir(*args, **kwargs)


def write_output(*args, **kwargs):
    return _get_instance().write_output(*args, **kwargs)


def tee_stdout(*args, **kwargs):
    return _get_instance().tee_stdout(*args, **kwargs)


def set_container(container: ServiceContainerAggregate) -> None:
    """Set the system-wide service container on the output controller singleton."""
    _get_instance().container = container

