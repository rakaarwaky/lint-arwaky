"""config_validation_port — Interface for configuration validation and orchestration."""

from abc import ABC, abstractmethod
from ..taxonomy import FilePath, AppConfig, ConfigError


class IConfigValidatorPort(ABC):
    """Port for validating and orchestrating application configuration."""

    @abstractmethod
    def load_config(
        self,
        env_path: FilePath | None = None,
        yaml_path: FilePath | None = None,
    ) -> AppConfig | ConfigError:
        """Load or reload configuration. Returns AppConfig."""
        ...

    @abstractmethod
    def get_config(self) -> AppConfig | ConfigError:
        """Get the current configuration."""
        ...

    @abstractmethod
    def reset_config(self) -> None:
        """Reset the configuration."""
        ...
