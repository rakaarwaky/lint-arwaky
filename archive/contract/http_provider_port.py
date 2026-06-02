"""http_provider_port — Port interface for HTTP operations."""

from abc import ABC, abstractmethod
from ..taxonomy import ResponseData, TransportUrlVO, Timeout, TransportError


class IHttpProviderPort(ABC):
    """Abstraction for HTTP operations."""

    @abstractmethod
    def get(
        self, url: TransportUrlVO, timeout: Timeout = Timeout(value=2000)
    ) -> ResponseData | TransportError:
        """Performs a GET request."""
        ...
