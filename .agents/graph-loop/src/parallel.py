"""Parallel dispatcher: spawns Business-Analyst + Tech-Lead simultaneously."""
from __future__ import annotations

from pathlib import Path

from src.common import Logger
from src.config import Config
from src.dispatcher import Dispatcher
from src.state import StateManager

PARALLEL_HARD_TIMEOUT = 1800   # same as bash: timeout 1800 qwen ...


class ParallelDispatcher:
    def __init__(self, config: Config, state: StateManager,
                 dispatcher: Dispatcher, log: Logger):
        self.config = config
        self.state = state
        self.dispatcher = dispatcher
        self.log = log
        self.results_dir = config.results_dir
        self.generated_dir = config.prompts_dir / "generated"
        self.results_dir.mkdir(parents=True, exist_ok=True)
        self.generated_dir.mkdir(parents=True, exist_ok=True)

    def spawn(self, node: str, feature: str, feature_path: str, frd_path: str) -> None:
        prompt_file = self.generated_dir / f"{feature}-{node}.prompt"
        result_file = self.results_dir / f"{feature}-{node}.result"
        pid_file = self.results_dir / f"{feature}-{node}.pid"

        if node == "business-analyst":
            prompt = self.dispatcher.business_analyst(feature, feature_path, frd_path)
        elif node == "tech-lead":
            prompt = self.dispatcher.tech_lead(feature, feature_path, frd_path)
        else:
            self.log.write("parallel", "error", f"Unknown node for parallel dispatch: {node}")
            return
        prompt_file.write_text(prompt)

        # background process; timeout enforced by `timeout` like in bash
        import subprocess
        out = result_file.open("w")
        err = self.log.log_file.open("a")
        proc = subprocess.Popen(
            ["timeout", str(PARALLEL_HARD_TIMEOUT), "qwen", "-p", prompt, "-o", "text"],
            stdout=out, stderr=err, cwd=self.config.project_root)
        pid_file.write_text(str(proc.pid))     # written from parent — correct PID

        self.log.write("parallel", "spawn",
                       f"Agent {node} spawned (PID: {proc.pid}) for feature: {feature}")
        self.log.write("parallel", "spawn", f"Prompt: {prompt_file}")
        self.log.write("parallel", "spawn", f"Result: {result_file}")

    def dispatch_parallel_analysis(self, feature: str, feature_path: str, frd_path: str) -> None:
        self.log.write("parallel", "start", f"Starting parallel analysis for: {feature}")
        self.spawn("business-analyst", feature, feature_path, frd_path)
        self.spawn("tech-lead", feature, feature_path, frd_path)
        self.state.start_analyzing()
        self.log.write("parallel", "start", "Both Business-Analyst and Tech-Lead spawned")
