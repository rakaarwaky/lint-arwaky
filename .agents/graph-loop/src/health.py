"""Health check."""
from __future__ import annotations

import json
import os
import shutil
from pathlib import Path

from src.common import elapsed_minutes, now_iso, run
from src.config import Config


class HealthCheck:
    def __init__(self, config: Config):
        self.config = config
        self.health_log = config.resolve(config.get("paths.health_log",
                                                    ".agents/graph-loop/health.log"))
        self.health_log.parent.mkdir(parents=True, exist_ok=True)

    def _hlog(self, status: str, message: str) -> None:
        with self.health_log.open("a", encoding="utf-8") as f:
            f.write(f"[{now_iso()}] [{status}] {message}\n")

    def check_engine_process(self) -> bool:
        pid_file = Path(__file__).resolve().parent.parent / "engine.pid"
        if pid_file.exists():
            try:
                pid = int(pid_file.read_text().strip())
                os.kill(pid, 0)
                self._hlog("OK", f"Engine process running (PID: {pid})")
                return True
            except (ValueError, ProcessLookupError, PermissionError, OSError):
                self._hlog("WARN", f"Engine process not running (stale PID)")
                return False
        r = run(["systemctl", "is-active", "graph-loop.service"])
        if r.stdout.strip() == "active":
            self._hlog("OK", "Engine running via systemd")
            return True
        self._hlog("WARN", "No engine PID file or active service found")
        return False

    def check_agent_processes(self) -> bool:
        ok = True
        if shutil.which("pgrep"):
            r = run(["pgrep", "-f", "qwen -p"])
            count = len([l for l in r.stdout.splitlines() if l.strip()])
            self._hlog("OK" if count else "INFO",
                       f"Active agent processes: {count}" if count else "No active agent processes")
        results = self.config.results_dir
        if results.exists():
            stale = 0
            for pf in results.glob("*.pid"):
                try:
                    os.kill(int(pf.read_text().strip()), 0)
                except (ValueError, ProcessLookupError, PermissionError, OSError):
                    stale += 1
            if stale:
                self._hlog("WARN", f"Stale agent PID files: {stale}")
                ok = False
        return ok

    def check_state_file(self) -> bool:
        try:
            s = json.loads(self.config.state_file.read_text())
        except (FileNotFoundError, json.JSONDecodeError):
            self._hlog("ERROR", f"State file not found: {self.config.state_file}")
            return False
        current = s.get("pipeline", {}).get("current_state", "")
        if current in ("IDLE", "MERGED"):
            self._hlog("OK", f"State: {current} (healthy)"); return True
        if current in ("ANALYZING", "ARCHITECT", "DEVELOPER", "QUALITY-ANALYSIS", "DISPATCHING"):
            hours = elapsed_minutes(s.get("pipeline", {}).get("started_at")) // 60
            if hours >= 2:
                self._hlog("WARN", f"State {current} for {hours}h (may be stuck)"); return False
            self._hlog("OK", f"State: {current} (active)"); return True
        if current in ("WAITING_HUMAN", "ESCALATED"):
            self._hlog("WARN", f"State: {current} (requires attention)"); return False
        self._hlog("ERROR", f"State: {current} (failure)"); return False

    def check_disk_space(self) -> bool:
        usage = shutil.disk_usage(self.config.project_root).used * 100 // \
            shutil.disk_usage(self.config.project_root).total
        if usage >= 90:
            self._hlog("ERROR", f"Disk usage critical: {usage}%"); return False
        if usage >= 80:
            self._hlog("WARN", f"Disk usage high: {usage}%"); return False
        self._hlog("OK", f"Disk usage: {usage}%"); return True

    def check_log_size(self) -> bool:
        log = self.config.log_file
        if not log.exists():
            return True
        size_kb = log.stat().st_size // 1024
        if size_kb >= 10240:
            self._hlog("WARN", f"Log file large: {size_kb}KB (consider rotation)"); return False
        self._hlog("OK", f"Log file size: {size_kb}KB"); return True

    def check_lock_files(self) -> bool:
        locks = self.config.locks_dir
        if not locks.exists():
            return True
        count = len(list(locks.glob("*.lock")))
        if count > 1:
            self._hlog("WARN", f"Multiple lock files: {count} (expected: 0-1)"); return False
        self._hlog("OK", f"Lock files: {count}"); return True

    def report(self) -> None:
        GREEN, YELLOW, RED, NC = "\033[0;32m", "\033[1;33m", "\033[0;31m", "\033[0m"
        print("=== Graph Loop Health Report (v2.0 / Python) ===")
        print(f"Timestamp: {now_iso()}\n")
        checks = (("Engine process", self.check_engine_process),
                  ("Agent processes", self.check_agent_processes),
                  ("State file", self.check_state_file),
                  ("Disk space", self.check_disk_space),
                  ("Log size", self.check_log_size),
                  ("Lock files", self.check_lock_files))
        issues = 0
        for name, check in checks:
            ok = check()
            label = f"{GREEN}HEALTHY{NC}" if ok else f"{RED}ISSUE{NC}"
            mark = "✓" if ok else "✗"
            print(f"  {mark} {name:<18} {label}")
            issues += 0 if ok else 1
        print()
        if issues == 0:
            print(f"Status: {GREEN}HEALTHY{NC}")
        elif issues <= 2:
            print(f"Status: {YELLOW}WARNING ({issues} issues){NC}")
        else:
            print(f"Status: {RED}CRITICAL ({issues} issues){NC}")


def cli(argv: list[str]) -> int:
    h = HealthCheck(Config())
    command = argv[0] if argv else "report"
    actions = {"report": h.report, "check-engine": lambda: h.check_engine_process(),
               "check-agents": lambda: h.check_agent_processes(),
               "check-state": lambda: h.check_state_file(),
               "check-disk": lambda: h.check_disk_space(),
               "check-logs": lambda: h.check_log_size(),
               "check-locks": lambda: h.check_lock_files()}
    fn = actions.get(command)
    if fn is None:
        print("Usage: main.py health {report|check-engine|check-agents|check-state|check-disk|check-logs|check-locks}")
        return 1
    result = fn()
    return 0 if result is None or result else 1
