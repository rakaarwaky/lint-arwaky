"""
pipeline_extended_orchestrator — Orchestration for multi-project and watch modes (Agent Layer).
"""
from __future__ import annotations
from ..contract import (
    OrchestratorContainerAggregate,
    PipelineExtendedOrchestratorAggregate,
    PipelineOutputAggregate,
    MultiProjectAggregate as MultiProjectContractAggregate,
    DirectoryWatchAggregate as DirectoryWatchContractAggregate
)
from ..taxonomy import FilePath, JobId, SuccessStatus, BooleanVO, ResponseData, ErrorMessage

class PipelineExtendedOrchestrator(PipelineExtendedOrchestratorAggregate):
    """Handles extended pipeline operations like multi-project and watch."""

    def __init__(self, container: OrchestratorContainerAggregate, **data: object):
        super().__init__(container=container, **data)

    async def execute_multi_project(
        self,
        request,
        use_retry: bool | None = None,
        config_path: FilePath | None = None,
    ) -> PipelineOutputAggregate:
        """Orchestrate linting across multiple projects."""
        paths, use_retry_flag = self._normalize_multi_project_request(request, use_retry, config_path)
        return await self._run_multi_project_with_job(paths, use_retry_flag)

    def _normalize_multi_project_request(self, request, use_retry, config_path):
        """Normalize a multi-project request."""
        if isinstance(request, MultiProjectContractAggregate):
            paths = list(request.paths) if request.paths else []
            use_retry_flag = bool(request.use_retry) if request.use_retry else False
            cfg = None
        else:
            paths = [FilePath(value=str(p)) for p in request]
            use_retry_flag = bool(use_retry) if use_retry is not None else False
            cfg = config_path
        if cfg:
            config_paths = self.container.multi_project.load_config(FilePath(value=str(cfg)))
            if config_paths:
                paths.extend([FilePath(value=str(p)) for p in config_paths])
        return paths, use_retry_flag

    async def _run_multi_project_with_job(self, paths, use_retry_flag):
        """Run multi-project scan with job tracking."""
        job_id_str = await self.container.job_registry.create_job("multi_project")
        job_id = JobId(value=job_id_str)
        try:
            if use_retry_flag:
                results = await self.container.job_registry.run_with_retry(
                    lambda: self.container.multi_project.scan_all_projects(paths)
                )
            else:
                results = await self.container.multi_project.scan_all_projects(paths)
            data = results.to_dict()
            await self.container.job_registry.complete_job(job_id_str, data)
            pipeline_output_cls = self.container.get(PipelineOutputAggregate)
            return pipeline_output_cls(
                success=SuccessStatus(value=BooleanVO(value=True)),
                job_id=job_id,
                data=ResponseData(data=data),
            )
        except Exception as e:
            await self.container.job_registry.fail_job(job_id_str, str(e))
            pipeline_output_cls = self.container.get(PipelineOutputAggregate)
            return pipeline_output_cls(
                success=SuccessStatus(value=BooleanVO(value=False)),
                job_id=job_id,
                error=ErrorMessage(value=str(e)),
            )

    async def execute_watch(self, request: DirectoryWatchContractAggregate) -> PipelineOutputAggregate:
        """Orchestrate watching a directory for changes."""
        job_id_str = await self.container.job_registry.create_job("watch")
        job_id = JobId(value=job_id_str)
        try:
            data = await self.container.watch_orchestrator.execute(request)
            await self.container.job_registry.complete_job(job_id_str, data)
            pipeline_output_cls = self.container.get(PipelineOutputAggregate)
            return pipeline_output_cls(
                success=SuccessStatus(value=BooleanVO(value=True)),
                job_id=job_id,
                data=ResponseData(data=data)
            )
        except Exception as e:
            await self.container.job_registry.fail_job(job_id_str, str(e))
            pipeline_output_cls = self.container.get(PipelineOutputAggregate)
            return pipeline_output_cls(
                success=SuccessStatus(value=BooleanVO(value=False)),
                job_id=job_id,
                error=ErrorMessage(value=str(e))
            )