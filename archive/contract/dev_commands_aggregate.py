"""dev_commands_aggregate - Aggregate contract for dev commands."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from .service_container_aggregate import ServiceContainerAggregate
from ..taxonomy import FilePath, FileFormat, Identity, BooleanVO

class DevCommandsAggregate(BaseModel, ABC):
    """AGGREGATE: Domain contract for development-related surface commands."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    container: ServiceContainerAggregate

    @abstractmethod
    def diff(
        self,
        path1: FilePath | Identity,
        path2: FilePath | Identity,
        output_format: FileFormat | Identity,
    ) -> None:
        """Compare lint results between two versions."""
        ...

    @abstractmethod
    def suggest(self, path: FilePath | Identity, ai: BooleanVO) -> None:
        """AI-powered fix suggestions."""
        ...

    @abstractmethod
    def ignore(
        self, rule: Identity, remove: BooleanVO, path: FilePath | Identity
    ) -> None:
        """Manage ignore rules in configuration."""
        ...

    @abstractmethod
    def config(self, action: Identity, path: FilePath | Identity) -> None:
        """Edit configuration settings."""
        ...

    @abstractmethod
    def export(
        self, output_format: FileFormat | Identity, output: FilePath | Identity
    ) -> None:
        """Export lint reports in different formats."""
        ...

    @abstractmethod
    def init(self, path: FilePath | Identity) -> None:
        """Initialize a new Auto-Linter configuration."""
        ...

    @abstractmethod
    def install_hook(self, path: FilePath | Identity) -> None:
        """Install git pre-commit hook."""
        ...

    @abstractmethod
    def uninstall_hook(self, path: FilePath | Identity) -> None:
        """Remove git pre-commit hook."""
        ...
