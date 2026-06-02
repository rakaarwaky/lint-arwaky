"""Watch CLI command - file watcher with auto-lint on changes."""

import asyncio
import click
import os

from typing import Any
from ..taxonomy import WatchResult
from ..contract import ServiceContainerAggregate


class WatchdogBridge:
    """Bridges watchdog events to the WatchExecutionOrchestrator."""

    def __init__(self, container, loop):
        from watchdog.events import FileSystemEventHandler

        class _Handler(FileSystemEventHandler):
            def on_modified(self, event):
                if event.is_directory or not event.src_path.endswith(
                    (".py", ".js", ".ts")
                ):
                    return
                click.echo(f"Re-checking {event.src_path}...")
                # Delegate to orchestrator
                container.watch_orchestrator.process_event(event.src_path)

        self.handler = _Handler()


class WatchCommandsSurface:
    """Surface for directory watching CLI commands."""

    cli: Any = None
    container: ServiceContainerAggregate | None = None

    def __init__(self, cli, container: ServiceContainerAggregate | None = None):
        self.cli = cli
        self.container = container

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Register all watch commands."""
        self.container = container

        @self.cli.command("watch")
        @click.argument("path", type=click.Path(exists=True))
        def watch_cmd(path):
            """Watch for file changes and run linters automatically."""
            self.watch(path)

    @click.argument("path", type=click.Path(exists=True))
    def watch(self, path) -> None:
        """Watch for file changes and run linters automatically."""
        try:
            from watchdog.observers import Observer
        except ImportError:
            click.echo("Error: 'watchdog' is not installed. Run: pip install watchdog")
            return

        # Use injected container if it matches Protocol
        if not self.container:
            raise RuntimeError("WatchCommandsSurface not initialized with container")

        container = self.container.get_for_path(path)

        abs_path = os.path.abspath(path)
        click.echo(f" Watching {abs_path} for changes...")

        # Initial run
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)

        click.echo("Performing initial scan...")
        initial_result: WatchResult = loop.run_until_complete(
            container.watch_orchestrator.execute(abs_path)
        )
        click.echo(
            f"Initial scan complete. Score: {initial_result.report.score.value:.1f}"
        )

        # Setup watcher — wrapped in try/finally to guarantee cleanup
        observer = None
        try:
            bridge = WatchdogBridge(container, loop)
            observer = Observer()
            observer.schedule(bridge.handler, abs_path, recursive=True)
            observer.start()
            loop.run_forever()
        except KeyboardInterrupt:
            click.echo("\nStopping watch mode...")
        except Exception as e:
            click.echo(f"\nWatch error: {e}")
        finally:
            if observer is not None:
                observer.stop()
                observer.join()
            loop.close()


def register_watch_command(cli: Any, container: ServiceContainerAggregate) -> None:
    """Factory function for backward compatibility."""
    surface = WatchCommandsSurface(cli, container)
    surface.register_all(container)
