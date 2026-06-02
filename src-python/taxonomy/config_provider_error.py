"""config_provider_error — Configuration domain error types."""

from pydantic import BaseModel, ConfigDict, Field

from .config_identifier_vo import ConfigKey
from .file_path_vo import FilePath
from .error_value_vo import ErrorMessage, ExpectedValue, ActualValue


class ConfigError(BaseModel):
    """Invalid or missing configuration."""

    model_config = ConfigDict(frozen=True)

    key: ConfigKey
    message: ErrorMessage
    expected: ExpectedValue = Field(default_factory=lambda: ExpectedValue(value=""))
    actual: ActualValue = Field(default_factory=lambda: ActualValue(value=""))
    config_file: FilePath | None = None

    def __str__(self):
        file_info = f" in {self.config_file}" if self.config_file else ""
        return f"Config error on '{self.key}'{file_info}: {self.message}"
