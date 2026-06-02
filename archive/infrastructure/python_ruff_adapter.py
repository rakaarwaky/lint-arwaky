"""Ruff adapter for Python linting."""

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
    AdapterError,
    ScanError,
    ErrorMessage,
)

import json
import logging
import os

from ..contract import ILinterAdapterPort, ICommandExecutorPort, IPathNormalizationPort

logger = logging.getLogger("mcp.adapters.ruff")


class RuffAdapter(ILinterAdapterPort):
    """Adapter for Ruff linter."""

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
        return AdapterName(value="ruff")

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

    async def scan(self, path: FilePath) -> LintResultList | ScanError | AdapterError:
        path_str = str(path)
        results = []
        try:
            abs_path = os.path.abspath(path_str)
            executable = self._resolve_executable(AdapterName(value="ruff"))

            cmd = [
                str(executable),
                "check",
                abs_path,
                "--output-format=json",
                "--exit-zero",
                "--no-cache",
            ]
            result = await self.executor.execute_command(
                command=PatternList(values=cmd),
                working_dir=self._resolve_working_dir(path),
                timeout=Timeout(value=60.0),
            )

            if result.stderr:
                logger.debug(f"Ruff stderr: {str(result.stderr)}")

            stdout_str = str(result.stdout).strip()
            findings = json.loads(stdout_str) if stdout_str else []
            for f in findings:
                filename_vo = self.path_norm.normalize_path(
                    FilePath(value=f.get("filename", ""))
                )
                filename = str(filename_vo)
                if not filename:
                    continue

                results.append(
                    LintResult(
                        file=FilePath(value=filename),
                        line=LineNumber(value=f.get("location", {}).get("row", 0)),
                        column=ColumnNumber(
                            value=f.get("location", {}).get("column", 0)
                        ),
                        code=ErrorCode(code=f.get("code", "UNKNOWN")),
                        message=LintMessage(value=f.get("message", "")),
                        source=AdapterName(value="ruff"),
                        severity=self._map_severity(
                            f.get("severity", ""), f.get("code", "")
                        ),
                    )
                )
        except Exception as e:
            logger.error(f"Ruff scan failed: {e}")
            return ScanError(
                path=path,
                message=ErrorMessage(value=str(e)),
                adapter_name=self.name()
            )

        return LintResultList(values=results)

    async def apply_fix(self, path: FilePath) -> ComplianceStatus | AdapterError:
        path_str = str(path)
        try:
            executable = self._resolve_executable(AdapterName(value="ruff"))
            cmd = [str(executable), "check", path_str, "--fix", "--exit-zero"]
            await self.executor.execute_command(
                command=PatternList(values=cmd),
                working_dir=self._resolve_working_dir(path),
                timeout=Timeout(value=60.0),
            )
            return ComplianceStatus(value=True)
        except Exception as e:
            logger.error(f"Error applying Ruff fixes: {e}")
            return AdapterError(
                adapter_name=self.name(),
                message=ErrorMessage(value=str(e))
            )

    def _resolve_executable(self, name: AdapterName) -> FilePath:
        if self.bin_path:
            return FilePath(value=os.path.join(str(self.bin_path), str(name)))
        return FilePath(value=str(name))

    def _map_severity(self, raw, code="") -> Severity:
        """Map Ruff severity string to Severity VO."""
        s = str(raw).upper()
        c = str(code).upper()

        # Logic errors and syntax errors are CRITICAL
        if c.startswith(("F", "E9")):
            return Severity.CRITICAL

        if "ERROR" in s:
            return Severity.HIGH
        if "WARNING" in s:
            return Severity.MEDIUM
        return Severity.LOW

    async def _to_lint_result(self, item, scan_path: FilePath) -> LintResult:
        data = item
        code = data.get("code", "")
        severity = Severity.MEDIUM
        if code.startswith("F") or code.startswith("E9"):
            severity = Severity.HIGH
        elif code.startswith("W"):
            severity = Severity.LOW

        filename_vo = self.path_norm.resolve_infrastructure_path(
            FilePath(value=data["filename"]), scan_path
        )

        return LintResult(
            file=filename_vo,
            line=LineNumber(value=data["location"]["row"]),
            column=ColumnNumber(value=data["location"]["column"]),
            code=ErrorCode(code=code),
            message=LintMessage(value=data["message"]),
            source=self.name(),
            severity=severity,
        )
