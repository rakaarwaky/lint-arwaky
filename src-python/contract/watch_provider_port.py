"""watch_provider_port — Port interface for file system watching."""

from abc import ABC, abstractmethod

from ..taxonomy import FilePath, BooleanVO, WatchServiceError


class IWatchProviderPort(ABC):
    """Port interface for file system watching."""

    @abstractmethod
    def start(self, path: FilePath) -> WatchServiceError | None:
        """Start watching the specified path."""
        ...

    @abstractmethod
    def stop(self) -> WatchServiceError | None:
        """Stop watching."""
        ...

    @abstractmethod
    def is_available(self) -> BooleanVO:
        """Check if watching is available."""
        ...
