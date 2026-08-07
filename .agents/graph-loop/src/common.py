"""Shared helpers: time, subprocess, logging."""
from __future__ import annotations

import subprocess
from datetime import datetime
from pathlib import Path
from typing import Optional


def now_iso() -> str:
    return datetime.now().astimezone().isoformat(timespec="seconds")


def parse_iso(ts: Optional[str]) -> Optional[datetime]:
    if not ts or ts == "null":
        return None
    try:
        return datetime.fromisoformat(ts)
    except (ValueError, TypeError):
        return None


def elapsed_minutes(ts: Optional[str]) -> int:
    dt = parse_iso(ts)
    if dt is None:
        return 0
    return int((datetime.now().astimezone() - dt).total_seconds() // 60)


def run(cmd: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Run a command, never raise on non-zero exit or missing binary.

    Returns a CompletedProcess; on missing executable it simulates an
    exit code of 127 so callers can log a traceable failure instead of
    crashing the pipeline."""
    kwargs.setdefault("capture_output", True)
    kwargs.setdefault("text", True)
    try:
        return subprocess.run(cmd, **kwargs)
    except FileNotFoundError:
        trace = f"{cmd[0]}: command not found"
        return subprocess.CompletedProcess(cmd, 127, stdout="", stderr=trace)


class Logger:
    """Append to execution.log using the same format as the bash scripts."""

    def __init__(self, log_file: Path):
        self.log_file = Path(log_file)
        self.log_file.parent.mkdir(parents=True, exist_ok=True)

    def write(self, component: str, event: str, message: str) -> None:
        line = f"[{now_iso()}] [{component}:{event}] {message}\n"
        try:
            with self.log_file.open("a", encoding="utf-8") as f:
                f.write(line)
        except OSError:
            pass
