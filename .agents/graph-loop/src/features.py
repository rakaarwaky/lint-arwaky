"""Feature queue manager: claiming, locks, cooldown, priority selection."""
from __future__ import annotations

import json
import os
from datetime import datetime, timedelta
from pathlib import Path
from typing import Optional

from src.common import Logger, now_iso, parse_iso
from src.config import Config

_DEFAULT = {"version": "2.0", "features": {}, "dedup_rules": {"cooldown_minutes": 60}}


class FeatureManager:
    def __init__(self, config: Config, log: Logger):
        self.config = config
        self.log = log
        self.file = config.features_file
        self.locks_dir = config.locks_dir
        self.locks_dir.mkdir(parents=True, exist_ok=True)
        if not self.file.exists():
            self._save(_DEFAULT)
            self.log.write("feature", "init", "Features file created")

    # ── I/O ────────────────────────────────────────────────────
    def load(self) -> dict:
        try:
            return json.loads(self.file.read_text())
        except (json.JSONDecodeError, FileNotFoundError):
            return json.loads(json.dumps(_DEFAULT))

    def _save(self, data: dict) -> None:
        tmp = self.file.with_suffix(".json.tmp")
        tmp.write_text(json.dumps(data, indent=2))
        os.replace(tmp, self.file)

    # ── queries ────────────────────────────────────────────────
    def exists(self, name: str) -> bool:
        return name in self.load().get("features", {})

    def status(self, name: str) -> str:
        return self.load().get("features", {}).get(name, {}).get("status", "NOT_FOUND")

    def path(self, name: str) -> str:
        return self.load().get("features", {}).get(name, {}).get("path", "")

    # ── sync from config.yaml ──────────────────────────────────
    def sync(self) -> None:
        data = self.load()
        for feat in self.config.features():
            name = feat["name"]
            if name in data["features"]:
                continue
            data["features"][name] = {
                "status": "PENDING",
                "priority": int(feat.get("priority", 999)),
                "path": feat.get("path", ""),
                "description": feat.get("description", ""),
                "pipeline_id": None,
                "claimed_at": None,
                "locked_by": None,
                "iteration": 0,
                "history": [],
            }
            self.log.write("feature", "sync",
                           f"Feature synced from config: {name} (priority {feat.get('priority')})")
        self._save(data)

    # ── guards ─────────────────────────────────────────────────
    def check_cooldown(self, name: str) -> bool:
        data = self.load()
        cooldown = int(data.get("dedup_rules", {}).get("cooldown_minutes", 60))
        feat = data.get("features", {}).get(name)
        if not feat:
            return True
        done = [h for h in feat.get("history", []) if h.get("status") == "DONE"]
        if not done:
            return True
        last = parse_iso(done[-1].get("completed_at"))
        if last is None:
            return True
        if datetime.now().astimezone() < last + timedelta(minutes=cooldown):
            remaining = int((last + timedelta(minutes=cooldown) - datetime.now().astimezone()).total_seconds() // 60)
            self.log.write("feature", "cooldown", f"Feature {name} in cooldown ({remaining}m remaining)")
            return False
        return True

    def check_lock(self, name: str) -> bool:
        lock_file = self.locks_dir / f"{name}.lock"
        if lock_file.exists():
            try:
                info = json.loads(lock_file.read_text())
            except json.JSONDecodeError:
                info = {}
            self.log.write("feature", "locked",
                           f"Feature {name} is locked by {info.get('locked_by', 'unknown')} "
                           f"since {info.get('locked_at', 'unknown')}")
            return False
        return True

    # ── lifecycle ──────────────────────────────────────────────
    def claim(self, name: str, feature_path: str, pipeline_id: str = "manual") -> bool:
        status = self.status(name)
        if status in ("LOCKED", "ACTIVE"):
            self.log.write("feature", "claim_failed", f"Feature {name} is {status} — cannot claim")
            return False
        if not self.check_cooldown(name) or not self.check_lock(name):
            return False

        ts = now_iso()
        (self.locks_dir / f"{name}.lock").write_text(
            json.dumps({"locked_by": "graph-engine", "locked_at": ts}))

        data = self.load()
        feat = data["features"].get(name)
        if feat is None:
            data["features"][name] = {
                "status": "LOCKED", "pipeline_id": pipeline_id, "path": feature_path,
                "claimed_at": ts, "locked_by": "graph-engine", "iteration": 0, "history": [],
            }
        else:
            feat.update({"status": "LOCKED", "pipeline_id": pipeline_id,
                         "claimed_at": ts, "locked_by": "graph-engine"})
        self._save(data)
        self.log.write("feature", "claimed", f"Feature {name} claimed for pipeline {pipeline_id}")
        return True

    def activate(self, name: str) -> None:
        data = self.load()
        if name in data["features"]:
            data["features"][name]["status"] = "ACTIVE"
            self._save(data)
            self.log.write("feature", "activated", f"Feature {name} activated")

    def complete(self, name: str, pipeline_id: str) -> None:
        ts = now_iso()
        data = self.load()
        feat = data["features"].get(name)
        if feat is None:
            return
        feat["status"] = "DONE"
        feat.setdefault("history", []).append(
            {"status": "DONE", "pipeline_id": pipeline_id, "completed_at": ts})
        self._save(data)
        self.release_lock(name)
        self.log.write("feature", "completed", f"Feature {name} completed (pipeline {pipeline_id})")

    def fail(self, name: str, reason: str) -> None:
        ts = now_iso()
        data = self.load()
        feat = data["features"].get(name)
        if feat is None:
            return
        feat["status"] = "FAILED"
        feat.setdefault("history", []).append(
            {"status": "FAILED", "reason": reason, "failed_at": ts})
        self._save(data)
        self.release_lock(name)
        self.log.write("feature", "failed", f"Feature {name} failed: {reason}")

    def release_lock(self, name: str) -> None:
        lock_file = self.locks_dir / f"{name}.lock"
        if lock_file.exists():
            lock_file.unlink()
            self.log.write("feature", "lock_released", f"Lock released for feature {name}")

    # ── selection (priority order) ─────────────────────────────
    def select_next(self) -> Optional[str]:
        data = self.load()
        pending = [(int(f.get("priority", 999)), name)
                   for name, f in data.get("features", {}).items()
                   if f.get("status") == "PENDING"]
        if not pending:
            self.log.write("feature", "select", "No PENDING features found")
            return None
        for _prio, name in sorted(pending):
            if self.check_cooldown(name) and self.check_lock(name):
                return name
        self.log.write("feature", "select", "No claimable PENDING features")
        return None
