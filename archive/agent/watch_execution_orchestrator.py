"""Watch execution orchestrator — Agent responsibility for file watching.

Coordinates directory watching and event processing.
"""

from __future__ import annotations
import asyncio
import logging
from typing import Type

from ..contract import (
    DirectoryWatchAggregate,
    IJobRegistryPort,
    IWatchProviderPort,
    PipelineOutputAggregate,
    ServiceContainerAggregate,
    WatchExecutionOrchestratorAggregate,
)
from ..taxonomy import (
    BooleanVO,
    ErrorMessage,
    FilePath,
    JobId,
    ResponseData,
    SuccessStatus,
    WatchResult,
)

from ..contract import run_async

logger = logging.getLogger("agent.watch")


class DirectoryWatchRequest(DirectoryWatchAggregate):
    """Concrete implementation of the directory watch input contract."""

    @property
    def _INTERFACE(self) -> type[IWatchProviderPort]:
        return IWatchProviderPort


class WatchExecutionOrchestrator(WatchExecutionOrchestratorAggregate):
    """Orchestrator for managing file watch and live reload behaviors.
    Uses the agent-based observer pattern.
    """

    container: ServiceContainerAggregate | None = None

    def __init__(self, container: ServiceContainerAggregate):
        super().__init__(container=container)
        self.container = container

    @property
    def _INTERFACE_JOB_REGISTRY(self) -> Type[IJobRegistryPort]:
        return IJobRegistryPort

    def is_available(self) -> bool:
        """Check if the watchdog library is available for file watching."""
        try:
            import watchdog

            _ = watchdog  # mark as used
            return True
        except ImportError:
            return False

    async def execute_watch(self, request: DirectoryWatchAggregate) -> PipelineOutputAggregate:
        """Orchestrate watching a directory for changes and re-linting."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        job_id_str = await self.container.job_registry.create_job("watch")
        job_id = JobId(value=job_id_str)
        try:
            if not self.is_available():
                pipeline_output_cls = self.container.get(PipelineOutputAggregate)
                return pipeline_output_cls(
                    success=SuccessStatus(value=BooleanVO(value=False)),
                    job_id=job_id,
                    error=ErrorMessage(
                        value="'watchdog' is not installed. Run: pip install watchdog"
                    ),
                )

            watch_result = await self.execute(request)
            data_dict = watch_result.to_dict()
            if self.container is None:
                raise RuntimeError("Container not initialized")
            await self.container.job_registry.complete_job(job_id_str, data_dict)
            pipeline_output_cls = self.container.get(PipelineOutputAggregate)
            return pipeline_output_cls(
                success=SuccessStatus(value=BooleanVO(value=True)),
                job_id=job_id,
                data=ResponseData(value=data_dict),
            )
        except Exception as e:
            if self.container is None:
                raise RuntimeError("Container not initialized")
            await self.container.job_registry.fail_job(job_id_str, str(e))
            pipeline_output_cls = self.container.get(PipelineOutputAggregate)
            return pipeline_output_cls(
                success=SuccessStatus(value=BooleanVO(value=False)),
                job_id=job_id,
                error=ErrorMessage(value=str(e)),
            )

    async def execute(self, request: DirectoryWatchAggregate) -> WatchResult:
        """Initial execution for watch mode."""
        path = request.path
        if self.container is None:
            raise RuntimeError("Container not initialized")
        report = await self.container.analysis_orchestrator.run(path)
        return WatchResult(file=path, report=report)

    def process_event(self, file_path: FilePath) -> dict[str, object]:
        """Process a file change event."""
        # Accept str for surface convenience; convert to FilePath internally
        if isinstance(file_path, str):
            file_path = FilePath(value=file_path)
        if self.container is None:
            raise RuntimeError("Container not initialized")

        async def _run_analysis():
            report = await self.container.analysis_orchestrator.run(file_path)
            return report.model_dump()

        try:
            try:
                loop = asyncio.get_running_loop()
                fut = asyncio.run_coroutine_threadsafe(_run_analysis(), loop)
                data = fut.result(timeout=300)
            except RuntimeError:
                data = run_async(_run_analysis())

            return {
                "file": str(file_path),
                "score": data.get("score", 0.0),
                "is_passing": data.get("is_passing", False),
                "results": data,
            }
        except Exception as e:
            logger.error(f"Error processing watch event for {file_path}: {e}")
            return {
                "file": str(file_path),
                "error": str(e),
            }
