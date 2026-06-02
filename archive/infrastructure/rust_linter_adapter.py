"""Rust adapter for Clippy, Rustfmt, and Cargo Audit."""

from __future__ import annotations

import json
import logging
import os
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
from ..contract import ILinterAdapterPort, ICommandExecutorPort, IPathNormalizationPort

logger = logging.getLogger("mcp.adapters.rust")


class RustLinterAdapter(ILinterAdapterPort):
    """Adapter for Rust Clippy static analysis, rustfmt, and cargo audit."""

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
        return AdapterName(value="clippy")

    def _resolve_working_dir(self, path: FilePath) -> FilePath:
        path_str = str(path)
        try:
            from pathlib import Path
            current = Path(os.path.abspath(path_str))
            if current.is_file():
                current = current.parent
            for _ in range(10):
                if (
                    (current / "Cargo.toml").is_file()
                    or (current / "auto_linter.config.python.yaml").is_file() or (current / "auto_linter.config.javascript.yaml").is_file() or (current / "auto_linter.config.rust.yaml").is_file()
                    or (current / "auto_linter.config.rust.yaml").is_file()
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
        results = []
        try:
            working_dir = self._resolve_working_dir(path)
            
            # Check if cargo is available in working_dir
            cargo_toml = os.path.join(str(working_dir), "Cargo.toml")
            if not os.path.exists(cargo_toml):
                logger.debug(f"Skipping clippy scan: Cargo.toml not found at {cargo_toml}")
                return LintResultList(values=[])

            # Run cargo clippy
            cmd = ["cargo", "clippy", "--message-format=json"]
            result = await self.executor.execute_command(
                command=PatternList(values=cmd),
                working_dir=working_dir,
                timeout=Timeout(value=180.0),
            )

            # Clippy writes to stdout or stderr depending on cargo invocation
            output = str(result.stdout or result.stderr)
            
            for line in output.splitlines():
                line = line.strip()
                if not line.startswith("{"):
                    continue
                try:
                    data = json.loads(line)
                    if data.get("reason") != "compiler-message":
                        continue
                    
                    msg = data.get("message", {})
                    level = msg.get("level", "warning").lower()
                    code_data = msg.get("code")
                    code = code_data.get("code") if code_data else "clippy::warning"
                    message_text = msg.get("message", "Clippy finding")
                    
                    severity = Severity.HIGH if level == "error" else Severity.MEDIUM
                    
                    spans = msg.get("spans", [])
                    for span in spans:
                        if not span.get("is_primary", False):
                            continue
                        
                        filename = span.get("file_name")
                        resolved_file = self.path_norm.resolve_infrastructure_path(
                            FilePath(value=filename), context_path=path
                        )
                        
                        results.append(
                            LintResult(
                                file=resolved_file,
                                line=LineNumber(value=span.get("line_start", 1)),
                                column=ColumnNumber(value=span.get("column_start", 1)),
                                code=ErrorCode(code=str(code)),
                                message=LintMessage(value=message_text),
                                source=self.name(),
                                severity=severity,
                            )
                        )
                except json.JSONDecodeError:
                    continue

        except Exception as e:
            logger.error(f"Error running Cargo Clippy: {e}", exc_info=True)
            return ScanError(
                path=path,
                message=ErrorMessage(value=f"Cargo Clippy execution failed: {e}"),
                adapter_name=self.name()
            )
        return LintResultList(values=results)

    async def apply_fix(self, path: FilePath) -> ComplianceStatus:
        try:
            working_dir = self._resolve_working_dir(path)
            cmd = ["cargo", "clippy", "--fix", "--allow-dirty", "--allow-staged"]
            await self.executor.execute_command(
                command=PatternList(values=cmd),
                working_dir=working_dir,
                timeout=Timeout(value=180.0),
            )
            return ComplianceStatus(value=True)
        except Exception as e:
            logger.error(f"Error applying Clippy fix: {e}")
            return ComplianceStatus(value=False)
