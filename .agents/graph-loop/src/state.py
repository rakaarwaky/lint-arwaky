"""State manager: atomic read/write of state.json + transitions + counters."""
from __future__ import annotations

import copy
import json
import os
import random
from datetime import datetime
from pathlib import Path
from typing import Any, Optional

from src.common import Logger, now_iso
from src.config import Config


def _default_state() -> dict:
    node = lambda: {"status": "idle", "task_id": None, "report_file": None,
                    "started_at": None, "completed_at": None}
    return {
        "version": "2.0",
        "pipeline": {
            "id": None, "feature": None, "current_state": "IDLE",
            "started_at": None, "iteration": 0, "correlation_id": None,
            "pipeline_iteration_counter": 0, "rejection_loop_counter": 0,
            "parallel_nodes": {"business-analyst": node(), "tech-lead": node()},
            "pending_merge": [], "failure": None, "escalation": None,
        },
    }


class StateManager:
    def __init__(self, config: Config, log: Logger):
        self.config = config
        self.file = config.state_file
        self.log = log
        self.file.parent.mkdir(parents=True, exist_ok=True)
        if not self.file.exists():
            self._save(_default_state())

    # ── atomic I/O ─────────────────────────────────────────────
    def load(self) -> dict:
        try:
            return json.loads(self.file.read_text())
        except (json.JSONDecodeError, FileNotFoundError):
            return _default_state()

    def _save(self, data: dict) -> None:
        text = json.dumps(data, indent=2)
        if not text.strip():
            self.log.write("state", "warn", "Refusing to write empty state")
            return
        tmp = self.file.with_suffix(".json.tmp")
        tmp.write_text(text)
        os.replace(tmp, self.file)          # atomic

    def _update(self, fn) -> dict:
        data = self.load()
        fn(data)
        self._save(data)
        return data

    # ── accessors ──────────────────────────────────────────────
    def get(self, path: str, default: Any = None) -> Any:
        node: Any = self.load()
        for part in path.strip(".").split("."):
            if not isinstance(node, dict) or part not in node:
                return default
            node = node[part]
        return default if node is None else node

    @property
    def current_state(self) -> str:
        return self.get("pipeline.current_state", "IDLE")

    @property
    def feature(self) -> Optional[str]:
        return self.get("pipeline.feature")

    @property
    def pipeline_id(self) -> Optional[str]:
        return self.get("pipeline.id")

    @property
    def correlation_id(self) -> str:
        return self.get("pipeline.correlation_id", "unknown")

    @property
    def rejection_counter(self) -> int:
        return int(self.get("pipeline.rejection_loop_counter", 0))

    @property
    def pipeline_counter(self) -> int:
        return int(self.get("pipeline.pipeline_iteration_counter", 0))

    @property
    def started_at(self) -> Optional[str]:
        return self.get("pipeline.started_at")

    # ── transitions ────────────────────────────────────────────
    def transition(self, new_state: str) -> None:
        def fn(d): d["pipeline"]["current_state"] = new_state
        self._update(fn)
        self.log.write("state", "transition", f"State changed to: {new_state}")

    def start_pipeline(self, feature: str) -> tuple[str, str]:
        now = datetime.now()
        pipeline_id = f"pipeline-{now:%Y%m%d}-{random.randint(100, 999)}"
        correlation_id = f"corr-{now:%Y%m%d}-{feature}-{random.randint(100, 999)}"

        def fn(d):
            p = d["pipeline"]
            fresh = copy.deepcopy(_default_state())["pipeline"]
            p.update(fresh)
            p["id"] = pipeline_id
            p["feature"] = feature
            p["current_state"] = "DISPATCHING"
            p["started_at"] = now_iso()
            p["correlation_id"] = correlation_id
            p["iteration"] = 1
            p["pipeline_iteration_counter"] = 1
            p["rejection_loop_counter"] = 0
            for n in ("business-analyst", "tech-lead"):
                p["parallel_nodes"][n]["status"] = "pending"
        self._update(fn)
        self.log.write("state", "start",
                       f"Pipeline started for feature: {feature} "
                       f"(ID: {pipeline_id}, correlation: {correlation_id})")
        return pipeline_id, correlation_id

    def start_analyzing(self) -> None:
        ts = now_iso()
        def fn(d):
            p = d["pipeline"]
            p["current_state"] = "ANALYZING"
            for n in ("business-analyst", "tech-lead"):
                p["parallel_nodes"][n]["status"] = "running"
                p["parallel_nodes"][n]["started_at"] = ts
        self._update(fn)
        self.log.write("state", "analyzing", "Parallel analysis started")

    def complete_node(self, node: str, report_file: str) -> None:
        ts = now_iso()
        def fn(d):
            n = d["pipeline"]["parallel_nodes"][node]
            n["status"] = "completed"
            n["report_file"] = report_file
            n["completed_at"] = ts
        self._update(fn)
        self.log.write("state", "node_complete", f"Node {node} completed: {report_file}")
        self.check_parallel_completion()

    def check_parallel_completion(self) -> bool:
        ba = self.get("pipeline.parallel_nodes.business-analyst.status")
        tl = self.get("pipeline.parallel_nodes.tech-lead.status")
        if ba == "completed" and tl == "completed":
            self.transition("ARCHITECT")
            self.log.write("state", "parallel_complete",
                           "Both Business-Analyst and Tech-Lead completed — transitioning to ARCHITECT")
            return True
        return False

    def merge_complete(self, merged_plan: str) -> None:
        def fn(d):
            d["pipeline"]["current_state"] = "DEVELOPER"
            d["pipeline"]["pending_merge"] = []
        self._update(fn)
        self.log.write("state", "merge_complete", f"Architect merge complete: {merged_plan}")

    def developer_complete(self, pr_number: str) -> None:
        def fn(d): d["pipeline"]["current_state"] = "QUALITY-ANALYSIS"
        self._update(fn)
        self.log.write("state", "developer_complete",
                       f"Developer completed — PR #{pr_number} created — transitioning to QUALITY-ANALYSIS")

    def qa_approved(self) -> None:
        def fn(d): d["pipeline"]["current_state"] = "MERGED"
        self._update(fn)
        self.log.write("state", "qa_approved", "Quality-Analysis approved — pipeline complete")

    def qa_rejected(self, reason: str) -> None:
        def fn(d): d["pipeline"]["current_state"] = "ARCHITECT"
        self._update(fn)
        self.log.write("state", "qa_rejected", f"Quality-Analysis rejected: {reason} — re-merge needed")

    def reset(self) -> None:
        def fn(d):
            d["pipeline"] = copy.deepcopy(_default_state())["pipeline"]
        self._update(fn)
        self.log.write("state", "reset", "Pipeline reset to IDLE")

    def _failure(self, state: str, reason: str, key: str) -> None:
        ts = now_iso()
        def fn(d):
            d["pipeline"]["current_state"] = state
            d["pipeline"][key] = {"reason": reason, "at": ts}
        self._update(fn)
        self.log.write("state", state.lower(), f"Pipeline marked {state}: {reason}")

    def failed(self, reason: str) -> None:
        self._failure("FAILED", reason, "failure")

    def blocked(self, reason: str) -> None:
        self._failure("BLOCKED", reason, "failure")

    def escalated(self, reason: str) -> None:
        self._failure("ESCALATED", reason, "escalation")

    def waiting_human(self, reason: str) -> None:
        self._failure("WAITING_HUMAN", reason, "escalation")

    # ── counters ───────────────────────────────────────────────
    def increment_rejection(self) -> int:
        def fn(d):
            d["pipeline"]["rejection_loop_counter"] += 1
        data = self._update(fn)
        c = data["pipeline"]["rejection_loop_counter"]
        self.log.write("state", "counter", f"Rejection loop counter incremented to: {c}")
        return c

    def increment_pipeline(self) -> int:
        def fn(d):
            d["pipeline"]["pipeline_iteration_counter"] += 1
        data = self._update(fn)
        c = data["pipeline"]["pipeline_iteration_counter"]
        self.log.write("state", "counter", f"Pipeline iteration counter incremented to: {c}")
        return c

    def node_started_at(self, node: str) -> Optional[str]:
        return self.get(f"pipeline.parallel_nodes.{node}.started_at")

    def node_status(self, node: str) -> Optional[str]:
        return self.get(f"pipeline.parallel_nodes.{node}.status")
