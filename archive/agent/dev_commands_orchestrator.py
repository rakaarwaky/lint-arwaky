"""Orchestrator for development-related domain logic."""

import os
import yaml
from pathlib import Path
from ..contract import DevCommandsAggregate, ServiceContainerAggregate
from ..taxonomy import FilePath


class DevCommandsOrchestrator(DevCommandsAggregate):
    """
    AGENT LAYER ORCHESTRATOR

    Handles domain logic for development commands, including comparisons,
    configuration management, and hook installation.
    """

    container: ServiceContainerAggregate | None = None

    def __init__(self, container: ServiceContainerAggregate):
        super().__init__(container=container)

    async def get_diff_data(self, path1, path2):
        """Get comparison data between two paths."""
        abs_path1 = os.path.abspath(path1)
        abs_path2 = os.path.abspath(path2)

        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(abs_path1)
        report1 = await container.run_analysis(FilePath(value=abs_path1))
        report2 = await container.run_analysis(FilePath(value=abs_path2))

        score_diff = report2.score.value - report1.score.value
        status = (
            "IMPROVED"
            if score_diff > 0
            else "DECLINED"
            if score_diff < 0
            else "UNCHANGED"
        )

        return {
            "version1": {"score": report1.score.value, "path": path1},
            "version2": {"score": report2.score.value, "path": path2},
            "difference": score_diff,
            "status": status,
        }

    async def get_suggestions(self, path):
        """Get suggestions based on analysis."""
        abs_path = os.path.abspath(path)
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(abs_path)
        report = await container.run_analysis(FilePath(value=abs_path))

        return {
            "score": report.score.value,
            "path": path,
            "has_issues": report.score.value < 100,
        }

    def update_ignore_rule(self, rule, remove, config_path):
        """Add or remove an ignore rule in the config file."""
        config_file = Path(config_path)
        if not config_file.exists():
            return f"Config file not found: {config_path}"

        config = yaml.safe_load(config_file.read_text())
        if "ignored_rules" not in config:
            config["ignored_rules"] = []

        msg = ""
        if remove:
            if rule in config["ignored_rules"]:
                config["ignored_rules"].remove(rule)
                msg = f"Removed '{rule}' from ignore list"
            else:
                msg = f"'{rule}' not in ignore list"
        else:
            if rule not in config["ignored_rules"]:
                config["ignored_rules"].append(rule)
                msg = f"Added '{rule}' to ignore list"
            else:
                msg = f"'{rule}' already ignored"

        config_file.write_text(yaml.dump(config, sort_keys=False))
        return msg

    def initialize_config(self, path):
        """Initialize a default configuration."""
        config_file = os.path.join(path, "auto_linter.config.yaml")
        if os.path.exists(config_file):
            return f"ALREADY_EXISTS:{config_file}"

        default_config = {
            "project_name": os.path.basename(os.path.abspath(path)),
            "thresholds": {"score": 80.0, "complexity": 10},
            "adapters": [
                "ruff",
                "mypy",
                "bandit",
                "radon",
                "pip-audit",
                "architecture",
                "duplicates",
                "trends",
            ],
            "ignored_paths": ["node_modules", ".venv", "dist", "build"],
        }

        with open(config_file, "w") as f:
            yaml.dump(default_config, f, sort_keys=False)
        return f"Initialized {config_file}"

    def install_hook(self, path):
        """Install git hook."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(os.path.abspath(path))
        return container.hook_capability.install()

    def uninstall_hook(self, path):
        """Uninstall git hook."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(os.path.abspath(path))
        return container.hook_capability.uninstall()

    # Placeholders for required abstract methods from DevCommandsAggregate
    def diff(self, path1, path2, output_format) -> None:
        pass

    def suggest(self, path, ai) -> None:
        pass

    def ignore(self, rule, remove, path) -> None:
        pass

    def config(self, action, path) -> None:
        pass

    def export(self, output_format, output) -> None:
        pass

    def init(self, path) -> None:
        pass
