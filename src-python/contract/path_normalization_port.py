"""contract — Port for path normalization services."""

from abc import ABC, abstractmethod


from ..taxonomy import FilePath


class IPathNormalizationPort(ABC):
    @abstractmethod
    def normalize_path(self, path: FilePath) -> FilePath:
        """Normalize path: fix slashes, resolve phantom roots."""
        ...

    @abstractmethod
    def resolve_infrastructure_path(
        self, path: FilePath, context_path: FilePath | None = None
    ) -> FilePath:
        """Unified path resolution for infrastructure adapters."""
        ...
