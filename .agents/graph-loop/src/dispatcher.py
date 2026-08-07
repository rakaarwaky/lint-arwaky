"""Prompt assembly + agent execution. Single complete text per dispatch."""
from __future__ import annotations

import subprocess
from datetime import datetime
from pathlib import Path

from src.common import Logger, now_iso
from src.config import Config
from src.state import StateManager


class DispatchError(RuntimeError):
    pass


class Dispatcher:
    def __init__(self, config: Config, state: StateManager, log: Logger):
        self.config = config
        self.state = state
        self.log = log
        self.templates_dir = config.prompts_dir / "templates"

    # ── templates ──────────────────────────────────────────────
    def _template(self, name: str) -> str:
        f = self.templates_dir / f"{name}.txt"
        if not f.is_file():
            raise DispatchError(f"Template not found: {f}")
        return f.read_text()

    @staticmethod
    def _fill(template: str, values: dict) -> str:
        for key, val in values.items():
            template = template.replace("{{" + key + "}}", str(val or ""))
        return template

    def _context(self, feature: str) -> str:
        return self.config.build_feature_context(
            feature,
            correlation_id=self.state.correlation_id,
            pipeline_iteration=self.state.pipeline_counter,
            rejection_loop=self.state.rejection_counter,
        )

    @staticmethod
    def _date() -> str:
        return datetime.now().strftime("%Y%m%d")

    # ── prompt builders (one complete text block each) ────────
    def business_analyst(self, feature: str, feature_path: str, frd_path: str) -> str:
        tpl = self._fill(self._template("business-analyst"), {
            "FEATURE": feature, "FEATURE_PATH": feature_path,
            "FRD_PATH": frd_path, "DATE": self._date()})
        return self._context(feature) + "\n" + tpl

    def tech_lead(self, feature: str, feature_path: str, frd_path: str) -> str:
        tpl = self._fill(self._template("tech-lead"), {
            "FEATURE": feature, "FEATURE_PATH": feature_path,
            "FRD_PATH": frd_path, "DATE": self._date()})
        return self._context(feature) + "\n" + tpl

    def architect(self, feature: str, feature_path: str, frd_path: str,
                  ba_report: str, tl_report: str, skip_report: str = "") -> str:
        tpl = self._fill(self._template("architect"), {
            "FEATURE": feature, "FEATURE_PATH": feature_path, "FRD_PATH": frd_path,
            "BA_REPORT": ba_report, "TL_REPORT": tl_report, "SKIP_REPORT": skip_report,
            "CORRELATION_ID": self.state.correlation_id, "DATE": self._date()})
        return self._context(feature) + "\n" + tpl

    def developer(self, feature: str, feature_path: str, frd_path: str,
                  merged_plan: str) -> str:
        tpl = self._fill(self._template("developer"), {
            "FEATURE": feature, "FEATURE_PATH": feature_path, "FRD_PATH": frd_path,
            "MERGED_PLAN": merged_plan, "DATE": self._date()})
        return self._context(feature) + "\n" + tpl

    def quality_analysis(self, feature: str, pr_number: str, merged_plan: str,
                         frd_path: str, ba_report: str, tl_report: str,
                         dev_report: str, qa_mode: str = "full-review") -> str:
        tpl = self._fill(self._template("quality-analysis"), {
            "FEATURE": feature, "PR_NUMBER": pr_number, "MERGED_PLAN": merged_plan,
            "FRD_PATH": frd_path, "BA_REPORT": ba_report, "TL_REPORT": tl_report,
            "DEV_REPORT": dev_report, "QA_MODE": qa_mode, "DATE": self._date()})
        return self._context(feature) + "\n" + tpl

    # ── agent execution (blocking) ─────────────────────────────
    def run_agent(self, node: str, prompt: str, output_file: Path,
                  timeout_minutes: int = 30) -> int:
        self.log.write("dispatch", "run", f"Running agent: {node} (timeout: {timeout_minutes}m)")
        output_file = Path(output_file)
        output_file.parent.mkdir(parents=True, exist_ok=True)
        cmd = ["timeout", str(timeout_minutes * 60), "qwen", "-p", prompt, "-o", "text"]
        try:
            with output_file.open("w") as out, self.log.log_file.open("a") as err:
                proc = subprocess.run(cmd, stdout=out, stderr=err,
                                      cwd=self.config.project_root)
            self.log.write("dispatch", "complete", f"Agent {node} finished (exit {proc.returncode})")
            return proc.returncode
        except subprocess.TimeoutExpired:
            self.log.write("dispatch", "timeout", f"Agent {node} timed out after {timeout_minutes}m")
            return 124

    # ── agent execution (background, returns Popen) ────────────
    def spawn_agent(self, prompt: str, output_file: Path) -> subprocess.Popen:
        output_file = Path(output_file)
        output_file.parent.mkdir(parents=True, exist_ok=True)
        pid_file = output_file.with_suffix(".pid")
        out = output_file.open("w")
        err = self.log.log_file.open("a")
        proc = subprocess.Popen(
            ["qwen", "-p", prompt, "-o", "text"],
            stdout=out, stderr=err, cwd=self.config.project_root)
        pid_file.write_text(str(proc.pid))
        self.log.write("dispatch", "spawn", f"Agent spawned (PID: {proc.pid}) → {output_file}")
        return proc
