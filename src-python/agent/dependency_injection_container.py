"""dependency_injection_container — Internal wiring engine for the project."""

from __future__ import annotations
import os
import logging
from pydantic import ConfigDict
from typing import Type, TypeVar

from ..contract import (
    ServiceContainerAggregate,
    ILinterAdapterPort,
    IFileSystemPort,
    AgentLifecycleAggregate,
    ProjectContainerAggregate,
    JobRegistryAggregate,
    GitDiffResultAggregate,
)
from ..taxonomy import FilePath, GovernanceReport, DirectoryPath, Identity, BooleanVO
from .git_diff_manager import GitDiffResult

# Import Mixins
from .infrastructure_mixin_container import InfrastructureMixinContainer
from .adapter_mixin_container import AdapterMixinContainer
from .capability_mixin_container import CapabilityMixinContainer
from .orchestrator_mixin_container import OrchestratorMixinContainer

# Internal dependencies
from .lifecycle_state_manager import get_state
from .agent_job_registry import JobRegistry

logger = logging.getLogger("auto_linter.agent")

T = TypeVar("T")


class Container(
    InfrastructureMixinContainer,
    AdapterMixinContainer,
    CapabilityMixinContainer,
    OrchestratorMixinContainer,
    ServiceContainerAggregate,
):
    """
    Dependency Injection container (Internal Engine).
    Focused strictly on service wiring and lifecycle.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True, extra="allow")

    project_root: DirectoryPath = DirectoryPath(value=".")
    status: Identity = Identity(value="initializing")
    started: BooleanVO = BooleanVO(value=False)

    def __init__(self, project_root: DirectoryPath | None = None) -> None:
        super().__init__()
        self.project_root = DirectoryPath(value=str(project_root or os.getcwd()))

        # Phase 1: Infrastructure (Foundation)
        self._init_infrastructure()

        # Phase 2: Lifecycle & Shared Registry
        self.state_manager = get_state(self)
        self.state_manager.mark_started()
        self._job_registry_port = self.memory_job_registry
        self._job_registry_aggregate = JobRegistry(port=self._job_registry_port)

        # Phase 3: Capabilities (Analyzers)
        self._init_capabilities()

        # Phase 4: Adapters (Infrastructure Implementation)
        self._init_adapters()

        # Phase 5: Orchestrators (Agent Brain)
        self._init_orchestrators()

    @property
    def job_registry(self) -> JobRegistryAggregate:
        return self._job_registry_aggregate

    # === ServiceContainerAggregate IMPLEMENTATION (Identity & Registry) ===

    def get_for_path(self, path: FilePath) -> "ServiceContainerAggregate":
        """Get or create a container instance for a specific path."""
        from .agent_container_registry import get_container

        return get_container(path)

    def get(self, interface: Type[T]) -> T | None:
        """Internal service discovery."""
        from ..contract import (
            LintPipelineOrchestratorAggregate,
            IJobRegistryPort,
            IArchComplianceProtocol,
            CheckCommandsAggregate,
            FixCommandsAggregate,
            MaintenanceCommandsAggregate,
            SetupManagementAggregate,
            PluginCommandsAggregate,
            ReportCommandsAggregate,
            DevCommandsAggregate,
            WatchCommandsAggregate,
            GitCommandsAggregate,
            OutputClientAggregate,
            MultiProjectOrchestratorAggregate,
            PipelineActionDispatcherAggregate,
            PipelineExtendedOrchestratorAggregate,
            PipelineInputAggregate,
            PipelineOutputAggregate,
            MultiProjectAggregate,
            DirectoryWatchAggregate,
            ContainerRegistryAggregate,
            InfrastructureContainerAggregate,
            CapabilityContainerAggregate,
            AdapterContainerAggregate,
            OrchestratorContainerAggregate,
            AnalysisOrchestratorAggregate,
            ArchCoordinatorAggregate,
            LintFixOrchestratorAggregate,
            PipelineExecutionOrchestratorAggregate,
            WatchExecutionOrchestratorAggregate,
            HookManagementOrchestratorAggregate,
            ILintReportFormatterProtocol,
        )
        from .pipeline_execution_orchestrator import PipelineInput, PipelineOutput
        from .multi_project_orchestrator import MultiProjectRequest
        from .watch_execution_orchestrator import DirectoryWatchRequest
        from .agent_container_registry import AgentContainerRegistry

        mapping = {
            ServiceContainerAggregate: self,
            ILinterAdapterPort: self.adapters,
            IJobRegistryPort: self._job_registry_port,
            JobRegistryAggregate: self._job_registry_aggregate,
            IArchComplianceProtocol: self.analysis_orchestrator,
            IFileSystemPort: self.fs_scanner,
            AgentLifecycleAggregate: self.state_manager,
            ProjectContainerAggregate: ProjectAggregateFacade(self),
            LintPipelineOrchestratorAggregate: self.lint_pipeline,
            CheckCommandsAggregate: self.check_commands,
            FixCommandsAggregate: self.fix_commands,
            MaintenanceCommandsAggregate: self.maintenance_commands,
            SetupManagementAggregate: self.setup_commands,
            PluginCommandsAggregate: self.plugin_commands,
            ReportCommandsAggregate: self.report_commands,
            DevCommandsAggregate: self.dev_commands,
            WatchCommandsAggregate: self.watch_commands,
            GitCommandsAggregate: self.git_commands,
            OutputClientAggregate: self.output_client,
            MultiProjectOrchestratorAggregate: self.multi_project,
            PipelineActionDispatcherAggregate: self.pipeline_dispatcher,
            PipelineExtendedOrchestratorAggregate: self.pipeline_extended,
            GitDiffResultAggregate: GitDiffResult,
            PipelineInputAggregate: PipelineInput,
            PipelineOutputAggregate: PipelineOutput,
            MultiProjectAggregate: MultiProjectRequest,
            DirectoryWatchAggregate: DirectoryWatchRequest,
            ContainerRegistryAggregate: AgentContainerRegistry,
            InfrastructureContainerAggregate: self,
            CapabilityContainerAggregate: self,
            AdapterContainerAggregate: self,
            OrchestratorContainerAggregate: self,
            AnalysisOrchestratorAggregate: self.analysis_orchestrator,
            ArchCoordinatorAggregate: self.arch_coordinator
            if hasattr(self, "arch_coordinator")
            else None,
            LintFixOrchestratorAggregate: self.fix_orchestrator,
            PipelineExecutionOrchestratorAggregate: self.pipeline,
            WatchExecutionOrchestratorAggregate: self.watch_orchestrator,
            HookManagementOrchestratorAggregate: self.hook_capability,
            ILintReportFormatterProtocol: self.report_formatter,
        }
        return mapping.get(interface)

    def get_aggregate(self, aggregate_contract: Type[T]) -> T | None:
        """Resolve high-level aggregate contracts via the internal get mechanism."""
        return self.get(aggregate_contract)

    def shutdown(self) -> None:
        """Graceful shutdown."""
        self.state_manager.mark_stopped()


# Resolve forward references for Pydantic v2
Container.model_rebuild()


class ProjectAggregateFacade(ProjectContainerAggregate, ServiceContainerAggregate):
    """
    Pure Facade for the Surface layer.
    Wraps the internal Container and only exposes authorized 'Buttons'.
    """

    def __init__(self, container: Container) -> None:
        self._container = container

    @property
    def job_registry(self) -> JobRegistryAggregate:
        """ARCHITECTURAL COMMITMENT: Provide access to the central job tracking registry."""
        return self._container.job_registry

    async def run_analysis(self, path: FilePath) -> GovernanceReport:
        """Button: Proxy to internal orchestrator."""
        orchestrator = self._container.analysis_orchestrator
        if orchestrator is None:
            raise RuntimeError("Analysis orchestrator not initialized")
        return await orchestrator.run(path)

    async def get_health(self):
        """Button: Proxy to internal state manager."""
        return await self._container.state_manager.get_health()

    def get_aggregate(self, aggregate_contract: Type[T]) -> T | None:
        """Button: Proxy to container's aggregate resolver."""
        return self._container.get_aggregate(aggregate_contract)

    def get(self, interface: Type[T]) -> T | None:
        """Button: Proxy to internal service discovery."""
        return self._container.get(interface)

    def get_for_path(self, path: FilePath) -> ServiceContainerAggregate:
        """Get or create a container instance for a specific path."""
        return self._container.get_for_path(path)

    def __getattr__(self, name: str) -> object:
        """Proxy to internal container for authorized capabilities/orchestrators."""
        return getattr(self._container, name)
