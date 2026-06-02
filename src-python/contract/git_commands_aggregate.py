"""git_commands_aggregate - Aggregate contract for git commands."""
from abc import ABC, abstractmethod
from pydantic import BaseModel, ConfigDict
from .service_container_aggregate import ServiceContainerAggregate
from .diff_result_aggregate import GitDiffResultAggregate
from ..taxonomy import FilePath

class GitCommandsAggregate(BaseModel, ABC):
    """AGGREGATE: Domain contract for git-related surface commands."""
    model_config = ConfigDict(arbitrary_types_allowed=True)

    @abstractmethod
    async def run_git_diff_check(
        self, container: ServiceContainerAggregate, path: FilePath, tee=None
    ) -> None:
        """Execute a lint check only on files changed in git."""
        ...

    @abstractmethod
    async def get_diff(self, path: FilePath) -> GitDiffResultAggregate:
        """Return aggregated git diff results."""
        ...
