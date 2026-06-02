import json
import logging
import os


from ..taxonomy import (
    AdapterName,
    ColumnNumber,
    ComplianceStatus,
    Count,
    ErrorCode,
    FilePath,
    LineNumber,
    LintMessage,
    LintResult,
    LintResultList,
    Severity,
    PatternList,
    ScanError,
    ErrorMessage,
)

from ..contract import ILinterAdapterPort, ICommandExecutorPort, IPathNormalizationPort

logger = logging.getLogger("mcp.adapters.python")


def _resolve_working_dir(path: FilePath) -> FilePath:
    path_str = str(path)
    try:
        from pathlib import Path
        current = Path(os.path.abspath(path_str))
        if current.is_file():
            current = current.parent
        for _ in range(10):
            if (
                (current / "auto_linter.config.yaml").is_file()
                or (current / "auto_linter.config.python.yaml").is_file()
                or (current / "pyproject.toml").is_file()
                or (current / ".git").is_dir()
            ):
                return FilePath(value=str(current))
            if current.parent == current:
                break
            current = current.parent
    except Exception as e:
        logger.debug(f"Error resolving working directory: {e}")
    return FilePath(value=".")


class ComplexityAdapter(ILinterAdapterPort):
    """Adapter for Radon complexity checker."""

    def __init__(
        self,
        executor: ICommandExecutorPort,
        path_norm: IPathNormalizationPort,
        bin_path: FilePath | None = None,
        threshold: Count = Count(value=10),
    ):
        self.executor = executor
        self.path_norm = path_norm
        self.bin_path = bin_path
        self.threshold = threshold

    def name(self) -> AdapterName:
        return AdapterName(value="radon")

    async def scan(self, path: FilePath) -> LintResultList:
        path_str = str(path.value)
        results = []
        try:
            executable = "radon"
            if self.bin_path:
                executable = os.path.join(str(self.bin_path.value), "radon")

            cmd = [executable, "cc", os.path.abspath(path_str), "-s", "--json"]

            result = await self.executor.execute_command(
                command=PatternList(values=cmd),
                working_dir=_resolve_working_dir(path),
            )

            if not str(result.stdout).strip():
                return LintResultList(values=[])

            data = json.loads(str(result.stdout))
            for filename, issues in data.items():
                filename_vo = self.path_norm.resolve_infrastructure_path(
                    FilePath(value=filename), path
                )
                for issue in issues:
                    if not isinstance(issue, dict):
                        continue
                    if issue.get("complexity", 0) > int(self.threshold):
                        results.append(
                            LintResult(
                                file=filename_vo,
                                line=LineNumber(value=issue["lineno"]),
                                column=ColumnNumber(value=issue.get("col_offset", 0)),
                                code=ErrorCode(code="complexity"),
                                message=LintMessage(
                                    value=f"High complexity ({issue['complexity']}) in {issue['name']}"
                                ),
                                source=self.name(),
                                severity=ComplexityAdapter.severity_from_complexity(
                                    issue.get("complexity", 0)
                                ),
                            )
                        )
        except Exception as e:
            logger.error(f"Error running Radon: {e}", exc_info=True)
            return ScanError(
                path=path,
                message=ErrorMessage(value=f"Radon execution failed: {e}"),
                adapter_name=self.name()
            )
        return LintResultList(values=results)

    @staticmethod
    def severity_from_complexity(complexity) -> Severity:
        """Map complexity number to Severity level."""
        if complexity > 20:
            return Severity.CRITICAL
        if complexity > 15:
            return Severity.HIGH
        if complexity > 10:
            return Severity.MEDIUM
        if complexity > 5:
            return Severity.LOW
        return Severity.INFO

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        return ComplianceStatus(value=False)


class DuplicateAdapter(ILinterAdapterPort):
    """Adapter for duplicate code detection."""

    def __init__(
        self,
        executor: ICommandExecutorPort,
        path_norm: IPathNormalizationPort,
        bin_path: FilePath | None = None,
    ):
        self.executor = executor
        self.path_norm = path_norm
        self.bin_path = bin_path

    def name(self) -> AdapterName:
        return AdapterName(value="duplicates")

    async def scan(self, path: FilePath) -> LintResultList:
        path_str = str(path)
        results: list[LintResult] = []
        try:
            abs_path = os.path.abspath(path_str)
            if os.path.isfile(abs_path):
                self._check_file(abs_path, results)
            elif os.path.isdir(abs_path):
                for dirpath, _, filenames in os.walk(abs_path):
                    if "__pycache__" in dirpath or ".venv" in dirpath:
                        continue
                    for filename in filenames:
                        if filename.endswith((".py", ".js", ".ts")):
                            file_path = os.path.join(dirpath, filename)
                            self._check_file(file_path, results)
        except Exception as e:
            logger.debug(f"Error during Python file analysis: {e}")
        return LintResultList(values=results)

    def _check_file(self, file_path, results):
        """Helper to check a single file's length."""
        try:
            with open(file_path, "r", encoding="utf-8") as f:
                lines = f.readlines()
                if len(lines) > 500:
                    results.append(
                        LintResult(
                            file=FilePath(value=file_path),
                            line=LineNumber(value=1),
                            column=ColumnNumber(value=0),
                            code=ErrorCode(code="DUPE001"),
                            message=LintMessage(
                                value=f"File exceeds 500 lines ({len(lines)}); potential duplication or SRP violation."
                            ),
                            source=self.name(),
                            severity=Severity.LOW,
                        )
                    )
        except (OSError, UnicodeDecodeError):
            pass

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        return ComplianceStatus(value=False)


class TrendsAdapter(ILinterAdapterPort):
    """Adapter for quality trend tracking."""

    def __init__(
        self,
        executor: ICommandExecutorPort,
        path_norm: IPathNormalizationPort,
        history_file: FilePath = FilePath(value=".auto_lint_history"),
    ):
        self.executor = executor
        self.path_norm = path_norm
        self.history_file = history_file

    def name(self) -> AdapterName:
        return AdapterName(value="trends")

    async def scan(self, path: FilePath) -> LintResultList:
        results = []
        history_path = str(self.history_file)
        if os.path.exists(history_path):
            history = []
            try:
                with open(history_path, "r", encoding="utf-8") as f:
                    for line_num, line in enumerate(f, 1):
                        line = line.strip()
                        if not line:
                            continue
                        try:
                            history.append(json.loads(line))
                        except json.JSONDecodeError as e:
                            logger.warning(
                                f"Skipping corrupted line {line_num} in {history_path}: {e}"
                            )
                            continue
                if len(history) > 1:
                    last_score = history[-1].get("score", 0)
                    prev_score = history[-2].get("score", 0)
                    if last_score < prev_score:
                        results.append(
                            LintResult(
                                file=FilePath(value="project"),
                                line=LineNumber(value=1),
                                column=ColumnNumber(value=0),
                                code=ErrorCode(code="TREND001"),
                                message=LintMessage(
                                    value=f"Quality trend is negative: {prev_score} -> {last_score}"
                                ),
                                source=self.name(),
                                severity=Severity.MEDIUM,
                            )
                        )
            except (OSError, IOError) as e:
                logger.warning(f"Could not read history file {history_path}: {e}")
                pass
        return LintResultList(values=results)

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        return ComplianceStatus(value=False)


class DependencyAdapter(ILinterAdapterPort):
    """Adapter for dependency vulnerability scanning."""

    def __init__(
        self,
        executor: ICommandExecutorPort,
        path_norm: IPathNormalizationPort,
        bin_path: FilePath | None = None,
    ):
        self.executor = executor
        self.path_norm = path_norm
        self.bin_path = bin_path

    def name(self) -> AdapterName:
        return AdapterName(value="pip-audit")

    async def scan(self, path: FilePath) -> LintResultList:
        results = []
        try:
            executable = "pip-audit"
            if self.bin_path:
                executable = os.path.join(str(self.bin_path.value), "pip-audit")

            cmd = [executable, "-f", "json"]
            result = await self.executor.execute_command(
                command=PatternList(values=cmd),
                working_dir=_resolve_working_dir(path),
            )

            if not str(result.stdout).strip():
                return LintResultList(values=[])

            data = json.loads(str(result.stdout))
            for item in data.get("dependencies", []):
                for vuln in item.get("vulns", []):
                    results.append(
                        LintResult(
                            file=FilePath(value="requirements.txt"),
                            line=LineNumber(value=1),
                            column=ColumnNumber(value=0),
                            code=ErrorCode(code=vuln["id"]),
                            message=LintMessage(
                                value=f"Vulnerability in {item['name']} ({item['version']}): {vuln['fix_versions']}"
                            ),
                            source=self.name(),
                            severity=Severity.CRITICAL,
                        )
                    )
        except Exception as e:
            logger.error(f"Error running pip-audit: {e}", exc_info=True)
            return ScanError(
                path=path,
                message=ErrorMessage(value=f"pip-audit execution failed: {e}"),
                adapter_name=self.name()
            )
        return LintResultList(values=results)

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        return ComplianceStatus(value=False)
