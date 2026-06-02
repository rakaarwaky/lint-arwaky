"""transport_client_error — Transport domain error types."""

from pydantic import BaseModel, ConfigDict, Field

from .transport_protocol_vo import TransportProtocol, TransportEndpoint
from .error_value_vo import ErrorMessage


class TransportError(BaseModel):
    """Transport communication failed."""

    model_config = ConfigDict(frozen=True)

    protocol: TransportProtocol
    message: ErrorMessage
    endpoint: TransportEndpoint | None = None
    underlying_error: ErrorMessage = Field(
        default_factory=lambda: ErrorMessage(value="")
    )

    def __str__(self):
        ep = f" {self.endpoint}" if self.endpoint else ""
        return f"[{self.protocol.value}]{ep} {self.message}"
