from __future__ import annotations
import os
from typing import TYPE_CHECKING

from ..contract import OrchestratorContainerAggregate, ServiceContainerAggregate
from ..taxonomy import FilePath
from .analysis_execution_orchestrator import AnalysisOrchestrator
from .hook_management_orchestrator import HookManagementOrchestrator
from .lint_pipeline_orchestrator import LintPipelineOrchestrator
from .lint_fix_orchestrator import LintFixOrchestrator
from .pipeline_execution_orchestrator import PipelineExecutionOrchestrator
from .multi_project_orchestrator import MultiProjectOrchestrator
from .watch_execution_orchestrator import WatchExecutionOrchestrator
from .maintenance_commands_orchestrator import MaintenanceCommandsOrchestrator
from .git_commands_orchestrator import GitCommandsOrchestrator
from .output_client_orchestrator import OutputClientOrchestrator
from .pipeline_action_orchestrator import PipelineActionDispatcher
from .pipeline_extended_orchestrator import PipelineExtendedOrchestrator
from .check_commands_orchestrator import CheckCommandsOrchestrator
from .fix_commands_orchestrator import FixCommandsOrchestrator
from .setup_management_orchestrator import SetupManagementOrchestrator
from .watch_commands_orchestrator import WatchCommandsOrchestrator
from .plugin_commands_orchestrator import PluginCommandsOrchestrator
from .report_commands_orchestrator import ReportCommandsOrchestrator
from .dev_commands_orchestrator import DevCommandsOrchestrator

if TYPE_CHECKING:
    from .dependency_injection_container import Container


class OrchestratorMixinContainer(ServiceContainerAggregate, OrchestratorContainerAggregate):
    """Logic for initializing high-level agent orchestrators."""

    lint_pipeline: LintPipelineOrchestrator | None = None
    fix_orchestrator: LintFixOrchestrator | None = None
    analysis_orchestrator: AnalysisOrchestrator | None = None
    hook_capability: HookManagementOrchestrator | None = None
    pipeline: PipelineExecutionOrchestrator | None = None
    multi_project: MultiProjectOrchestrator | None = None
    watch_orchestrator: WatchExecutionOrchestrator | None = None
    pipeline_dispatcher: PipelineActionDispatcher | None = None
    pipeline_extended: PipelineExtendedOrchestrator | None = None

    maintenance_commands: MaintenanceCommandsOrchestrator | None = None
    git_commands: GitCommandsOrchestrator | None = None
    check_commands: CheckCommandsOrchestrator | None = None
    fix_commands: FixCommandsOrchestrator | None = None
    setup_commands: SetupManagementOrchestrator | None = None
    watch_commands: WatchCommandsOrchestrator | None = None
    plugin_commands: PluginCommandsOrchestrator | None = None
    report_commands: ReportCommandsOrchestrator | None = None
    dev_commands: DevCommandsOrchestrator | None = None
    output_client: OutputClientOrchestrator | None = None

    def _init_orchestrators(self: "Container") -> None:
        # Ensure taxonomy is used
        _base_path: FilePath = FilePath(value=str(os.getcwd()))

        # 1. Pipeline Orchestrators
        self.lint_pipeline = LintPipelineOrchestrator(self)
        self.fix_orchestrator = LintFixOrchestrator(self)

        # 2. Domain Orchestrators
        self.analysis_orchestrator = AnalysisOrchestrator(container=self)

        # 3. Git/Hook Management
        self.hook_capability = HookManagementOrchestrator(self.git_hook_manager)

        # 4. Execution Pipelines
        self.pipeline = PipelineExecutionOrchestrator(self)
        self.multi_project = MultiProjectOrchestrator(self)
        self.watch_orchestrator = WatchExecutionOrchestrator(self)
        self.pipeline_dispatcher = PipelineActionDispatcher(self)
        self.pipeline_extended = PipelineExtendedOrchestrator(self)

        # 5. Command Orchestrators (Command Logics)
        from .check_commands_orchestrator import CheckCommandsOrchestrator
        from .fix_commands_orchestrator import FixCommandsOrchestrator
        from .setup_management_orchestrator import SetupManagementOrchestrator
        from .watch_commands_orchestrator import WatchCommandsOrchestrator
        from .plugin_commands_orchestrator import PluginCommandsOrchestrator
        from .report_commands_orchestrator import ReportCommandsOrchestrator
        from .dev_commands_orchestrator import DevCommandsOrchestrator

        self.maintenance_commands = MaintenanceCommandsOrchestrator(self)
        self.git_commands = GitCommandsOrchestrator(self)
        self.check_commands = CheckCommandsOrchestrator(self)
        self.fix_commands = FixCommandsOrchestrator(self)
        self.setup_commands = SetupManagementOrchestrator(self)
        self.watch_commands = WatchCommandsOrchestrator(self)
        self.plugin_commands = PluginCommandsOrchestrator(self)
        self.report_commands = ReportCommandsOrchestrator(self)
        self.dev_commands = DevCommandsOrchestrator(self)
        self.output_client = OutputClientOrchestrator(self)
