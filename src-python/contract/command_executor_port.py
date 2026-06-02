"""contract — Port for executing external commands."""

from abc import ABC


from ..taxonomy import FilePath, ResponseData, Timeout, PatternList


class ICommandExecutorPort(ABC):
    """Port for executing shell commands and external processes."""

    def execute_command(
        self,
        command: PatternList,
        working_dir: FilePath = FilePath(value="."),
        timeout: Timeout | None = None,
    ) -> ResponseData:
        """Execute a command and return the response."""
        raise NotImplementedError

    def health_check(self) -> ResponseData:
        """Check the health of the execution transport."""
        raise NotImplementedError
