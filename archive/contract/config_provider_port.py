"""config_provider_port — Interface for configuration loading."""

from abc import ABC, abstractmethod

from ..taxonomy import ProjectConfig, FilePath  # mandatory


class IConfigProviderPort(ABC):
    """Port for loading project configuration from various sources."""

    @abstractmethod
    def load_project_config(self, path: FilePath | None = None) -> ProjectConfig:
        """Load the project configuration from the given path or auto-detect."""
        ...
