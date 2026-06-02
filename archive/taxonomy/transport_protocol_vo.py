"""transport_protocol_vo — Transport value objects."""

from enum import Enum
from pydantic import BaseModel, ConfigDict


class TransportProtocol(str, Enum):
    """Transport protocol type."""

    HTTP = "HTTP"
    UNIX_SOCKET = "UnixSocket"
    STDAggregate = "Stdio"

    @property
    def needs_desktop_commander(self) -> bool:
        return self in (TransportProtocol.HTTP, TransportProtocol.UNIX_SOCKET)


class TransportEndpoint(BaseModel):
    """Transport endpoint descriptor."""

    model_config = ConfigDict(frozen=True)

    protocol: TransportProtocol
    address: str

    def __str__(self) -> str:
        return f"{self.protocol.value}:{self.address}"

    @classmethod
    def from_url(cls, url: str) -> "TransportEndpoint":
        """Auto-detect protocol from URL."""
        if url.startswith("http://") or url.startswith("https://"):
            return cls(protocol=TransportProtocol.HTTP, address=url)
        if url == "stdio":
            return cls(protocol=TransportProtocol.STDAggregate, address="stdio")
        if url.startswith("/") or url.startswith("."):
            return cls(protocol=TransportProtocol.UNIX_SOCKET, address=url)
        return cls(protocol=TransportProtocol.STDAggregate, address="stdio")

    @property
    def display_name(self) -> str:
        if self.protocol == TransportProtocol.HTTP:
            return f"HTTP({self.address})"
        if self.protocol == TransportProtocol.UNIX_SOCKET:
            return f"Socket({self.address})"
        return "Stdio(direct)"
