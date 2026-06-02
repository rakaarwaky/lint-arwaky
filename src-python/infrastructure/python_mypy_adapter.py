"""MyPy adapter for Python type checking."""

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

import logging
import os
import re


from ..contract import ILinterAdapterPort, ICommandExecutorPort, IPathNormalizationPort

logger = logging.getLogger("mcp.adapters.mypy")


class MyPyAdapter(ILinterAdapterPort):
    """Adapter for MyPy type checker."""

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
        return AdapterName(value="mypy")

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
            executable = "mypy"
            if self.bin_path:
                executable = os.path.join(str(self.bin_path), "mypy")

            cmd = [
                executable,
                path_str,
                "--ignore-missing-imports",
                "--no-error-summary",
            ]
            result = await self.executor.execute_command(
                command=PatternList(values=cmd),
                working_dir=self._resolve_working_dir(path),
                timeout=Timeout(value=120.0),
            )
            output = str(result.stdout) + str(result.stderr)

            pattern = re.compile(r"^([^:]+):(\d+):(?:(\d+):)?\s+(\w+):\s+(.*)$")

            for line in output.splitlines():
                match = pattern.match(line.strip())
                if match:
                    filename, line_num, col_num, msg_type, msg = match.groups()
                    severity = self._map_severity(msg_type, msg)
                    filename_vo = self.path_norm.resolve_infrastructure_path(
                        FilePath(value=filename), path
                    )

                    results.append(
                        LintResult(
                            file=filename_vo,
                            line=LineNumber(value=int(line_num)),
                            column=ColumnNumber(
                                value=int(col_num)
                                if col_num and col_num.isdigit()
                                else 0
                            ),
                            code=ErrorCode(code="mypy"),
                            message=LintMessage(value=msg),
                            source=self.name(),
                            severity=severity,
                        )
                    )

        except FileNotFoundError as e:
            return ScanError(
                path=path,
                message=ErrorMessage(value=f"MyPy executable not found: {e}"),
                adapter_name=self.name()
            )
        except Exception as e:
            logger.error(f"Error running MyPy: {e}", exc_info=True)
            return ScanError(
                path=path,
                message=ErrorMessage(value=f"MyPy execution failed: {e}"),
                adapter_name=self.name()
            )

        return LintResultList(values=results)

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        return ComplianceStatus(value=False)

    def _map_severity(self, msg_type, msg="") -> Severity:
        m = str(msg).lower()
        if msg_type == "note":
            return Severity.INFO

        # Syntax/Parse errors in MyPy are CRITICAL
        if "syntax" in m or "parse" in m:
            return Severity.CRITICAL

        if msg_type == "warning":
            return Severity.MEDIUM
        return Severity.HIGH
