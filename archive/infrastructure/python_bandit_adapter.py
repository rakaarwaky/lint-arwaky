"""Bandit adapter for Python security scanning."""

from __future__ import annotations

from ..taxonomy import (
    AdapterName,
    ColumnNumber,
    ComplianceStatus,
    ErrorCode,
    FilePath,
    LineNumber,
    LintMessage,
    LintResult,
    LintResultList,
    Severity,
    PatternList,
    Timeout,
    ScanError,
    ErrorMessage,
)

import json
import logging
import os


from ..contract import ILinterAdapterPort, ICommandExecutorPort, IPathNormalizationPort


# No longer using standalone shim, using IPathNormalization instead
# from infrastructure.path_normalization_provider import resolve_infrastructure_path

logger = logging.getLogger("mcp.adapters.bandit")


class BanditAdapter(ILinterAdapterPort):
    """Adapter for Bandit security scanner."""

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
        return AdapterName(value="bandit")

    def _resolve_working_dir(self, path: FilePath) -> FilePath:
        path_str = str(path)
        try:
            from pathlib import Path
            current = Path(os.path.abspath(path_str))
            if current.is_file():
                current = current.parent
            for _ in range(10):
                if (
                    (current / "auto_linter.config.python.yaml").is_file() or (current / "auto_linter.config.javascript.yaml").is_file() or (current / "auto_linter.config.rust.yaml").is_file()
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

    async def scan(self, path: FilePath) -> LintResultList:
        path_str = str(path)
        results = []
        try:
            executable = "bandit"
            if self.bin_path:
                executable = os.path.join(str(self.bin_path), "bandit")

            cmd = [executable, "-r", path_str, "-f", "json", "-q"]
            result = await self.executor.execute_command(
                command=PatternList(values=cmd),
                working_dir=self._resolve_working_dir(path),
                timeout=Timeout(value=120.0),
            )

            if not str(result.stdout).strip():
                return LintResultList(values=[])

            data = json.loads(str(result.stdout))
            for item in data.get("results", []):
                raw_severity = item["issue_severity"].upper()
                if raw_severity == "HIGH":
                    severity = Severity.CRITICAL
                elif raw_severity == "MEDIUM":
                    severity = Severity.HIGH
                else:
                    severity = Severity.MEDIUM

                # Resolve filename relative to the original scan path
                filename = self.path_norm.resolve_infrastructure_path(
                    FilePath(value=item["filename"]), context_path=path
                )
                results.append(
                    LintResult(
                        file=filename,
                        line=LineNumber(value=item["line_number"]),
                        column=ColumnNumber(value=0),
                        code=ErrorCode(code=item["test_id"]),
                        message=LintMessage(value=item["issue_text"]),
                        source=self.name(),
                        severity=severity,
                    )
                )
        except Exception as e:
            logger.error(f"Error running Bandit: {e}", exc_info=True)
            return ScanError(
                path=path,
                message=ErrorMessage(value=f"Bandit execution failed: {e}"),
                adapter_name=self.name()
            )
        return LintResultList(values=results)

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        return ComplianceStatus(value=False)
