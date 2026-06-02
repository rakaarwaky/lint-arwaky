"""Orchestrates git hook management (Capability)."""

from ..taxonomy import AdapterName, FilePath, Identity, SuccessStatus

from ..contract import IHookManagerPort, HookManagementOrchestratorAggregate
from typing import Type


class HookManagementOrchestrator(HookManagementOrchestratorAggregate):
    def __init__(self, manager: IHookManagerPort):
        super().__init__(manager=manager)

    @property
    def _INTERFACE_HOOK_MANAGER(self) -> Type[IHookManagerPort]:
        return IHookManagerPort

    def get_hook_manager_identity(self) -> Identity:
        return Identity(value="git_hook_manager")

    def install(
        self, executable: AdapterName = AdapterName(value="auto-lint")
    ) -> SuccessStatus:
        return self.manager.install_pre_commit(FilePath(value=str(executable)))

    def uninstall(self) -> SuccessStatus:
        return self.manager.uninstall_pre_commit()
