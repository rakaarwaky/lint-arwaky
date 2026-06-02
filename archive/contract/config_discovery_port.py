"""config_discovery_protocol — Protocol for discovering configuration files."""

from abc import ABC, abstractmethod


from ..taxonomy import FilePath, DirectoryPath, ConfigError


class IConfigDiscoveryPort(ABC):
    """Port for discovering configuration files in the file system."""

    @abstractmethod
    def find_env_file(self, start: DirectoryPath | None = None) -> FilePath | ConfigError | None:
        """Walk up from start to find .env file."""
        ...

    @abstractmethod
    def find_yaml_config(self, start: DirectoryPath | None = None) -> FilePath | ConfigError | None:
        """Find config file: auto_linter.config.yaml or variants."""
        ...

    @abstractmethod
    def find_toml_config(self, start: DirectoryPath | None = None) -> FilePath | ConfigError | None:
        """Find pyproject.toml with [tool.auto_linter] section."""
        ...
