"""metrics_provider_port — Interface for technical metrics and history."""

from abc import ABC, abstractmethod

from ..taxonomy import FilePath, Count, ResponseDataList, MetricsError


class IMetricsProviderPort(ABC):
    """Port for retrieving code metrics and historical data."""

    @abstractmethod
    async def get_line_count(self, path: FilePath) -> Count | MetricsError:
        """Return the raw line count of a file."""
        ...

    @abstractmethod
    async def get_history(self) -> ResponseDataList | MetricsError:
        """Read the raw quality history log."""
        ...
