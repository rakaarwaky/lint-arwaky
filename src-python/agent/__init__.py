"""Auto-Linter Agent Layer.

This layer orchestrates capabilities and infrastructure.
It coordinates dependency injection, pipeline execution,
multi-project management, and lifecycle state.

Exports
-------
- Core orchestrators: Container, get_container, reset_container, PipelineExecutionOrchestrator, HookManagementOrchestrator, MultiProjectOrchestrator, get_state, State
"""

# Agent core
from .dependency_injection_container import Container
from .agent_container_registry import get_container, reset_container
from .pipeline_execution_orchestrator import PipelineExecutionOrchestrator
from .hook_management_orchestrator import HookManagementOrchestrator
from .multi_project_orchestrator import MultiProjectOrchestrator
from .lifecycle_state_manager import get_state, AgentState as State
from .arch_compliance_orchestrator import ArchitectureOrchestrator
from .agent_job_registry import JobRegistry
from .git_diff_manager import GitDiffResult

__all__ = [
    # Agent core
    "Container", "get_container", "reset_container", "PipelineExecutionOrchestrator",
    "HookManagementOrchestrator", "MultiProjectOrchestrator", "get_state", "State",
    "ArchitectureOrchestrator",
    "JobRegistry", "GitDiffResult",
]
