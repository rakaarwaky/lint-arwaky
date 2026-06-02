"""
pipeline_action_orchestrator — Logic for dispatching pipeline actions (Agent Layer).
"""

from __future__ import annotations
from typing import Mapping, Callable
from ..contract import OrchestratorContainerAggregate, PipelineActionDispatcherAggregate
from ..taxonomy import (
    ContentString,
    MetadataVO,
    FilePath,
    BooleanVO,
    ResponseData,
    VALID_PIPELINE_ACTIONS,
)


class PipelineActionDispatcher(PipelineActionDispatcherAggregate):
    """Dispatches actions to the appropriate orchestrators or capabilities."""

    def __init__(self, container: OrchestratorContainerAggregate, **data: object) -> None:
        super().__init__(container=container, **data)

    async def dispatch(self, action: ContentString, args: MetadataVO) -> ResponseData:
        """Dispatch action to the appropriate use case or tool via handler map."""
        action_str = str(action.value) if hasattr(action, "value") else str(action)

        handler_map: Mapping[str, Callable[[str, MetadataVO], ResponseData]] = {
            "check": self._handle_check,
            "scan": self._handle_check,
            "security": self._handle_security,
            "complexity": self._handle_complexity,
            "duplicates": self._handle_duplicates,
            "trends": self._handle_trends,
            "fix": self._handle_fix,
            "report": self._handle_report,
            "version": self._handle_version,
            "adapters": self._handle_adapters,
            "install-hook": self._handle_install_hook,
            "install_hook": self._handle_install_hook,
            "uninstall-hook": self._handle_uninstall_hook,
            "uninstall_hook": self._handle_uninstall_hook,
            "batch": self._handle_batch,
            "multi_project": self._handle_multi_project,
            "doctor": self._handle_doctor,
            "cancel": self._handle_cancel,
        }

        handler = handler_map.get(action_str)
        if handler is not None:
            return await handler(action_str, args)

        return ResponseData(
            data={"error": f"No pipeline handler for action: {action_str}"}
        )

    # ------------------------------------------------------------------
    # Handler methods (one per action)
    # ------------------------------------------------------------------

    @staticmethod
    def _extract_path(args: MetadataVO) -> FilePath:
        """Extract FilePath from MetadataVO args."""
        path_val = (
            args.data.get("path", ".")
            if hasattr(args, "data")
            else args.get("path", ".")
        )
        return FilePath(value=str(path_val))

    async def _handle_check(self, action_str: str, args: MetadataVO) -> ResponseData:
        path = self._extract_path(args)
        report = await self.container.analysis_orchestrator.run(path)
        data = self.container.to_dict(report)
        return ResponseData(data=data)

    async def _handle_security(self, action_str: str, args: MetadataVO) -> ResponseData:
        path = self._extract_path(args)
        report = await self.container.analysis_orchestrator.run(path)
        data = self.container.to_dict(report)
        return ResponseData(data={"bandit": data.get("bandit", [])})

    async def _handle_complexity(
        self, action_str: str, args: MetadataVO
    ) -> ResponseData:
        path = self._extract_path(args)
        report = await self.container.analysis_orchestrator.run(path)
        data = self.container.to_dict(report)
        return ResponseData(data={"radon": data.get("radon", [])})

    async def _handle_duplicates(
        self, action_str: str, args: MetadataVO
    ) -> ResponseData:
        path = self._extract_path(args)
        report = await self.container.analysis_orchestrator.run(path)
        data = self.container.to_dict(report)
        return ResponseData(data={"duplicates": data.get("duplicates", [])})

    async def _handle_trends(self, action_str: str, args: MetadataVO) -> ResponseData:
        path = self._extract_path(args)
        report = await self.container.analysis_orchestrator.run(path)
        data = self.container.to_dict(report)
        return ResponseData(data={"trends": data.get("trends", [])})

    async def _handle_fix(self, action_str: str, args: MetadataVO) -> ResponseData:
        path = self._extract_path(args)
        output = await self.container.fix_orchestrator.execute(path)
        return ResponseData(data={"output": str(output)})

    async def _handle_report(self, action_str: str, args: MetadataVO) -> ResponseData:
        path = self._extract_path(args)
        output_format = (
            str(args.data.get("format", "text"))
            if hasattr(args, "data")
            else str(args.get("format", "text"))
        )
        report = await self.container.analysis_orchestrator.run(path)
        data = self.container.to_dict(report)
        formatters = {
            "json": lambda r, d: {"format": "json", "data": d},
            "sarif": lambda r, d: {
                "format": "sarif",
                "data": self.container.to_sarif_report(r).value,
            },
            "junit": lambda r, d: {
                "format": "junit",
                "data": self.container.to_junit_report(r).value,
            },
        }
        formatter = formatters.get(
            output_format, lambda r, d: {"format": "text", "data": d}
        )
        return ResponseData(data=formatter(report, data))

    async def _handle_version(self, action_str: str, args: MetadataVO) -> ResponseData:
        return ResponseData(data=await self._get_version())

    async def _handle_adapters(self, action_str: str, args: MetadataVO) -> ResponseData:
        return ResponseData(
            data={"adapters": [str(a.name()) for a in self.container.adapters]}
        )

    async def _handle_install_hook(
        self, action_str: str, args: MetadataVO
    ) -> ResponseData:
        ok = self.container.hook_capability.install()
        return ResponseData(data={"installed": ok})

    async def _handle_uninstall_hook(
        self, action_str: str, args: MetadataVO
    ) -> ResponseData:
        ok = self.container.hook_capability.uninstall()
        return ResponseData(data={"uninstalled": ok})

    # Future / placeholder handlers
    async def _handle_batch(self, action_str: str, args: MetadataVO) -> ResponseData:
        return ResponseData(data={"error": "batch action not yet implemented"})

    async def _handle_multi_project(
        self, action_str: str, args: MetadataVO
    ) -> ResponseData:
        return ResponseData(data={"error": "multi_project action not yet implemented"})

    async def _handle_doctor(self, action_str: str, args: MetadataVO) -> ResponseData:
        return ResponseData(data={"error": "doctor action not yet implemented"})

    async def _handle_cancel(self, action_str: str, args: MetadataVO) -> ResponseData:
        return ResponseData(data={"error": "cancel action not yet implemented"})

    async def _get_version(self) -> Mapping[str, str]:
        from importlib.metadata import version as _v, PackageNotFoundError

        try:
            ver = _v("auto-linter")
        except PackageNotFoundError:
            ver = "1.0.0"
        return {"version": ver}

    def validate_action(self, action: ContentString) -> BooleanVO:
        """Check if action is known — uses canonical VALID_PIPELINE_ACTIONS."""
        return BooleanVO(
            value=str(action.value if hasattr(action, "value") else action)
            in VALID_PIPELINE_ACTIONS
        )