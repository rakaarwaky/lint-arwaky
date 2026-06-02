"""hook_manager_port — Port interface for Git hook management.

Infrastructure implements this. Capabilities consume it via DI.
"""

from abc import ABC, abstractmethod

from ..taxonomy import FilePath, SuccessStatus, GitHookError


class IHookManagerPort(ABC):
    """Port interface for Git hook lifecycle management."""

    @abstractmethod
    def install_pre_commit(self, executable_path: FilePath) -> SuccessStatus | GitHookError:
        """Install the pre-commit hook. Returns True (SuccessStatus) if successful."""
        ...

    @abstractmethod
    def uninstall_pre_commit(self) -> SuccessStatus | GitHookError:
        """Uninstall the pre-commit hook. Returns True (SuccessStatus) if successful."""
        ...
