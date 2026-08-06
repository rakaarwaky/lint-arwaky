"""Graph Loop execution engine (Revision 2.0) — main loop + state handlers."""
from __future__ import annotations

import os
import re
import signal
import subprocess
import time
from pathlib import Path
from typing import Optional

from src.common import Logger, elapsed_minutes, now_iso, run
from src.config import Config
from src.dispatcher import Dispatcher
from src.features import FeatureManager
from src.notify import Notify
from src.parallel import ParallelDispatcher
from src.scanner import TriggerScanner
from src.skip import ConditionalSkip
from src.state import StateManager


class Engine:
    def __init__(self, config: Config):
        self.config = config
        self.log = Logger(config.log_file)
        self.state = StateManager(config, self.log)
        self.features = FeatureManager(config, self.log)
        self.skip = ConditionalSkip(config, self.log)
        self.dispatcher = Dispatcher(config, self.state, self.log)
        self.parallel = ParallelDispatcher(config, self.state, self.dispatcher, self.log)
        self.scanner = TriggerScanner(config, self.state, self.features, self.parallel, self.log)
        self.notify = Notify(config, self.log)
        self._stop = False
        self.pid_file = Path(__file__).resolve().parent.parent / "engine.pid"

    # ══ recovery policy (3-tier) ════════════════════════════════
    def handle_recovery(self) -> None:
        if not self.config.state_file.exists():
            self.log.write("engine", "recovery", "No state.json found — starting fresh")
            self.state.reset()
            return

        current = self.state.current_state
        if current == "IDLE":
            self.log.write("engine", "recovery", "State is IDLE — no recovery needed")
            return

        started_at = self.state.started_at
        if not started_at:
            self.log.write("engine", "recovery", "No started_at — marking FAILED")
            self.state.failed("Missing started_at on restart")
            if self.state.feature:
                self.features.fail(self.state.feature, "Missing started_at on restart")
            self.state.reset()
            return

        elapsed = elapsed_minutes(started_at)
        self.log.write("engine", "recovery", f"State={current}, elapsed={elapsed}m")

        if current == "WAITING_HUMAN":
            self.log.write("engine", "recovery", "State is WAITING_HUMAN — staying (waiting for human)")
            return
        if elapsed < self.config.resume_stale_minutes:
            self.log.write("engine", "recovery",
                           f"Elapsed < {self.config.resume_stale_minutes}m — auto-resuming from {current}")
            return
        if elapsed < self.config.global_timeout_minutes:
            self.log.write("engine", "recovery",
                           f"WARNING: Elapsed {elapsed}m (stale) — resuming from {current} with caution")
            self.notify.error("engine",
                              f"Pipeline state is stale ({elapsed}m old) — resuming with caution",
                              self.state.feature or "unknown")
            return
        self.log.write("engine", "recovery",
                       f"Elapsed >= {self.config.global_timeout_minutes}m — marking BLOCKED")
        self.state.blocked(f"Global timeout on restart ({elapsed}m)")
        if self.state.feature:
            self.features.fail(self.state.feature, f"Global timeout on restart ({elapsed}m)")
        self.state.reset()

    # ══ state handlers ══════════════════════════════════════════
    def handle_idle(self) -> None:
        self.log.write("engine", "state", "State: IDLE — scanning for triggers...")
        data = self.features.load()
        if not data.get("features"):
            self.features.sync()

        prs = self.scanner.scan_prs()
        if prs:
            number, title, branch = prs[0]
            self.log.write("engine", "state", f"Found PR #{number}: {title}")
            self.scanner.handle_pr_created(number, title, branch)
            return

        feature = self.features.select_next()
        if feature:
            self.log.write("engine", "state", f"Found pending feature: {feature}")
            feature_path = self.features.path(feature)
            pipeline_id, _cid = self.state.start_pipeline(feature)
            self.features.claim(feature, feature_path, pipeline_id)
            return

        self.log.write("engine", "state", "No triggers found — staying IDLE")

    def handle_dispatching(self) -> None:
        feature = self.state.feature
        self.log.write("engine", "state", f"State: DISPATCHING — dispatching agents for: {feature}")
        feature_path = self.features.path(feature)
        frd_path = f"{feature_path}/FRD.md"
        self.parallel.dispatch_parallel_analysis(feature, feature_path, frd_path)

    def _node_timed_out(self, node: str) -> bool:
        timeout = self.config.node_timeout(node)
        started = self.state.node_started_at(node)
        if not started:
            return False
        if elapsed_minutes(started) >= timeout:
            self.log.write("engine", "timeout",
                           f"Node {node} timed out (limit: {timeout}m)")
            return True
        return False

    def handle_analyzing(self) -> None:
        feature = self.state.feature
        self.log.write("engine", "state", f"State: ANALYZING — feature: {feature}")

        for node in ("business-analyst", "tech-lead"):
            if self.state.node_status(node) == "running" and self._node_timed_out(node):
                self.state.failed(f"{node} timeout")
                self.features.fail(feature, f"{node} timeout")
                self.state.reset()
                return

        ba = self.state.node_status("business-analyst")
        tl = self.state.node_status("tech-lead")
        if ba == "completed" and tl == "completed":
            self.log.write("engine", "state",
                           "Both Business-Analyst and Tech-Lead completed — transitioning to ARCHITECT")
            self.state.transition("ARCHITECT")
            return

        results = self.config.results_dir
        ba_report = next(iter(sorted(results.glob(f"{feature}-business-analyst.result"))), None)
        tl_report = next(iter(sorted(results.glob(f"{feature}-tech-lead.result"))), None)
        if ba_report and ba == "running":
            self.log.write("engine", "state", f"Business-Analyst report found: {ba_report}")
            self.state.complete_node("business-analyst", str(ba_report))
        if tl_report and tl == "running":
            self.log.write("engine", "state", f"Tech-Lead report found: {tl_report}")
            self.state.complete_node("tech-lead", str(tl_report))

    def handle_architect(self) -> None:
        feature = self.state.feature
        correlation_id = self.state.correlation_id
        self.log.write("engine", "state", f"State: ARCHITECT — merging reports for: {feature}")
        feature_path = self.features.path(feature)
        frd_path = f"{feature_path}/FRD.md"
        results = self.config.results_dir
        plans = self.config.plans_dir
        plans.mkdir(parents=True, exist_ok=True)

        ba_report = next(iter(sorted(results.glob(f"{feature}-business-analyst.result"))), None)
        tl_report = next(iter(sorted(results.glob(f"{feature}-tech-lead.result"))), None)

        skip_report = ""
        if ba_report is None:
            skip_report = str(self.skip.generate_skip_report(
                "Business-Analyst", feature, feature_path,
                "Business-Analyst skipped (simple fix or low complexity)"))
            ba_report = Path(skip_report)
            self.log.write("engine", "state", "Business-Analyst skipped — Skip Report generated")
        if tl_report is None:
            tl_skip = str(self.skip.generate_skip_report(
                "Tech-Lead", feature, feature_path, "Tech-Lead skipped (doc-only update)"))
            skip_report = f"{skip_report}\n{tl_skip}" if skip_report else tl_skip
            tl_report = Path(tl_skip)
            self.log.write("engine", "state", "Tech-Lead skipped — Skip Report generated")

        self.log.write("engine", "state", "Dispatching Architect to merge reports")
        prompt = self.dispatcher.architect(feature, feature_path, frd_path,
                                           str(ba_report), str(tl_report), skip_report)
        result_file = results / f"architect-{feature}.result"
        self.log.write("engine", "state", "Running Architect agent...")
        proc = self.dispatcher.spawn_agent(prompt, result_file)

        merged_plan = self._wait_for(lambda: (
            next(iter(sorted(plans.glob(f"merged-{feature}-{correlation_id}.md"))), None)
            or next(iter(sorted(plans.glob(f"merged-{feature}-*.md"))), None)
        ), max_wait=60, label="Architect")

        if merged_plan is None:
            proc.kill()
            self.log.write("engine", "state", "Architect timed out — marking FAILED")
            self.state.failed("Architect timeout")
            self.features.fail(feature, "Architect timeout")
            self.state.reset()
            return

        self.log.write("engine", "state", f"Architect completed: {merged_plan}")
        self.state.merge_complete(str(merged_plan))

    def handle_developer(self) -> None:
        feature = self.state.feature
        self.log.write("engine", "state", f"State: DEVELOPER — implementing: {feature}")
        feature_path = self.features.path(feature)
        frd_path = f"{feature_path}/FRD.md"
        plans = self.config.plans_dir

        merged_plan = next(iter(sorted(plans.glob(f"merged-{feature}-*.md"))), None)
        if merged_plan is None:
            self.log.write("engine", "state", "Merged plan not found")
            return

        self.log.write("engine", "state", "Dispatching Developer")
        prompt = self.dispatcher.developer(feature, feature_path, frd_path, str(merged_plan))
        result_file = self.config.results_dir / f"developer-{feature}.result"
        self.log.write("engine", "state", "Running Developer agent...")
        proc = self.dispatcher.spawn_agent(prompt, result_file)

        reports = self.config.reports_dir
        dev_report = self._wait_for(lambda: next(
            iter(sorted(reports.glob(f"done-{feature}-*.md"))), None),
            max_wait=120, label="Developer")

        if dev_report is None:
            proc.kill()
            self.log.write("engine", "state", "Developer timed out — marking FAILED")
            self.state.failed("Developer timeout")
            self.features.fail(feature, "Developer timeout")
            self.state.reset()
            return

        self.log.write("engine", "state", f"Developer completed: {dev_report}")
        self.state.developer_complete("PR")

    def handle_quality_analysis(self) -> None:
        feature = self.state.feature
        self.log.write("engine", "state", f"State: QUALITY-ANALYSIS — reviewing: {feature}")
        feature_path = self.features.path(feature)
        frd_path = f"{feature_path}/FRD.md"
        reports = self.config.reports_dir
        results = self.config.results_dir
        plans = self.config.plans_dir

        dev_report = next(iter(sorted(reports.glob(f"done-{feature}-*.md"))), None)
        if dev_report is None:
            self.log.write("engine", "state", "Developer report not found")
            return
        m = re.search(r"PR.*?#(\d+)", dev_report.read_text(errors="ignore"))
        if not m:
            self.log.write("engine", "state", "PR number not found in report")
            return
        pr_number = m.group(1)

        merged_plan = next(iter(sorted(plans.glob(f"merged-{feature}-*.md"))), None)
        ba_report = next(iter(sorted(results.glob(f"{feature}-business-analyst.result"))), None)
        tl_report = next(iter(sorted(results.glob(f"{feature}-tech-lead.result"))), None)
        qa_mode = self.skip.qa_mode(feature_path)

        self.log.write("engine", "state",
                       f"Dispatching Quality-Analysis for PR #{pr_number} (mode: {qa_mode})")
        prompt = self.dispatcher.quality_analysis(
            feature, pr_number, str(merged_plan or ""), frd_path,
            str(ba_report or ""), str(tl_report or ""), str(dev_report), qa_mode)
        result_file = results / f"quality-analysis-{feature}.result"
        self.log.write("engine", "state", "Running Quality-Analysis agent...")
        proc = self.dispatcher.spawn_agent(prompt, result_file)

        verdict = None
        for _ in range(60):
            time.sleep(30)
            if self._stop:
                break
            rejection_plan = next(iter(sorted(plans.glob(f"*quality-analysis-{feature}*.md"))), None)
            if rejection_plan:
                verdict = "REJECTED"
                break
            r = run(["gh", "pr", "view", pr_number, "--json", "state", "--jq", ".state"])
            if r.stdout.strip() == "MERGED":
                verdict = "APPROVED"
                break

        if verdict == "APPROVED":
            self.log.write("engine", "state", "Quality-Analysis APPROVED — pipeline complete")
            self.state.qa_approved()
            self.features.complete(feature, self.state.pipeline_id or "")
            self.notify.qa_approved(feature, pr_number)
        elif verdict == "REJECTED":
            self.log.write("engine", "state", "Quality-Analysis REJECTED — checking rejection counter")
            counter = self.state.rejection_counter + 1
            if counter >= self.config.max_rejection_loops:
                self.log.write("engine", "state",
                               f"Rejection counter reached max ({counter}/{self.config.max_rejection_loops}) — ESCALATED")
                self.state.increment_rejection()
                self.state.escalated(f"Max rejection loops reached ({counter}/{self.config.max_rejection_loops})")
                self.state.waiting_human("Max rejection loops reached — human intervention required")
                self.notify.error("quality-analysis", f"Max rejection loops reached for feature: {feature}", feature)
            else:
                self.log.write("engine", "state",
                               f"Rejection counter: {counter}/{self.config.max_rejection_loops} — re-merge")
                self.state.increment_rejection()
                self.state.increment_pipeline()   # FIX preserved: iteration++ per re-merge loop
                self.state.qa_rejected(
                    f"Quality-Analysis rejected PR (rejection {counter}/{self.config.max_rejection_loops})")
                self.notify.qa_rejected(feature, pr_number,
                                        f"Rejection {counter}/{self.config.max_rejection_loops}")
        else:
            proc.kill()
            self.log.write("engine", "state", "Quality-Analysis timed out — marking FAILED")
            self.state.failed("Quality-Analysis timeout")
            self.features.fail(feature, "Quality-Analysis timeout")
            self.state.reset()

    def handle_failed(self) -> None:
        feature = self.state.feature
        self.log.write("engine", "state", f"State: FAILED — feature: {feature}")
        iteration = self.state.pipeline_counter
        if iteration >= self.config.max_pipeline_iterations:
            self.log.write("engine", "state",
                           f"Max pipeline iterations reached ({iteration}/{self.config.max_pipeline_iterations}) — BLOCKED")
            self.state.blocked("Max pipeline iterations reached")
            return
        backoff = self.config.backoff_initial_minutes * (2 ** max(iteration - 1, 0))
        backoff = min(backoff, self.config.backoff_max_minutes)
        self.log.write("engine", "state",
                       f"Retrying after {backoff}m backoff (iteration {iteration}/{self.config.max_pipeline_iterations})")
        self.cleanup_results(feature)
        time.sleep(backoff * 60)
        self.state.transition("ANALYZING")

    def handle_blocked(self) -> None:
        feature = self.state.feature
        self.log.write("engine", "state", f"State: BLOCKED — skipping feature: {feature}")
        self.features.fail(feature, "BLOCKED — max retries exceeded")
        self.notify.error("engine", f"Feature BLOCKED: {feature} — skipping to next", feature)
        self.cleanup_results(feature)
        self.state.reset()

    def handle_timeout(self) -> None:
        feature = self.state.feature
        self.log.write("engine", "state", f"State: TIMEOUT — feature: {feature}")
        self.features.fail(feature, "Global timeout")
        self.notify.timeout("pipeline", feature, self.config.global_timeout_minutes)
        self.state.blocked("Global timeout")

    def handle_waiting_human(self) -> None:
        self.log.write("engine", "state",
                       f"State: WAITING_HUMAN — waiting for human intervention on: {self.state.feature}")

    def handle_escalated(self) -> None:
        feature = self.state.feature
        self.log.write("engine", "state", f"State: ESCALATED — escalating feature: {feature} to human")
        self.state.waiting_human("Escalated by Quality-Analysis — critical issue")
        self.notify.error("quality-analysis", f"CRITICAL escalation for feature: {feature}", feature)

    def handle_merged(self) -> None:
        feature = self.state.feature
        self.log.write("engine", "state", f"State: MERGED — pipeline complete for: {feature}")
        self.features.complete(feature, self.state.pipeline_id or "")
        self.notify.pipeline_completed(feature, self.state.pipeline_id or "")
        self.cleanup_results(feature)
        self.state.reset()

    def handle_resumed(self) -> None:
        self.log.write("engine", "state",
                       f"State: RESUMED — resuming from Architect for: {self.state.feature}")
        self.state.transition("ARCHITECT")

    # ══ helpers ═════════════════════════════════════════════════
    def _wait_for(self, finder, max_wait: int, interval: int = 30, label: str = ""):
        for i in range(max_wait):
            time.sleep(interval)
            if self._stop:
                return None
            found = finder()
            if found:
                return found
            print(f"[engine] {now_iso()} Waiting for {label}... ({i + 1}/{max_wait})")
        return None

    def cleanup_results(self, feature: str) -> None:
        results = self.config.results_dir
        generated = self.config.prompts_dir / "generated"
        self.log.write("engine", "cleanup", f"Cleaning up results for feature: {feature}")
        for pattern in (f"{feature}-*.result", f"*-{feature}.result", f"{feature}-*.pid",
                        f"business-analyst-{feature}.md", f"tech-lead-{feature}.md",
                        f"architect-{feature}.md"):
            for f in results.glob(pattern):
                f.unlink(missing_ok=True)
        for f in generated.glob(f"{feature}-*.prompt"):
            f.unlink(missing_ok=True)
        self._remove_stale(results, "*.pid", minutes=60)
        self._remove_stale(generated, "*.prompt", minutes=30)
        self.log.write("engine", "cleanup", f"Results cleaned for feature: {feature}")

    @staticmethod
    def _remove_stale(directory: Path, pattern: str, minutes: int) -> None:
        import time as _t
        if not directory.exists():
            return
        cutoff = _t.time() - minutes * 60
        for f in directory.glob(pattern):
            if f.stat().st_mtime < cutoff:
                f.unlink(missing_ok=True)

    def startup_cleanup(self) -> None:
        self.log.write("engine", "cleanup", "Startup cleanup — removing stale files")
        results = self.config.results_dir
        generated = self.config.prompts_dir / "generated"
        if results.exists():
            for pid_file in results.glob("*.pid"):
                try:
                    pid = int(pid_file.read_text().strip())
                    os.kill(pid, 0)
                except (ValueError, ProcessLookupError, PermissionError, OSError):
                    pid_file.unlink(missing_ok=True)
        for pattern in ("business-analyst-*.md", "tech-lead-*.md", "architect-*.md"):
            for f in results.glob(pattern):
                f.unlink(missing_ok=True)
        data = self.features.load()
        for name, feat in data.get("features", {}).items():
            if feat.get("status") == "DONE":
                for pattern in (f"{name}-*.result", f"*-{name}.result"):
                    for f in results.glob(pattern):
                        f.unlink(missing_ok=True)
                for f in generated.glob(f"{name}-*.prompt"):
                    f.unlink(missing_ok=True)
        self._remove_stale(generated, "*.prompt", minutes=30)
        self.log.write("engine", "cleanup", "Startup cleanup complete")

    # ══ main loop ═══════════════════════════════════════════════
    HANDLERS = {
        "IDLE": "handle_idle", "DISPATCHING": "handle_dispatching",
        "ANALYZING": "handle_analyzing", "ARCHITECT": "handle_architect",
        "DEVELOPER": "handle_developer", "QUALITY-ANALYSIS": "handle_quality_analysis",
        "MERGED": "handle_merged", "FAILED": "handle_failed",
        "BLOCKED": "handle_blocked", "TIMEOUT": "handle_timeout",
        "WAITING_HUMAN": "handle_waiting_human", "ESCALATED": "handle_escalated",
        "SKIPPED": "handle_blocked", "RESUMED": "handle_resumed",
    }

    def _handle_current(self) -> None:
        current = self.state.current_state
        handler = self.HANDLERS.get(current)
        if handler is None:
            self.log.write("engine", "engine", f"Unknown state: {current}")
            self.state.reset()
            return
        getattr(self, handler)()

    def _signal(self, *_args) -> None:
        self._stop = True

    def start(self) -> None:
        signal.signal(signal.SIGINT, self._signal)
        signal.signal(signal.SIGTERM, self._signal)
        self.log.write("engine", "engine", f"Graph Loop Engine started (PID: {os.getpid()})")
        self.log.write("engine", "engine", f"Poll interval: {self.config.poll_interval}s")
        self.pid_file.write_text(str(os.getpid()))
        try:
            self.handle_recovery()
            self.startup_cleanup()
            while not self._stop:
                current = self.state.current_state
                print(f"[engine] {now_iso()} State: {current}")
                self._handle_current()
                for _ in range(self.config.poll_interval):
                    if self._stop:
                        break
                    time.sleep(1)
        finally:
            self.log.write("engine", "engine", f"Engine stopping (PID: {os.getpid()})")
            self.pid_file.unlink(missing_ok=True)

    def once(self) -> None:
        self.handle_recovery()
        self.log.write("engine", "engine", f"Single cycle — state: {self.state.current_state}")
        self._handle_current()

    def status(self) -> str:
        return (f"State: {self.state.current_state}\n"
                f"Feature: {self.state.feature}\n"
                f"Pipeline: {self.state.pipeline_id}\n"
                f"Correlation: {self.state.correlation_id}\n"
                f"Rejection loop: {self.state.rejection_counter}/{self.config.max_rejection_loops}\n"
                f"Pipeline iteration: {self.state.pipeline_counter}/{self.config.max_pipeline_iterations}")


def cli(argv: list[str]) -> int:
    command = argv[0] if argv else ""
    engine = Engine(Config())
    if command == "start":
        engine.start()
    elif command == "once":
        engine.once()
    elif command == "recover":
        engine.handle_recovery()
    elif command == "status":
        print(engine.status())
    else:
        print("Usage: main.py engine {start|once|recover|status}")
        return 1
    return 0
