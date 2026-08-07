"""Single Source of Truth reader for config.yaml."""
from __future__ import annotations

import hashlib
from pathlib import Path
from typing import Any, Optional

import yaml

SCRIPT_DIR = Path(__file__).resolve().parent.parent          # .agents/graph-loop/
CONFIG_FILE = SCRIPT_DIR / "config.yaml"


class Config:
    def __init__(self, path: Path | str = CONFIG_FILE):
        self.path = Path(path)
        self.data: dict = yaml.safe_load(self.path.read_text()) or {}

    # ── generic accessor (dot path) ────────────────────────────
    def get(self, path: str, default: Any = None) -> Any:
        node: Any = self.data
        for part in path.strip(".").split("."):
            if not isinstance(node, dict) or part not in node:
                return default
            node = node[part]
        return default if node is None else node

    # ── top-level (from config.yaml) ──────────────────────────
    @property
    def project_root(self) -> Path:
        return Path(self.get("project_root", "/home/raka/mcp-arwaky/lint-arwaky"))

    # ── hardcoded constants ───────────────────────────────────
    POLL_INTERVAL = 30               # seconds between polls
    GLOBAL_TIMEOUT = 180             # minutes before pipeline blocked
    MAX_REJECTION_LOOPS = 3          # QA rejects before escalation
    MAX_PIPELINE_ITERATIONS = 5      # retries before blocked
    RESUME_STALE_MINUTES = 60        # auto-resume if stale < 60m
    BACKOFF_INITIAL = 2              # minutes
    BACKOFF_MAX = 30                 # minutes
    DEBOUNCE_SECONDS = 30
    IGNORE_BOT_EVENTS = True

    # ── node timeouts (minutes) ───────────────────────────────
    NODE_TIMEOUTS = {
        "business-analyst": 20,
        "tech-lead": 30,
        "architect": 30,
        "developer": 60,
        "quality-analysis": 30,
    }

    # ── node retry config ─────────────────────────────────────
    NODE_RETRY = {
        "business-analyst": {"retry_on_error": True, "max_retries": 2},
        "tech-lead": {"retry_on_error": True, "max_retries": 2},
        "architect": {"retry_on_error": True, "max_retries": 2},
        "developer": {"retry_on_error": True, "max_retries": 3},
        "quality-analysis": {"retry_on_error": False, "max_retries": 0},
    }

    # ── properties (use hardcoded, fallback to config) ────────
    @property
    def poll_interval(self) -> int:
        return int(self.get("settings.poll_interval_seconds", self.POLL_INTERVAL))

    @property
    def global_timeout_minutes(self) -> int:
        return int(self.get("settings.global_timeout_minutes", self.GLOBAL_TIMEOUT))

    @property
    def max_rejection_loops(self) -> int:
        return int(self.get("counters.max_rejection_loops", self.MAX_REJECTION_LOOPS))

    @property
    def max_pipeline_iterations(self) -> int:
        return int(self.get("counters.max_pipeline_iterations", self.MAX_PIPELINE_ITERATIONS))

    @property
    def resume_stale_minutes(self) -> int:
        return int(self.get("recovery.resume_if_stale_minutes", self.RESUME_STALE_MINUTES))

    @property
    def backoff_initial_minutes(self) -> int:
        return int(self.get("recovery.exponential_backoff.initial_minutes", self.BACKOFF_INITIAL))

    @property
    def backoff_max_minutes(self) -> int:
        return int(self.get("recovery.exponential_backoff.max_minutes", self.BACKOFF_MAX))

    @property
    def debounce_seconds(self) -> int:
        return int(self.get("trigger_guards.debounce_seconds", self.DEBOUNCE_SECONDS))

    @property
    def ignore_bot_events(self) -> bool:
        val = self.get("trigger_guards.ignore_bot_events", self.IGNORE_BOT_EVENTS)
        return str(val).lower() == "true"

    # ── paths (relative → absolute vs project root) ────────────
    def resolve(self, rel: str | Path) -> Path:
        p = Path(rel)
        return p if p.is_absolute() else self.project_root / p

    @property
    def state_file(self) -> Path:
        return self.resolve(self.get("paths.state_file", ".agents/graph-loop/state.json"))

    @property
    def features_file(self) -> Path:
        return self.resolve(self.get("paths.features_file", ".agents/graph-loop/features.json"))

    @property
    def log_file(self) -> Path:
        return self.resolve(self.get("paths.log_file", ".agents/graph-loop/execution.log"))

    @property
    def notifications_log(self) -> Path:
        return self.resolve(self.get("paths.notifications_log", ".agents/graph-loop/notifications.log"))

    @property
    def results_dir(self) -> Path:
        return self.resolve(self.get("paths.results_dir", ".agents/graph-loop/results"))

    @property
    def plans_dir(self) -> Path:
        return self.resolve(self.get("paths.plans_dir", ".agents/graph-loop/plans"))

    @property
    def reports_dir(self) -> Path:
        return self.resolve(self.get("paths.reports_dir", ".agents/graph-loop/reports"))

    @property
    def locks_dir(self) -> Path:
        return self.resolve(self.get("paths.locks_dir", ".agents/graph-loop/locks"))

    @property
    def prompts_dir(self) -> Path:
        return self.resolve(self.get("paths.prompts_dir", ".agents/graph-loop/prompts"))

    # ── nodes (hardcoded) ─────────────────────────────────────
    def node(self, name: str, field: str, default: Any = None) -> Any:
        cfg = self.get(f"nodes.{name}.{field}")
        if cfg is not None:
            return cfg
        node_cfg = self.NODE_RETRY.get(name, {})
        if field == "retry_on_error":
            return node_cfg.get("retry_on_error", False)
        if field == "max_retries":
            return node_cfg.get("max_retries", 0)
        return default

    def node_timeout(self, name: str) -> int:
        return self.NODE_TIMEOUTS.get(name, 30)

    # ── feature queue ──────────────────────────────────────────
    def features(self) -> list[dict]:
        feats = self.get("features", []) or []
        return sorted(feats, key=lambda f: int(f.get("priority", 999)))

    def feature_by_name(self, name: str) -> Optional[dict]:
        for f in self.get("features", []) or []:
            if f.get("name") == name:
                return f
        return None

    # ── FRD hash ───────────────────────────────────────────────
    @staticmethod
    def compute_frd_hash(frd_path: str | Path) -> str:
        p = Path(frd_path)
        if p.is_file():
            return "sha256:" + hashlib.sha256(p.read_bytes()).hexdigest()
        return "sha256:NOT_FOUND"

    # ── feature context block (identical output to bash) ──────
    def build_feature_context(
        self,
        feature_name: str,
        correlation_id: str = "unknown",
        pipeline_iteration: int = 1,
        rejection_loop: int = 0,
    ) -> str:
        feat = self.feature_by_name(feature_name)
        if feat is None:
            raise ValueError(f"Feature not found: {feature_name}")
        feature_path = feat["path"]
        frd_path = f"{feature_path}/FRD.md"
        frd_hash = self.compute_frd_hash(frd_path)
        return f"""## Feature Context
- **Feature:** {feature_name}
- **Feature Path:** {feature_path}
- **FRD Path:** {frd_path}
- **FRD Hash:** {frd_hash}
- **Project Root:** {self.project_root}
- **Correlation ID:** {correlation_id}
- **Pipeline Iteration:** {pipeline_iteration}/5
- **Rejection Loop:** {rejection_loop}/3
- **Rule:** Only analyze files within Feature Path. Do NOT touch files outside Feature Path.

## Shared Acceptance Criteria
- [ ] All findings must have evidence (file + line number)
- [ ] All recommendations must be actionable
- [ ] Report must follow the specified output schema
- [ ] FRD snapshot must be consistent with the recorded hash
"""
