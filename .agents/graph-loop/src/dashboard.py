"""Terminal dashboard."""
from __future__ import annotations

from pathlib import Path

from src.config import Config

RED, GREEN, YELLOW, BLUE, CYAN, MAGENTA, NC = (
    "\033[0;31m", "\033[0;32m", "\033[1;33m", "\033[0;34m", "\033[0;36m",
    "\033[0;35m", "\033[0m")
BOX_WIDTH = 60


class Dashboard:
    def __init__(self, config: Config):
        self.config = config

    def _state(self) -> dict | None:
        import json
        try:
            return json.loads(self.config.state_file.read_text())
        except (FileNotFoundError, json.JSONDecodeError):
            return None

    def _features(self) -> dict | None:
        import json
        try:
            return json.loads(self.config.features_file.read_text())
        except (FileNotFoundError, json.JSONDecodeError):
            return None

    @staticmethod
    def _title(title: str) -> str:
        fill = max(BOX_WIDTH - len(title) - 3, 1)
        return f" {title} {'═' * fill}"

    def header(self) -> None:
        print(f"\n{CYAN}┌{'─' * (BOX_WIDTH - 2)}┐{NC}")
        print(f"{CYAN}│{NC} {self._title('GRAPH LOOP PIPELINE DASHBOARD')}")
        print(f"{CYAN}│{NC} {self._title(f'v2.0 / Python')}")
        print(f"{CYAN}└{'─' * (BOX_WIDTH - 2)}┘{NC}\n")

    def pipeline_status(self) -> None:
        print(f"{BLUE}━━━ Pipeline Status ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{NC}")
        s = self._state()
        if not s:
            print(f"{RED}State file not found{NC}\n"); return
        p = s.get("pipeline", {})
        print(f"  State:         {GREEN}{p.get('current_state')}{NC}")
        print(f"  Feature:       {p.get('feature', 'none')}")
        print(f"  Pipeline:      {p.get('id', 'none')}")
        print(f"  Correlation:   {p.get('correlation_id', 'none')}")
        print(f"  Started:       {p.get('started_at', 'none')}")
        print(f"  Rejection:     {YELLOW}{p.get('rejection_loop_counter', 0)}/3{NC}")
        print(f"  Iteration:     {YELLOW}{p.get('pipeline_iteration_counter', 0)}/5{NC}\n")

    def node_status(self) -> None:
        print(f"{BLUE}━━━ Node Status ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{NC}")
        s = self._state()
        if not s:
            print(f"{RED}State file not found{NC}\n"); return
        p = s.get("pipeline", {})
        nodes = p.get("parallel_nodes", {})
        for node, label in (("business-analyst", "Business-Analyst"), ("tech-lead", "Tech-Lead")):
            n = nodes.get(node, {})
            if n.get("status") == "completed":
                print(f"  {label + ':':22}{GREEN}COMPLETED{NC}  → {n.get('report_file', 'none')}")
            elif n.get("status") == "running":
                print(f"  {label + ':':22}{YELLOW}RUNNING{NC}")
            else:
                print(f"  {label + ':':22}{RED}IDLE{NC}")
        current = p.get("current_state", "")
        order = ["ARCHITECT", "DEVELOPER", "QUALITY-ANALYSIS", "MERGED"]
        idx = order.index(current) if current in order else -1
        for i, label in enumerate(("Architect", "Developer", "Quality-Analysis")):
            if idx == i or (current == "ARCHITECT" and i == 0):
                print(f"  {label + ':':22}{YELLOW}ACTIVE{NC}")
            elif idx > i or current == "MERGED":
                print(f"  {label + ':':22}{GREEN}DONE{NC}")
            else:
                print(f"  {label + ':':22}{RED}WAITING{NC}")
        print()

    def feature_queue(self) -> None:
        print(f"{BLUE}━━━ Feature Queue ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{NC}")
        data = self._features()
        if not data:
            print(f"{RED}Features file not found{NC}\n"); return
        feats = data.get("features", {})
        print(f"  Total features: {len(feats)}\n")
        for status, color in (("PENDING", YELLOW), ("LOCKED", BLUE), ("ACTIVE", GREEN),
                              ("DONE", GREEN), ("FAILED", RED), ("BLOCKED", RED)):
            count = sum(1 for f in feats.values() if f.get("status") == status)
            if count:
                print(f"  {color}{status}:{NC} {count}")
        print()

    def recent_activity(self) -> None:
        print(f"{BLUE}━━━ Recent Activity (last 10 events) ━━━━━━━━━━━━━━━━━━━━━━━━{NC}")
        log = self.config.log_file
        if not log.exists():
            print(f"{RED}Log file not found{NC}\n"); return
        for line in log.read_text().splitlines()[-10:]:
            print(f"  {line}")
        print()

    def metrics(self) -> None:
        print(f"{BLUE}━━━ Metrics ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{NC}")
        data = self._features()
        if not data:
            return
        feats = list(data.get("features", {}).values())
        total = len(feats)
        completed = sum(1 for f in feats if f.get("status") == "DONE")
        failed = sum(1 for f in feats if f.get("status") in ("FAILED", "BLOCKED"))
        print(f"  Completed:    {completed}")
        print(f"  Failed:       {failed}")
        print(f"  Total:        {total}")
        if total:
            print(f"  Success rate: {completed * 100 / total:.1f}%")
        print()

    def full(self) -> None:
        self.header(); self.pipeline_status(); self.node_status()
        self.feature_queue(); self.recent_activity(); self.metrics()


def cli(argv: list[str]) -> int:
    command = argv[0] if argv else "full"
    d = Dashboard(Config())
    actions = {"full": d.full, "status": d.pipeline_status, "nodes": d.node_status,
               "features": d.feature_queue, "activity": d.recent_activity, "metrics": d.metrics}
    fn = actions.get(command)
    if fn is None:
        print("Usage: main.py dashboard {full|status|nodes|features|activity|metrics}")
        return 1
    fn()
    return 0
