from __future__ import annotations

import logging
import os
from pathlib import Path

from dotenv import load_dotenv
import yaml

from ..contract import (
    IConfigDiscoveryPort,
    IConfigParserPort,
    IConfigProviderPort,
    IConfigValidatorPort,
)
from ..taxonomy import (
    AppConfig,
    ConfigError,
    ConfigKey,
    ErrorMessage,
    FilePath,
    ProjectConfig,
)

logger = logging.getLogger("infrastructure.config")


class ConfigParserProvider(IConfigParserPort):
    """Provider for parsing configuration files in YAML format."""

    def parse_yaml_config(self, path: FilePath) -> ProjectConfig | ConfigError:
        """Parse configuration file into ProjectConfig."""
        path_str = str(path.value if hasattr(path, "value") else path)
        try:
            with open(path_str, encoding="utf-8") as f:
                content = f.read()
            raw = yaml.safe_load(content) or {}
            return ProjectConfig.model_validate(raw)
        except Exception as e:
            return ConfigError(
                key=ConfigKey(value="yaml.parse"),
                message=ErrorMessage(value=f"Failed to parse YAML config: {e}"),
                config_file=FilePath(value=path_str),
            )

    def parse_toml_config(self, path: FilePath) -> ProjectConfig | ConfigError | None:
        """TOML configuration is no longer supported per project guidelines."""
        return None


class ConfigValidationProvider(IConfigValidatorPort):
    """Main configuration orchestrator for the application."""

    def __init__(
        self,
        discovery: IConfigDiscoveryPort,
        parser: IConfigParserPort,
        json_provider: IConfigProviderPort,
    ):
        self._discovery = discovery
        self._parser = parser
        self._json_provider = json_provider
        self._config: AppConfig | None = None

    def load_config(
        self,
        env_path: FilePath | Path | str | None = None,
        yaml_path: FilePath | Path | str | None = None,
    ) -> AppConfig:
        """Load or reload configuration. Returns AppConfig."""
        # 1. Load .env
        env_str = (
            str(env_path.value if hasattr(env_path, "value") else env_path)
            if env_path
            else None
        )
        if env_str:
            load_dotenv(env_str, override=False)
        else:
            found_env = self._discovery.find_env_file()
            if found_env:
                load_dotenv(str(found_env), override=False)

        # 2. Load yaml config
        yaml_config: ProjectConfig | ConfigError | None = None
        yaml_str = (
            str(yaml_path.value if hasattr(yaml_path, "value") else yaml_path)
            if yaml_path
            else None
        )
        if yaml_str:
            yaml_config = self._parser.parse_yaml_config(FilePath(value=yaml_str))
        else:
            found_yaml = self._discovery.find_yaml_config()
            if isinstance(found_yaml, ConfigError):
                logger.error(f"Configuration discovery error: {found_yaml}")
                yaml_config = ProjectConfig.defaults()
            elif found_yaml:
                yaml_config = self._parser.parse_yaml_config(found_yaml)
            else:
                yaml_config = ProjectConfig.defaults()

        # Ensure we have a valid ProjectConfig
        if yaml_config is None or isinstance(yaml_config, ConfigError):
            if isinstance(yaml_config, ConfigError):
                logger.error(f"Failed to load project configuration: {yaml_config}")
            yaml_config = ProjectConfig.defaults()

        # 3. Build AppConfig
        self._config = AppConfig.create(
            phantom_root=os.environ.get("PHANTOM_ROOT", os.path.expanduser("~")),
            project_root=os.environ.get("PROJECT_ROOT", os.getcwd()),
            project=yaml_config,
        )
        return self._config

    def get_config(self) -> AppConfig:
        """Get the global config. Auto-loads on first call."""
        if self._config is None:
            self._config = self.load_config()
        return self._config

    def reset_config(self) -> None:
        """Reset internal state (for testing)."""
        self._config = None


class YAMLConfigProvider(IConfigProviderPort):
    """Legitimate implementation of IConfigProvider using the validator."""

    def __init__(self, validator: IConfigValidatorPort):
        self._validator = validator

    def load_project_config(self, path: FilePath | None = None) -> ProjectConfig:
        return self._validator.load_config(yaml_path=path).project


class ConfigYamlProvider(IConfigProviderPort):
    """Backward-compatible YAML provider."""

    def load_project_config(self, path: FilePath | None = None) -> ProjectConfig:
        path_str = (
            str(path.value if hasattr(path, "value") else path) if path else None
        )
        if path_str and Path(path_str).is_file():
            try:
                with open(path_str, encoding="utf-8") as f:
                    data = yaml.safe_load(f)
                    return (
                        ProjectConfig.model_validate(data)
                        if data
                        else ProjectConfig.defaults()
                    )
            except Exception:
                pass
        return ProjectConfig.defaults()


class ConfigJSONProvider(IConfigProviderPort):
    """Backward-compatible JSON stub returning defaults."""

    def load_project_config(self, path: FilePath | None = None) -> ProjectConfig:
        return ProjectConfig.defaults()
