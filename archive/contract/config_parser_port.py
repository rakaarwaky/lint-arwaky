"""config_parser_protocol — Protocol for parsing configuration files."""

from abc import ABC, abstractmethod


from ..taxonomy import ProjectConfig, FilePath, ConfigError


class IConfigParserPort(ABC):
    """Port for parsing various configuration formats (YAML, TOML, Jinja2)."""

    @abstractmethod
    def parse_yaml_config(self, path: FilePath) -> ProjectConfig | ConfigError:
        """Parse config file into ProjectConfig. Supports Jinja2 templates."""
        ...

    @abstractmethod
    def parse_toml_config(self, path: FilePath) -> ProjectConfig | ConfigError | None:
        """Parse TOML config."""
        ...
