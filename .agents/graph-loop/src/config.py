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

    # ── top-level ──────────────────────────────────────────────
    @property
    def project_root(self) -> Path:
        return Path(self.get("project_root", "/home/raka/mcp-arwaky/lint-arwaky"))

    @property
    def poll_interval(self) -> int:
        return int(self.get("settings.poll_interval_seconds", 30))

    @property
    def global_timeout_minutes(self) -> int:
        return int(self.get("settings.global_timeout_minutes", 180))

    @property
    def max_rejection_loops(self) -> int:
        return int(self.get("counters.max_rejection_loops", 3))

    @property
    def max_pipeline_iterations(self) -> int:
        return int(self.get("counters.max_pipeline_iterations", 5))

    @property
    def resume_stale_minutes(self) -> int:
        return int(self.get("recovery.resume_if_stale_minutes", 60))

    @property
    def backoff_initial_minutes(self) -> int:
        return int(self.get("recovery.exponential_backoff.initial_minutes", 2))

    @property
    def backoff_max_minutes(self) -> int:
        return int(self.get("recovery.exponential_backoff.max_minutes", 30))

    @property
    def debounce_seconds(self) -> int:
        return int(self.get("trigger_guards.debounce_seconds", 30))

    @property
    def ignore_bot_events(self) -> bool:
        return str(self.get("trigger_guards.ignore_bot_events", True)).lower() == "true"

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

    # ── nodes ──────────────────────────────────────────────────
    def node(self, name: str, field: str, default: Any = None) -> Any:
        return self.get(f"nodes.{name}.{field}", default)

    def node_timeout(self, name: str) -> int:
        return int(self.node(name, "timeout_minutes", 30))

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
