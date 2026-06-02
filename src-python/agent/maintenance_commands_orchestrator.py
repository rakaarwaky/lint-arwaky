"""Orchestrator for maintenance-related domain logic."""

import logging
import os
import shutil
import subprocess  # nosec — trusted pip commands only
import sys
from pathlib import Path


from ..taxonomy import JobId, FilePath, MaintenanceStatsVO, DoctorResultVO
from ..contract import MaintenanceCommandsAggregate, ServiceContainerAggregate, JobRegistryAggregate

logger = logging.getLogger(__name__)


class MaintenanceCommandsOrchestrator(MaintenanceCommandsAggregate):
    """
    AGENT LAYER ORCHESTRATOR

    Satisfies the MaintenanceCommandsAggregate contract by providing domain logic
    for maintenance operations.
    """

    container: ServiceContainerAggregate | None = None

    def __init__(self, container: ServiceContainerAggregate):
        super().__init__(container=container)

    def stats(self, project_path: FilePath) -> MaintenanceStatsVO:
        """Logic for stats command."""
        abs_path = project_path.value
        py_files = list(Path(abs_path).rglob("*.py"))
        py_count = len(py_files)
        test_files = [f for f in py_files if f.name.startswith("test_")]
        test_count = len(test_files)
        ratio = (test_count / py_count * 100) if py_count > 0 else 0.0

        return MaintenanceStatsVO(
            project_path=project_path,
            total_files=py_count,
            test_files=test_count,
            test_ratio=ratio,
            python_files=py_count,
        )

    def clean(self) -> None:
        """Cleanup cache and temporary files."""
        cwd = Path.cwd()
        if cwd.resolve() == Path("/").resolve():
            return

        cache_dirs = [
            ".pytest_cache",
            ".mypy_cache",
            ".ruff_cache",
            "__pycache__",
            ".auto_linter_cache",
        ]
        for cache_dir in cache_dirs:
            # Recursively find all instances of these dirs
            for path in cwd.rglob(cache_dir):
                if path.is_dir():
                    try:
                        shutil.rmtree(path)
                    except Exception:
                        logger.warning("Failed to remove cache directory: %s", path)
                        pass

    def update(self) -> None:
        """Update linter adapters to latest versions."""
        adapters = ["ruff", "mypy", "bandit", "radon"]
        for adapter in adapters:
            try:
                subprocess.run(  # nosec — trusted pip install with sys.executable
                    [sys.executable, "-m", "pip", "install", "--upgrade", adapter],
                    capture_output=True,
                    text=True,
                    timeout=60,
                )
            except Exception:
                logger.warning("Failed to update adapter: %s", adapter)
                pass

    # ------------------------------------------------------------------
    # Doctor sub-checks
    # ------------------------------------------------------------------

    def _check_python_env(self):
        """Check Python version meets minimum requirements."""
        import platform

        issues = []
        py_ver = platform.python_version()
        ver_tuple = platform.python_version_tuple()
        if int(ver_tuple[0]) < 3 or (int(ver_tuple[0]) == 3 and int(ver_tuple[1]) < 12):
            issues.append(f"Python >= 3.12 required, got {py_ver}")
        return py_ver, issues

    def _check_auto_linter_installed(self):
        """Check whether the auto-linter package is installed."""
        issues = []
        result = subprocess.run(  # nosec — trusted pip show with sys.executable
            [sys.executable, "-m", "pip", "show", "auto-linter"],
            capture_output=True,
            text=True,
        )
        is_installed = result.returncode == 0
        if not is_installed:
            issues.append("auto-linter not installed in current environment")
        return is_installed, issues

    def _check_mcp_config(self):
        """Check for presence of configuration files."""
        issues = []
        config_found = []
        for cfg in [
            ".auto_linter.json",
            "auto_linter.config.yaml",
            "pyproject.toml",
            "auto_linter.config.python.yaml",
            "auto_linter.config.javascript.yaml",
            "auto_linter.config.rust.yaml",
        ]:
            if Path(cfg).exists():
                config_found.append(cfg)
        if not config_found:
            issues.append("No configuration file found")
        return config_found, issues

    def _check_linter_binaries(self):
        """Check that all required linter binaries are available."""
        issues = []
        adapters = ["ruff", "mypy", "bandit", "radon"]
        adapter_statuses = {}
        for adapter in adapters:
            found = shutil.which(adapter)
            if not found:
                venv_bin = os.path.dirname(sys.executable)
                adapter_path = os.path.join(venv_bin, adapter)
                if os.path.exists(adapter_path):
                    found = adapter_path
            adapter_statuses[adapter] = found if found else "MISSING"
            if not found:
                issues.append(f"Linter adapter '{adapter}' is not installed")
        return adapter_statuses, issues

    def doctor(self) -> DoctorResultVO:
        """Diagnose common issues by running all sub-checks."""
        issues = []

        py_ver, py_issues = self._check_python_env()
        issues.extend(py_issues)

        is_installed, install_issues = self._check_auto_linter_installed()
        issues.extend(install_issues)

        config_found, config_issues = self._check_mcp_config()
        issues.extend(config_issues)

        adapter_statuses, adapter_issues = self._check_linter_binaries()
        issues.extend(adapter_issues)

        return DoctorResultVO(
            python_version=py_ver,
            is_installed=is_installed,
            config_found=config_found,
            adapter_statuses=adapter_statuses,
            issues=issues,
            healthy=len(issues) == 0,
        )

    async def cancel(self, job_id: JobId) -> None:
        """Cancel a running lint job."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        job_registry = self.container.get(JobRegistryAggregate)
        if job_registry:
            try:
                await job_registry.cancel_job(job_id.value)
            except Exception:
                logger.warning("Failed to cancel job: %s", job_id.value)
                pass
