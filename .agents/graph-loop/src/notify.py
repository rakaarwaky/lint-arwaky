"""Notifications: log, PR comment, desktop, webhook (human alerts)."""
from __future__ import annotations

import json
import shutil
import urllib.request
from typing import Optional

from src.common import Logger, now_iso, run
from src.config import Config


class Notify:
    def __init__(self, config: Config, log: Logger):
        self.config = config
        self.log = log
        self.notify_log = config.notifications_log
        self.notify_log.parent.mkdir(parents=True, exist_ok=True)

    def _nlog(self, ntype: str, message: str) -> None:
        with self.notify_log.open("a", encoding="utf-8") as f:
            f.write(f"[{now_iso()}] [{ntype}] {message}\n")

    def log(self, event: str, message: str) -> None:
        self.log.write(event.split(":")[0], event, message)
        self._nlog("log", f"{event}: {message}")

    def pr_comment(self, pr_number: Optional[str], message: str) -> None:
        if not pr_number:
            self._nlog("error", "No PR number provided")
            return
        run(["gh", "pr", "comment", pr_number, "--body", message])
        self._nlog("pr_comment", f"PR #{pr_number}: {message}")

    def desktop(self, title: str, message: str) -> None:
        if shutil.which("notify-send"):
            run(["notify-send", title, message])
            self._nlog("desktop", f"{title}: {message}")

    def webhook(self, message: str) -> None:
        channels = self.config.get("notifications.channels", []) or []
        for ch in channels:
            if ch.get("type") == "webhook" and ch.get("enabled") and ch.get("url"):
                try:
                    req = urllib.request.Request(
                        ch["url"],
                        data=json.dumps({"text": message}).encode(),
                        headers={"Content-Type": "application/json"})
                    urllib.request.urlopen(req, timeout=10)
                    self._nlog("webhook", message)
                except Exception:
                    pass

    def human_alert(self, severity: str, message: str) -> None:
        self.log(severity.lower(), message)
        self.desktop(f"Graph Loop [{severity}]", message)
        self.webhook(f"[{severity}] {message}")

    # ── pipeline events ────────────────────────────────────────
    def pipeline_started(self, feature: str, pipeline_id: str) -> None:
        self.log("pipeline_started", f"Pipeline {pipeline_id} started for feature: {feature}")

    def pipeline_completed(self, feature: str, pipeline_id: str, pr_number: str = "") -> None:
        self.log("pipeline_completed", f"Pipeline {pipeline_id} completed for feature: {feature}")
        if pr_number:
            self.pr_comment(pr_number, f"✅ Pipeline completed for feature: {feature}")

    def qa_approved(self, feature: str, pr_number: str) -> None:
        self.log("qa_approved", f"Quality-Analysis approved PR #{pr_number} for feature: {feature}")
        self.pr_comment(pr_number, "✅ Quality-Analysis APPROVED — PR merged successfully")

    def qa_rejected(self, feature: str, pr_number: str, reason: str) -> None:
        self.log("qa_rejected",
                 f"Quality-Analysis rejected PR #{pr_number} for feature: {feature} — {reason}")
        self.pr_comment(pr_number, f"❌ Quality-Analysis REJECTED — {reason}")

    def timeout(self, node: str, feature: str, timeout_minutes) -> None:
        msg = f"{node} timed out after {timeout_minutes}m for feature: {feature}"
        self.log("timeout", msg)
        self.human_alert("TIMEOUT", msg)

    def error(self, component: str, error: str, feature: str = "unknown") -> None:
        msg = f"Error in {component} for feature: {feature} — {error}"
        self.log("error", msg)
        self.human_alert("ERROR", f"[{component}] {feature}: {error}")
