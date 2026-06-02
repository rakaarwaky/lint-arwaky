from ..taxonomy import FilePath, BooleanVO, WatchServiceError, ErrorMessage
from ..contract import IWatchProviderPort

import logging
import os


logger = logging.getLogger("infrastructure.watch")

try:
    from watchdog.events import FileSystemEventHandler
    from watchdog.observers import Observer

    HAS_WATCHDOG = True
except ImportError:
    HAS_WATCHDOG = False


class WatchServiceProvider(IWatchProviderPort):
    """Provides file system watching capabilities."""

    def __init__(self, event_callback):
        self.event_callback = event_callback
        self.observer = None

    def is_available(self) -> BooleanVO:
        return BooleanVO(value=HAS_WATCHDOG)

    def start(self, path: FilePath) -> WatchServiceError | None:
        """Start watching the specified path."""
        if not HAS_WATCHDOG:
            return WatchServiceError(
                path=path,
                message=ErrorMessage(value="watchdog library is required for watching. Please install it using pip.")
            )

        if self.observer:
            self.stop()

        class _Handler(FileSystemEventHandler):
            def __init__(self, callback):
                self.callback = callback

            def on_modified(self, event):
                if event.is_directory or not event.src_path.endswith(
                    (".py", ".js", ".ts")
                ):
                    return
                self.callback(FilePath(value=event.src_path))

        try:
            self.observer = Observer()
            watch_path = os.path.normpath(str(path.value))
            self.observer.schedule(
                _Handler(self.event_callback), watch_path, recursive=True
            )
            self.observer.start()
            return None
        except Exception as e:
            logger.error(f"Failed to start watching path {path}: {e}", exc_info=True)
            self.observer = None
            return WatchServiceError(
                path=path,
                message=ErrorMessage(value=f"Failed to start watch service: {e}")
            )

    def stop(self) -> WatchServiceError | None:
        """Stop watching."""
        if self.observer:
            try:
                self.observer.stop()
                self.observer.join()
                self.observer = None
                return None
            except Exception as e:
                logger.error(f"Failed to stop watch service: {e}", exc_info=True)
                self.observer = None
                return WatchServiceError(
                    message=ErrorMessage(value=f"Failed to stop watch service gracefully: {e}")
                )
        return None
