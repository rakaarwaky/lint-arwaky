"""Trigger scanner: GitHub PR scan + trigger guards (debounce, bot filter,
correlation/state guard, idempotency)."""
from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Optional

from src.common import Logger, run
from src.config import Config
from src.features import FeatureManager
from src.parallel import ParallelDispatcher
from src.state import StateManager

BOT_PATTERNS = ("github-actions", "dependabot", "renovate",
                "greenkeeper", "snyk-bot", "codecov")


class TriggerScanner:
    def __init__(self, config: Config, state: StateManager,
                 features: FeatureManager, parallel: ParallelDispatcher, log: Logger):
        self.config = config
        self.state = state
        self.features = features
        self.parallel = parallel
        self.log = log
        self.debounce_file = config.locks_dir / ".debounce"
        config.locks_dir.mkdir(parents=True, exist_ok=True)

    # ── Guard: debounce ────────────────────────────────────────
    def check_debounce(self, trigger_key: str) -> bool:
        seconds = self.config.debounce_seconds
        data = {}
        if self.debounce_file.exists():
            try:
                data = json.loads(self.debounce_file.read_text())
            except json.JSONDecodeError:
                data = {}
        import time
        now = int(time.time())
        if now - int(data.get(trigger_key, 0)) < seconds:
            self.log.write("scanner", "debounce",
                           f"Trigger '{trigger_key}' debounced (< {seconds}s)")
            return False
        data[trigger_key] = now
        self.debounce_file.write_text(json.dumps(data))
        return True

    # ── Guard: bot filter ──────────────────────────────────────
    def is_bot_author(self, pr_number: str) -> bool:
        if not self.config.ignore_bot_events:
            return False
        r = run(["gh", "pr", "view", pr_number, "--json", "author", "--jq", ".author.login"])
        author = r.stdout.strip()
        if not author:
            return False
        if any(p in author for p in BOT_PATTERNS):
            self.log.write("scanner", "bot_filter",
                           f"PR #{pr_number} authored by bot ({author}) — ignoring")
            return True
        r = run(["gh", "api", f"/users/{author}", "--jq", ".type"])
        if r.stdout.strip() == "Bot":
            self.log.write("scanner", "bot_filter",
                           f"PR #{pr_number} authored by Bot type ({author}) — ignoring")
            return True
        return False

    # ── Guard: state guard (only process triggers when IDLE) ──
    def check_state_guard(self) -> bool:
        current = self.state.current_state
        if current != "IDLE":
            self.log.write("scanner", "guard", f"State is {current} (not IDLE) — queuing trigger")
            return False
        return True

    # ── scan ───────────────────────────────────────────────────
    def scan_prs(self) -> list[tuple[str, str, str]]:
        self.log.write("scanner", "scan", "Scanning GitHub for PRs with 'need review' label...")
        r = run(["gh", "pr", "list", "--label", "need review",
                 "--json", "number,title,headRefName"])
        if r.returncode != 0 or not r.stdout.strip():
            self.log.write("scanner", "scan", "No PRs found with 'need review' label")
            return []
        try:
            items = json.loads(r.stdout)
        except json.JSONDecodeError:
            return []
        if not items:
            self.log.write("scanner", "scan", "No PRs found with 'need review' label")
            return []
        return [(str(i["number"]), i["title"], i["headRefName"]) for i in items]

    @staticmethod
    def extract_feature(pr_branch: str, pr_title: str) -> str:
        m = re.match(r"^worktree-(.+)$", pr_branch)
        if m:
            return m.group(1)
        m = re.match(r"^(feature|fix|hotfix)/(.+)$", pr_branch)
        if m:
            return m.group(2)
        slug = re.sub(r"[^a-z0-9-]", "-", pr_title.lower())
        return re.sub(r"-+", "-", slug).strip("-")

    def feature_folder(self, feature: str) -> Optional[Path]:
        root = self.config.project_root
        for sub in ("crates", "modules", "packages"):
            candidate = root / sub / feature
            if candidate.is_dir():
                return candidate
        return None

    def is_feature_claimed(self, feature: str) -> bool:
        status = self.features.status(feature)
        if status in ("LOCKED", "ACTIVE"):
            return True
        if status == "DONE":
            return not self.features.check_cooldown(feature)
        return False

    # ── handle PR trigger ──────────────────────────────────────
    def handle_pr_created(self, pr_number: str, pr_title: str, pr_branch: str) -> bool:
        self.log.write("scanner", "trigger", f"Handling new PR: #{pr_number} - {pr_title}")
        if not self.check_debounce(f"pr-{pr_number}"):
            return False
        if self.is_bot_author(pr_number):
            return False
        if not self.check_state_guard():
            return False

        feature = self.extract_feature(pr_branch, pr_title)
        if not feature:
            self.log.write("scanner", "trigger", f"Could not extract feature name from PR #{pr_number}")
            return False
        if self.is_feature_claimed(feature):
            self.log.write("scanner", "trigger", f"Feature {feature} already claimed or in cooldown — skipping")
            return False
        folder = self.feature_folder(feature)
        if folder is None:
            self.log.write("scanner", "trigger", f"Feature folder not found for: {feature}")
            return False
        frd_path = folder / "FRD.md"
        if not frd_path.is_file():
            self.log.write("scanner", "trigger", f"FRD not found: {frd_path}")
            return False

        pipeline_id, correlation_id = self.state.start_pipeline(feature)
        self.features.claim(feature, str(folder), pipeline_id)

        # track correlation ID on the PR (label + comment)
        run(["gh", "pr", "edit", pr_number, "--add-label", f"corr:{correlation_id}"])
        run(["gh", "pr", "comment", pr_number, "--body",
             f"🔗 Correlation ID: `{correlation_id}` | Pipeline: `{pipeline_id}`"])

        self.log.write("scanner", "trigger",
                       f"Dispatching Business-Analyst and Tech-Lead for feature: {feature}")
        self.parallel.dispatch_parallel_analysis(feature, str(folder), str(frd_path))
        self.log.write("scanner", "trigger",
                       f"Pipeline started: {pipeline_id} (correlation: {correlation_id})")
        return True
