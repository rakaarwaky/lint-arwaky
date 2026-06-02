"""plugin_manager_error — Plugin management domain error types."""

from pydantic import BaseModel, ConfigDict

from .error_code_vo import ErrorCode
from .error_value_vo import (
    ErrorMessage,
    Cause,
)
from .adapter_name_vo import AdapterName


class PluginError(BaseModel):
    """General failure during plugin discovery or registration."""

    model_config = ConfigDict(frozen=True)

    message: ErrorMessage
    error_code: ErrorCode | None = None
    cause: Cause | None = None

    def __str__(self):
        code = f" [{self.error_code}]" if self.error_code else ""
        return f"Plugin Error{code}: {self.message}"


class DiscoveryError(PluginError):
    """Failed to discover plugins via entry points."""
    pass


class RegistrationError(PluginError):
    """Failed to register a specific adapter."""
    adapter_name: AdapterName | None = None

    def __str__(self):
        target = f" for '{self.adapter_name}'" if self.adapter_name else ""
        return f"Registration Error{target}: {self.message}"
