"""Agent pipeline — receive→think→act→respond orchestrator."""

from __future__ import annotations
from ..taxonomy import (
    ActionArgs,
    BooleanVO,
    ErrorMessage,
    FilePath,
    JobId,
    ResponseData,
    SuccessStatus,
    Suggestion,
    VALID_PIPELINE_ACTIONS,
)

import logging
from pathlib import Path
from typing import Type, cast

from ..contract import (
    IArchComplianceProtocol,
    IJobRegistryPort,
    PipelineInputAggregate,
    PipelineOutputAggregate,
    MultiProjectAggregate,
    DirectoryWatchAggregate,
    PipelineExecutionOrchestratorAggregate,
    JobRegistryAggregate,
    ServiceContainerAggregate,
)

logger = logging.getLogger("agent.pipeline")


class PipelineInput(PipelineInputAggregate):
    """Concrete implementation of the pipeline input contract."""

    @property
    def _INTERFACE_REGISTRY(self) -> type[IJobRegistryPort]:
        from ..contract import IJobRegistryPort

        return IJobRegistryPort

    @property
    def _INTERFACE_COMPLIANCE(self) -> type[IArchComplianceProtocol]:
        from ..contract import IArchComplianceProtocol

        return IArchComplianceProtocol


class PipelineOutput(PipelineOutputAggregate):
    """Concrete implementation of the pipeline output contract."""

    @property
    def _INTERFACE_REGISTRY(self) -> type[IJobRegistryPort]:
        from ..contract import IJobRegistryPort

        return IJobRegistryPort

    @property
    def _INTERFACE_COMPLIANCE(self) -> type[IArchComplianceProtocol]:
        from ..contract import IArchComplianceProtocol

        return IArchComplianceProtocol


class PipelineExecutionOrchestrator(PipelineExecutionOrchestratorAggregate):
    """Orchestrates request → thinking → action → response.

    The brain stem of the agent:
    1. Receive — validate input, create job
    2. Think — decide which action to take
    3. Act — execute via capabilities/infrastructure
    4. Respond — format and return result
    """

    def __init__(self, container: ServiceContainerAggregate):
        super().__init__(container=container)

    async def _create_job_and_execute(
        self,
        action: str,
        execute_fn,
        *args,
        **kwargs,
    ) -> PipelineOutputAggregate:
        """Create a job and execute the given function with error handling."""
        job_id = await self.container.job_registry.create_job(action)
        try:
            data = await execute_fn(*args, **kwargs)
            await self.container.job_registry.complete_job(job_id, data)
            return PipelineOutput(
                success=SuccessStatus(value=BooleanVO(value=True)),
                job_id=job_id,
                data=ResponseData(value=data),
            )
        except Exception as e:
            logger.warning("Job creation/execution failed", exc_info=True)
            await self.container.job_registry.fail_job(job_id, str(e))
            return PipelineOutput(
                success=SuccessStatus(value=BooleanVO(value=False)),
                job_id=job_id,
                error=ErrorMessage(value=str(e)),
            )

    def _format_error_response(
        self,
        job_id: JobId,
        error: Exception,
    ) -> PipelineOutputAggregate:
        """Format a standardized error response."""
        return PipelineOutput(
            success=SuccessStatus(value=BooleanVO(value=False)),
            job_id=job_id,
            error=ErrorMessage(value=str(error)),
        )

    def _format_success_response(
        self,
        job_id: JobId,
        data: object,
    ) -> PipelineOutputAggregate:
        """Format a standardized success response."""
        return PipelineOutput(
            success=SuccessStatus(value=BooleanVO(value=True)),
            job_id=job_id,
            data=ResponseData(value=data),
        )

    @property
    def _INTERFACE_JOB_REGISTRY(self) -> Type[JobRegistryAggregate]:
        return JobRegistryAggregate

    async def execute(
        self,
        request: PipelineInputAggregate,
    ) -> PipelineOutputAggregate:
        """Full pipeline execution: receive → think → act → respond."""
        action = str(request.action)

        # 1. Receive — create job
        job_id = await self.container.job_registry.create_job(action)

        try:
            # 2. Think — validate and decide
            validation = await self._stage_validate(action, job_id)
            if validation is not None:
                return validation

            # 3. Act — execute
            result = await self._stage_execute(request, action)

            # 4. Respond — format and complete
            return await self._stage_format_response(job_id, result)

        except Exception as e:
            logger.warning("Pipeline execute failed", exc_info=True)
            await self.container.job_registry.fail_job(job_id, str(e))
            return PipelineOutput(
                success=SuccessStatus(value=BooleanVO(value=False)),
                job_id=job_id,
                error=ErrorMessage(value=str(e)),
            )

    async def _stage_validate(
        self,
        action: str,
        job_id: JobId,
    ) -> PipelineOutputAggregate | None:
        """Stage 2 — think: validate action, return error response or None."""
        if not self._validate_action(action):
            return PipelineOutput(
                success=SuccessStatus(value=BooleanVO(value=False)),
                job_id=job_id,
                error=ErrorMessage(value=f"Invalid action '{action}'"),
                suggestion=Suggestion(value="Use list_commands() for catalog"),
            )
        return None

    async def _stage_execute(
        self,
        request: PipelineInputAggregate,
        action: str,
    ) -> object:
        """Stage 3 — act: prepare args and dispatch execution."""
        args_vo = request.args or ActionArgs()
        args = dict(args_vo.value)
        if request.path:
            args["path"] = str(request.path)

        if request.use_retry and bool(request.use_retry):
            result_vo = await self.container.job_registry.run_with_retry(
                lambda: self._dispatch(action, args),
            )
        else:
            result_vo = await self._dispatch(action, args)

        return result_vo.value if hasattr(result_vo, "value") else result_vo

    async def _stage_format_response(
        self,
        job_id: JobId,
        result: object,
    ) -> PipelineOutputAggregate:
        """Stage 4 — respond: complete job and format output."""
        await self.container.job_registry.complete_job(job_id, result)

        has_error = "error" in result if isinstance(result, dict) else False
        error_msg = cast(dict, result).get("error") if has_error else None

        return PipelineOutput(
            success=SuccessStatus(value=BooleanVO(value=not has_error)),
            job_id=job_id,
            data=ResponseData(value=result) if not has_error else None,
            error=ErrorMessage(value=str(error_msg)) if has_error else None,
        )

    async def execute_check(self, path: FilePath) -> PipelineOutputAggregate:
        """Direct lint check — optimized pipeline path."""
        job_id = await self.container.job_registry.create_job("check")
        try:
            report = await self.container.analysis_orchestrator.run(path)
            data = self.container.to_dict(report)
            await self.container.job_registry.complete_job(job_id, data)
            return PipelineOutput(
                success=SuccessStatus(value=BooleanVO(value=True)),
                job_id=job_id,
                data=ResponseData(value=data),
            )
        except Exception as e:
            logger.warning("execute_check failed", exc_info=True)
            await self.container.job_registry.fail_job(job_id, str(e))
            return PipelineOutput(
                success=SuccessStatus(value=BooleanVO(value=False)),
                job_id=job_id,
                error=ErrorMessage(value=str(e)),
            )

    async def _dispatch(self, action: str, args: dict) -> ResponseData:
        """Dispatch action — delegates to PipelineActionDispatcher (single source of truth).

        Previously this method duplicated the same handler logic found in
        PipelineActionDispatcher. Now it converts the raw dict args into
        taxonomy VOs and delegates, ensuring any new handler added to
        PipelineActionDispatcher is automatically available here.
        """
        from ..taxonomy import ContentString, MetadataVO

        action_vo = ContentString(value=action)
        args_vo = MetadataVO(data=args)
        result = await self.container.pipeline_dispatcher.dispatch(action_vo, args_vo)
        # Normalise: PipelineActionDispatcher returns ResponseData with .data,
        # but callers of _dispatch expect ResponseData with .value
        if hasattr(result, "data") and not hasattr(result, "value"):
            return ResponseData(value=result.data)
        return result

    def _validate_action(self, action: str) -> bool:
        """Check if action is known — uses canonical VALID_PIPELINE_ACTIONS."""
        return action in VALID_PIPELINE_ACTIONS

    @staticmethod
    def _error_response(msg: str, **extra) -> dict:
        response = {"error": msg}
        response.update(extra)
        return response

    # === MULTI-PROJECT ORCHESTRATION ===

    async def execute_multi_project(
        self,
        request,
        use_retry: bool | None = None,
        config_path: FilePath | None = None,
    ) -> PipelineOutputAggregate:
        """Orchestrate linting across multiple projects.

        Supports both MultiProjectAggregate and legacy test API.
        """
        # 1. Normalize request
        paths, use_retry_flag, job_id = await self._stage_normalize_multi_request(
            request,
            use_retry,
            config_path,
        )

        try:
            # 2. Execute scan
            data = await self._stage_execute_multi_scan(paths, use_retry_flag)

            # 3. Respond
            await self.container.job_registry.complete_job(job_id.value, data)
            return self._format_success_response(job_id, data)

        except Exception as e:
            logger.warning("execute_multi_project failed", exc_info=True)
            await self.container.job_registry.fail_job(job_id.value, str(e))
            return self._format_error_response(job_id, e)

    async def _stage_normalize_multi_request(
        self,
        request,
        use_retry: bool | None,
        config_path: FilePath | None,
    ) -> tuple[list[FilePath], bool, JobId]:
        """Stage 1 — normalize: normalise input into (paths, retry_flag, job_id)."""
        if isinstance(request, MultiProjectAggregate):
            paths = list(request.paths) if request.paths else []
            use_retry_flag = bool(request.use_retry) if request.use_retry else False
            cfg = None
        else:
            paths = [FilePath(value=p) for p in request]
            use_retry_flag = bool(use_retry) if use_retry is not None else False
            cfg = config_path

        if cfg:
            config_paths = self.container.multi_project.load_config(Path(str(cfg)))
            if config_paths:
                paths.extend([FilePath(value=p) for p in config_paths])

        job_id_str = await self.container.job_registry.create_job("multi_project")
        return paths, use_retry_flag, JobId(value=job_id_str)

    async def _stage_execute_multi_scan(
        self,
        paths: list[FilePath],
        use_retry_flag: bool,
    ) -> dict[str, object]:
        """Stage 2 — execute: run scan across all projects."""
        if use_retry_flag:
            results = await self.container.job_registry.run_with_retry(
                lambda: self.container.multi_project.scan_all_projects(paths),
            )
        else:
            results = await self.container.multi_project.scan_all_projects(paths)

        return results.to_dict()

    # === WATCH ORCHESTRATION ===

    async def execute_watch(self, request: DirectoryWatchAggregate) -> PipelineOutput:
        """Orchestrate watching a directory for changes and re-linting."""
        job_id_str = await self.container.job_registry.create_job("watch")
        job_id = JobId(value=job_id_str)
        try:
            data = await self.container.watch_orchestrator.execute(request)
            await self.container.job_registry.complete_job(job_id_str, data)
            return PipelineOutput(
                success=SuccessStatus(value=BooleanVO(value=True)),
                job_id=job_id,
                data=ResponseData(value=data),
            )
        except Exception as e:
            logger.warning("execute_watch failed", exc_info=True)
            await self.container.job_registry.fail_job(job_id_str, str(e))
            return PipelineOutput(
                success=SuccessStatus(value=BooleanVO(value=False)),
                job_id=job_id,
                error=ErrorMessage(value=str(e)),
            )

    def process_watch_event(self, file_path: FilePath) -> dict[str, object]:
        """Process a file change event in watch mode."""
        try:
            return self.container.watch_orchestrator.process_event(file_path)
        except Exception as e:
            logger.warning(
                "process_watch_event failed for %s", file_path, exc_info=True
            )
            return {"file": str(file_path), "error": str(e)}


# Backwards compatibility: tests expect `Pipeline`
Pipeline = PipelineExecutionOrchestrator