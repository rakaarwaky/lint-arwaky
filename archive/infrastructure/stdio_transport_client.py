"""Stdio transport — direct subprocess execution, no DesktopCommander needed."""

from __future__ import annotations

import asyncio
import os
from pathlib import Path

from ..contract import ICommandExecutorPort
from ..taxonomy import FilePath, ResponseData, Timeout, PatternList


class StdioClient(ICommandExecutorPort):
    """Direct subprocess transport. Runs commands locally without any intermediary."""

    def __init__(self, timeout: Timeout = Timeout(value=300.0)):
        self.timeout = timeout.value

    async def execute_command(
        self,
        command: PatternList,
        working_dir: FilePath = FilePath(value="."),
        timeout: Timeout | None = None,
    ) -> ResponseData:
        """Execute command via subprocess and capture output."""
        resolved_dir = str(Path(working_dir.value).resolve())
        timeout_val = timeout.value if timeout else self.timeout
        cmd_list = command.values

        try:
            proc = await asyncio.create_subprocess_exec(
                *cmd_list,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                cwd=resolved_dir,
                env={**os.environ, "PYTHONUNBUFFERED": "1"},
            )

            try:
                stdout_bytes, stderr_bytes = await asyncio.wait_for(
                    proc.communicate(), timeout=timeout_val
                )
            except asyncio.TimeoutError:
                proc.kill()
                await proc.wait()
                return ResponseData(
                    stdout="",
                    stderr=f"Command timed out after {timeout_val}s",
                    returncode=-1,
                    metadata={
                        "protocol": "Stdio",
                        "error": "timeout",
                    },
                )

            return ResponseData(
                stdout=stdout_bytes.decode("utf-8", errors="replace"),
                stderr=stderr_bytes.decode("utf-8", errors="replace"),
                returncode=proc.returncode if proc.returncode is not None else -1,
                metadata={"protocol": "Stdio"},
            )

        except FileNotFoundError:
            return ResponseData(
                stdout="",
                stderr=f"Command not found: {cmd_list[0]}",
                returncode=127,
                metadata={"protocol": "Stdio", "error": "command_not_found"},
            )
        except Exception as e:
            return ResponseData(
                stdout="",
                stderr=str(e),
                returncode=1,
                metadata={"protocol": "Stdio", "error": str(e)},
            )

    async def health_check(self) -> ResponseData:
        """Stdio is always healthy if Python is running."""
        import time

        start = time.time()
        result = await self.execute_command(
            PatternList(values=["echo", "health"]), FilePath(value=".")
        )
        latency_ms = (time.time() - start) * 1000

        if result.returncode == 0:
            return ResponseData(
                stdout=result.stdout,
                stderr=result.stderr,
                returncode=0,
                metadata={
                    "status": "healthy",
                    "latency_ms": round(latency_ms, 2),
                    "protocol": "Stdio",
                    "pid": os.getpid(),
                },
            )
        return ResponseData(
            stdout=result.stdout,
            stderr=result.stderr,
            returncode=result.returncode,
            metadata={
                "status": "unhealthy",
                "error": result.stderr or "Unknown",
                "protocol": "Stdio",
            },
        )

    def close(self):
        """No resources to clean up for stdio."""
        pass
