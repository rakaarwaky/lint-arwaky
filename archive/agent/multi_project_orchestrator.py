from typing import Type
import asyncio
import json
import logging
from pathlib import Path

from ..taxonomy import (
    AggregatedResults,
    BooleanVO,
    ComplianceStatus,
    ErrorMessage,
    FilePath,
    PatternList,
    ProjectResult,
    ResponseData,
    Score,
    SuccessStatus,
)
from ..contract import (
    MultiProjectAggregate,
    MultiProjectOrchestratorAggregate,
    PipelineOutputAggregate,
    IFileSystemPort,
    IConfigDiscoveryPort,
    ServiceContainerAggregate,
    IJobRegistryPort,
    IArchComplianceProtocol,
    ILintReportFormatterProtocol,
)

logger = logging.getLogger("agent.orchestrator")


class MultiProjectRequest(MultiProjectAggregate):
    """Concrete implementation of the multi-project input contract."""

    @property
    def _INTERFACE_REGISTRY(self) -> type[IJobRegistryPort]:
        return IJobRegistryPort

    @property
    def _INTERFACE_COMPLIANCE(self) -> type[IArchComplianceProtocol]:
        return IArchComplianceProtocol


class MultiProjectOrchestrator(MultiProjectOrchestratorAggregate):
    """Orchestrates multi-project scans.

    Agent coordinates. Infrastructure adapts. Capabilities think.
    """

    container: ServiceContainerAggregate

    def __init__(self, container: ServiceContainerAggregate):
        super().__init__(container=container)
        self._formatter = container.get(ILintReportFormatterProtocol)

    @property
    def _INTERFACE_FS(self) -> Type[IFileSystemPort]:
        from ..contract import IFileSystemPort

        return IFileSystemPort

    @property
    def _INTERFACE_CONFIG(self) -> Type[IConfigDiscoveryPort]:
        from ..contract import IConfigDiscoveryPort

        return IConfigDiscoveryPort

    async def execute_multi_project(
        self,
        request: MultiProjectAggregate,
    ) -> PipelineOutputAggregate:
        """Orchestrate linting across multiple projects."""
        job_id = await self.container.job_registry.create_job("multi_project")

        try:
            paths = list(request.paths.values) if request.paths else []
            if request.config_path:
                config_paths = self.load_config(Path(str(request.config_path.value)))
                paths.extend([FilePath(value=p) for p in config_paths])

            if request.use_retry and bool(request.use_retry):
                results = await self.container.job_registry.run_with_retry(
                    lambda: self.scan_all_projects(paths)
                )
            else:
                results = await self.scan_all_projects(paths)

            data = results.to_dict()
            await self.container.job_registry.complete_job(job_id, data)
            OutputClass = self.container.get(PipelineOutputAggregate)
            return OutputClass(
                success=SuccessStatus(value=BooleanVO(value=True)),
                job_id=job_id,
                data=ResponseData(value=data),
            )

        except Exception as e:
            await self.container.job_registry.fail_job(job_id, str(e))
            OutputClass = self.container.get(PipelineOutputAggregate)
            return OutputClass(
                success=SuccessStatus(value=BooleanVO(value=False)),
                job_id=job_id,
                error=ErrorMessage(value=str(e)),
            )

    async def analyze_project(self, path: FilePath) -> ProjectResult:
        """Analyze a single project using its own DI container."""
        try:
            # Get the project-specific container
            container = self.container.get_for_path(str(path))

            # Use the project-specific analysis orchestrator
            report = await container.analysis_orchestrator.run(path)
            data = self._formatter.report_to_dict(report)
            issues = []
            for source, results in data.items():
                if source in ["score", "summary", "is_passing"]:
                    continue
                if isinstance(results, list):
                    issues.extend(results)
            return ProjectResult(
                path=path,
                score=Score(value=float(data.get("score", 0.0))),
                is_passing=ComplianceStatus(value=bool(data.get("is_passing", True))),
                issues=issues,
                adapters=PatternList(
                    values=list(data.keys() - {"score", "summary", "is_passing"})
                ),
            )
        except Exception as e:
            return ProjectResult(
                path=path,
                score=Score(value=0.0),
                is_passing=ComplianceStatus(value=False),
                issues=[],
                adapters=PatternList(values=[]),
                error=ErrorMessage(value=str(e)),
            )

    async def scan_all_projects(
        self,
        paths: list[FilePath],
        max_concurrency: int = 10,
    ) -> AggregatedResults:
        """Scan a specific list of projects."""
        if not paths:
            return AggregatedResults()

        semaphore = asyncio.Semaphore(max_concurrency)

        async def limited_analyze(p: FilePath) -> ProjectResult:
            async with semaphore:
                return await self.analyze_project(p)

        results = await asyncio.gather(*[limited_analyze(p) for p in paths])
        return self.aggregate_results(list(results))

    def aggregate_results(self, projects: list[ProjectResult]) -> AggregatedResults:
        """Aggregate results from multiple projects."""
        passing = [p for p in projects if p.is_passing]
        scores = [float(p.score) for p in projects if float(p.score) > 0]
        return AggregatedResults(
            projects=projects,
            total_projects=len(projects),
            passing_projects=len(passing),
            failing_projects=len(projects) - len(passing),
            average_score=Score(value=sum(scores) / len(scores) if scores else 0.0),
        )

    @staticmethod
    def load_config(config_path: Path | None) -> list[str]:
        """Load list of project paths from a config file."""
        if not config_path or not config_path.exists():
            return []

        try:
            with open(config_path, "r") as f:
                if config_path.suffix == ".json":
                    data = json.load(f)
                    return data.get("projects", [])
                elif config_path.suffix in [".yaml", ".yml"]:
                    import yaml

                    data = yaml.safe_load(f)
                    return data.get("projects", [])
        except (OSError, json.JSONDecodeError):
            # Config file missing or malformed — treat as no projects
            pass
        return []

    @staticmethod
    def find_projects(
        root: FilePath, config_name: str = ".auto_linter.json"
    ) -> list[FilePath]:
        """Find all projects with auto-linter configs."""
        from pathlib import Path

        root_path = Path(str(root))
        projects: list[FilePath] = []
        for config_file in root_path.rglob(config_name):
            projects.append(FilePath(value=str(config_file.parent)))
        for config_file in root_path.rglob("auto_linter.config.yaml"):
            p = FilePath(value=str(config_file.parent))
            if p not in projects:
                projects.append(p)
        return projects
