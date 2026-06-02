"""Development CLI commands: diff, suggest, ignore, config, export."""

import click
import json
from typing import Any
import os
import shutil
import subprocess  # nosec — trusted editor/git commands only
from pathlib import Path

from ..taxonomy import FilePath, GovernanceReport
from ..contract import ServiceContainerAggregate
from ..contract import run_async


class DevCommandsSurface:
    """Surface for development-related CLI commands."""

    cli: Any = None
    container: ServiceContainerAggregate | None = None

    def __init__(self, cli, container: ServiceContainerAggregate | None = None):
        self.cli = cli
        self.container = container

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Register all dev commands."""
        self.container = container

        @self.cli.command("diff")
        @click.argument("path1", type=click.Path(exists=True))
        @click.argument("path2", type=click.Path(exists=True))
        @click.option(
            "--output-format", type=click.Choice(["text", "json"]), default="text"
        )
        def diff_cmd(path1, path2, output_format):
            """Compare lint results between two versions."""
            self.diff(path1, path2, output_format)

        @self.cli.command("suggest")
        @click.argument("path", type=click.Path(exists=True))
        @click.option("--ai", is_flag=True, help="Use AI-powered suggestions")
        def suggest_cmd(path, ai):
            """AI-powered fix suggestions."""
            self.suggest(path, ai)

        @self.cli.command("ignore")
        @click.argument("rule")
        @click.option("--remove", is_flag=True, help="Remove rule from ignore list")
        @click.option(
            "--path", default="auto_linter.config.yaml", help="Config file path"
        )
        def ignore_cmd(rule, remove, path):
            """Manage ignore rules in configuration."""
            self.ignore(rule, remove, path)

        @self.cli.command("config")
        @click.argument("action", type=click.Choice(["show", "edit", "reset"]))
        @click.option(
            "--path", default="auto_linter.config.yaml", help="Config file path"
        )
        def config_cmd(action, path):
            """Edit configuration settings."""
            self.config(action, path)

        @self.cli.command("export")
        @click.argument("output_format", type=click.Choice(["sarif", "junit", "json"]))
        @click.option("--output", "-o", help="Output file path")
        def export_cmd(output_format, output):
            """Export lint reports in different formats."""
            self.export(output_format, output)

        @self.cli.command("import")
        @click.argument("config_file", type=click.Path(exists=True))
        def import_cmd(config_file):
            """Import configurations from a JSON/YAML file."""
            self.import_config(config_file)

        @self.cli.command("init")
        @click.option("--path", default=".", help="Project root directory")
        @click.option(
            "--non-interactive",
            is_flag=True,
            help="Run in non-interactive mode with default settings",
        )
        def init_cmd(path, non_interactive):
            """Initialize a new Auto-Linter configuration."""
            self.init(path, non_interactive)

        @self.cli.command("install-hook")
        @click.option("--path", default=".", help="Project root directory")
        def install_hook_cmd(path):
            """Install git pre-commit hook."""
            self.install_hook(path)

        @self.cli.command("uninstall-hook")
        @click.option("--path", default=".", help="Project root directory")
        def uninstall_hook_cmd(path):
            """Remove git pre-commit hook."""
            self.uninstall_hook(path)

    @click.argument("path1", type=click.Path(exists=True))
    @click.argument("path2", type=click.Path(exists=True))
    @click.option(
        "--output-format", type=click.Choice(["text", "json"]), default="text"
    )
    def diff(self, path1, path2, output_format) -> None:
        """Compare lint results between two versions."""

        async def _diff() -> None:
            abs_path1 = os.path.abspath(path1)
            abs_path2 = os.path.abspath(path2)

            # Use path1 for container discovery by default
            if self.container is None:
                raise RuntimeError("Container not initialized")
            container = self.container.get_for_path(abs_path1)
            abs_path1 = os.path.abspath(path1)
            abs_path2 = os.path.abspath(path2)
            report1: GovernanceReport = await container.run_analysis(
                FilePath(value=abs_path1)
            )
            report2: GovernanceReport = await container.run_analysis(
                FilePath(value=abs_path2)
            )

            score_diff = report2.score.value - report1.score.value
            status = (
                " IMPROVED"
                if score_diff > 0
                else " DECLINED"
                if score_diff < 0
                else " UNCHANGED"
            )

            if output_format == "json":
                click.echo(
                    json.dumps(
                        {
                            "version1": {"score": report1.score.value, "path": path1},
                            "version2": {"score": report2.score.value, "path": path2},
                            "difference": score_diff,
                            "status": status,
                        }
                    )
                )
            else:
                click.echo("Version Comparison:")
                click.echo(f" {path1}: {report1.score.value:.1f}")
                click.echo(f" {path2}: {report2.score.value:.1f}")
                click.echo(f" Difference: {score_diff:+.1f} {status}")

        run_async(_diff())

    @click.argument("path", type=click.Path(exists=True))
    @click.option("--ai", is_flag=True, help="Use AI-powered suggestions")
    def suggest(self, path, ai) -> None:
        """AI-powered fix suggestions."""

        async def _suggest() -> None:
            abs_path = os.path.abspath(path)
            if self.container is None:
                raise RuntimeError("Container not initialized")
            container = self.container.get_for_path(abs_path)
            click.echo(f" Analyzing {path} for suggestions...")
            abs_path = os.path.abspath(path)
            report: GovernanceReport = await container.run_analysis(
                FilePath(value=abs_path)
            )

            click.echo(f"\nSuggestions for {path}:")

            if report.score.value < 100:
                click.echo(
                    f"  Architecture compliance score is {report.score.value:.1f}/100"
                )
                click.echo(f" → Run 'auto-lint fix {path}' to apply safe fixes")
                click.echo(" → Review remaining issues manually")
            else:
                click.echo("  Code is at 100.0 architecture compliance score!")

            if ai:
                click.echo("\n  AI suggestions: Coming soon (requires LLM integration)")

        run_async(_suggest())

    @click.argument("rule")
    @click.option("--remove", is_flag=True, help="Remove rule from ignore list")
    @click.option("--path", default="auto_linter.config.yaml", help="Config file path")
    def ignore(self, rule, remove, path) -> None:
        """Manage ignore rules in configuration."""
        if path == "auto_linter.config.yaml" and not os.path.exists(path) and self.container:
            container = self.container.get_for_path(os.path.abspath("."))
            discovered = container.config_discovery.find_yaml_config()
            if discovered and not isinstance(discovered, Exception):
                config_file = Path(discovered.value)
            else:
                config_file = Path(path)
        else:
            config_file = Path(path)

        if not config_file.exists():
            click.echo(f" Config file not found: {path}")
            click.echo("Run 'auto-lint setup init' first")
            return

        import yaml

        config = yaml.safe_load(config_file.read_text())

        if "ignored_rules" not in config:
            config["ignored_rules"] = []

        if remove:
            if rule in config["ignored_rules"]:
                config["ignored_rules"].remove(rule)
                click.echo(f" Removed '{rule}' from ignore list")
            else:
                click.echo(f" '{rule}' not in ignore list")
        else:
            if rule not in config["ignored_rules"]:
                config["ignored_rules"].append(rule)
                click.echo(f" Added '{rule}' to ignore list")
            else:
                click.echo(f" '{rule}' already ignored")

        config_file.write_text(yaml.dump(config, sort_keys=False))

    @click.argument("action", type=click.Choice(["show", "edit", "reset"]))
    @click.option("--path", default="auto_linter.config.yaml", help="Config file path")
    def config(self, action, path) -> None:
        """Edit configuration settings."""
        if path == "auto_linter.config.yaml" and not os.path.exists(path) and self.container:
            container = self.container.get_for_path(os.path.abspath("."))
            discovered = container.config_discovery.find_yaml_config()
            if discovered and not isinstance(discovered, Exception):
                config_file = Path(discovered.value)
            else:
                config_file = Path(path)
        else:
            config_file = Path(path)

        if action == "show":
            if not config_file.exists():
                click.echo(" Config not found. Run 'auto-lint setup init'")
                return
            click.echo(config_file.read_text())

        elif action == "edit":
            editor = os.environ.get("EDITOR", "nano")
            _editor_path = shutil.which(editor) or editor
            subprocess.run([_editor_path, str(config_file)])  # nosec — trusted editor path from shutil.which
            click.echo(" Config saved")

        elif action == "reset":
            if click.confirm("Reset config to defaults?"):
                default_config = {
                    "project_name": "auto-linter",
                    "thresholds": {"score": 80.0, "complexity": 10},
                    "adapters": [
                        "ruff",
                        "mypy",
                        "bandit",
                        "radon",
                        "pip-audit",
                        "architecture",
                    ],
                    "ignored_paths": ["node_modules", ".venv", "dist", "build"],
                }
                import yaml

                config_file.write_text(yaml.dump(default_config, sort_keys=False))
                click.echo(" Config reset to defaults")

    @click.argument("output_format", type=click.Choice(["sarif", "junit", "json"]))
    @click.option("--output", "-o", help="Output file path")
    def export(self, output_format, output) -> None:
        """Export lint reports in different formats."""
        path = os.getcwd()

        async def _export() -> None:
            abs_path = os.path.abspath(path)
            if self.container is None:
                raise RuntimeError("Container not initialized")
            container = self.container.get_for_path(abs_path)
            abs_path = os.path.abspath(path)
            report: GovernanceReport = await container.run_analysis(
                FilePath(value=abs_path)
            )

            if output_format == "sarif":
                result = container.report_formatter.to_sarif(report).value
            elif output_format == "junit":
                result = container.report_formatter.to_junit(report).value
            else:
                # json: delegate to report formatter
                data = container.report_formatter.report_to_dict(report)
                result = json.dumps(data, indent=2)

            if output:
                Path(output).write_text(str(result))
                click.echo(f" Exported to {output}")
            else:
                click.echo(str(result))

        run_async(_export())

    @click.option("--path", default=".", help="Project root directory")
    @click.option("--non-interactive", is_flag=True, help="Use default configuration without interaction")
    def init(self, path, non_interactive=False) -> None:
        """Initialize a new Auto-Linter configuration."""
        # 1. Project auto-detection
        detected_stack, has_python, has_js = self._detect_stack(path)

        # 2. Prompt user or use defaults
        stack_choice, arch_choice, score_threshold, complexity_threshold, import_gitignore, auto_hook = self._prompt_interactive_or_defaults(detected_stack, non_interactive)

        # Determine config file name based on stack choice
        if stack_choice == 1:
            config_name = "auto_linter.config.python.yaml"
        elif stack_choice == 2:
            config_name = "auto_linter.config.javascript.yaml"
        else:
            config_name = "auto_linter.config.rust.yaml"

        config_file = os.path.join(path, config_name)
        if os.path.exists(config_file) and not non_interactive:
            if not click.confirm(f"{config_file} already exists. Overwrite?"):
                return

        # 3. Configure adapters
        adapters = self._configure_adapters(stack_choice)

        # 4. Ignored paths
        ignored_paths = ["node_modules", ".venv", "dist", "build"]
        if import_gitignore:
            self._read_gitignore(path, ignored_paths)

        # 5. Build config structure
        config = {
            "project_name": os.path.basename(os.path.abspath(path)),
            "thresholds": {
                "score": score_threshold,
                "complexity": complexity_threshold
            },
            "adapters": adapters,
            "ignored_paths": ignored_paths
        }

        # 6. Write configuration
        if arch_choice == 1:
            self._write_ddd_template(path, config_file, stack_choice, score_threshold, complexity_threshold, auto_hook)
        else:
            self._write_standard_mvc_or_flat(config_file, config, arch_choice, auto_hook, path)

    def _detect_stack(self, path) -> tuple[str, bool, bool]:
        python_sigs = ["pyproject.toml", "requirements.txt", "setup.py", "setup.cfg", "Pipfile", "poetry.lock", "uv.lock"]
        js_sigs = ["package.json", "tsconfig.json", "pnpm-lock.yaml", "yarn.lock", "package-lock.json"]

        has_python = any(os.path.exists(os.path.join(path, sig)) for sig in python_sigs)
        has_js = any(os.path.exists(os.path.join(path, sig)) for sig in js_sigs)

        detected_stack = "Multi-Language"
        if has_python and not has_js:
            detected_stack = "Python"
        elif has_js and not has_python:
            detected_stack = "JavaScript/TypeScript"
        elif not has_python and not has_js:
            detected_stack = "Generic"
        return detected_stack, has_python, has_js

    def _prompt_interactive_or_defaults(self, detected_stack, non_interactive) -> tuple[int, int, float, int, bool, bool]:
        if not non_interactive:
            click.echo("\n" + "=" * 50)
            click.echo(" Auto-Linter Setup Wizard")
            click.echo("=" * 50)
            click.echo(f"Detected stack: {click.style(detected_stack, fg='green', bold=True)}")

            # Stack selection
            click.echo("\n[1/5] Select your Technology Stack:")
            click.echo("  1. Python (ruff, mypy, bandit, radon, pip-audit)")
            click.echo("  2. JavaScript/TypeScript (eslint, prettier, tsc)")
            click.echo("  3. Multi-Language (All standard linters)")
            stack_choice = click.prompt("Choose option (1-3)", type=int, default=1 if detected_stack == "Python" else 2 if detected_stack == "JavaScript/TypeScript" else 3)

            # Architectural template
            click.echo("\n[2/5] Select Architectural Enforcement Style:")
            click.echo("  1. Clean / DDD Architecture (Strict layer boundaries & suffix rules)")
            click.echo("  2. Standard MVC / Layered (controllers, services, repositories)")
            click.echo("  3. Flat / Minimal (Complexity & standard linting only)")
            arch_choice = click.prompt("Choose option (1-3)", type=int, default=1)

            # Thresholds
            click.echo("\n[3/5] Configure Compliance Thresholds:")
            score_threshold = click.prompt("Minimum quality compliance score (0.0 - 100.0)", type=float, default=80.0)
            complexity_threshold = click.prompt("Maximum allowed cyclomatic complexity", type=int, default=10)

            # Exclusions
            click.echo("\n[4/5] Exclusions:")
            import_gitignore = click.confirm("Auto-import patterns from .gitignore?", default=True)

            # Pre-commit hook
            click.echo("\n[5/5] Git Integration:")
            auto_hook = click.confirm("Automatically install pre-commit git hook?", default=False)
        else:
            # Non-interactive defaults
            stack_choice = 1 if detected_stack == "Python" else 2 if detected_stack == "JavaScript/TypeScript" else 3
            arch_choice = 1
            score_threshold = 80.0
            complexity_threshold = 10
            import_gitignore = True
            auto_hook = False

        return stack_choice, arch_choice, score_threshold, complexity_threshold, import_gitignore, auto_hook

    def _configure_adapters(self, stack_choice) -> list[str]:
        adapters = []
        if stack_choice in (1, 3):
            adapters.extend(["ruff", "mypy", "bandit", "radon", "pip-audit"])
        if stack_choice in (2, 3):
            adapters.extend(["eslint", "prettier", "tsc"])
        return adapters

    def _read_gitignore(self, path, ignored_paths) -> None:
        gitignore_path = os.path.join(path, ".gitignore")
        if os.path.exists(gitignore_path):
            try:
                with open(gitignore_path, "r") as gf:
                    for line in gf:
                        line = line.strip()
                        if line and not line.startswith("#"):
                            clean_line = line.rstrip("/")
                            if clean_line and clean_line not in ignored_paths:
                                ignored_paths.append(clean_line)
            except (IOError, OSError):
                pass

    def _write_ddd_template(self, path, config_file, stack_choice, score_threshold, complexity_threshold, auto_hook) -> None:
        from .cli_config_templates import PYTHON_CONFIG_TEMPLATE, JS_CONFIG_TEMPLATE, RUST_CONFIG_TEMPLATE
        if stack_choice == 1:
            template_str = PYTHON_CONFIG_TEMPLATE
        elif stack_choice == 2:
            template_str = JS_CONFIG_TEMPLATE
        else:
            template_str = RUST_CONFIG_TEMPLATE

        try:
            import re
            project_name = os.path.basename(os.path.abspath(path))
            
            # Replace the score threshold in the YAML template (retaining comments)
            modified_template = re.sub(
                r"(\s+score:\s*)[\d\.]+",
                rf"\g<1>{score_threshold}",
                template_str
            )
            # Replace the complexity threshold in the YAML template (retaining comments)
            modified_template = re.sub(
                r"(\s+complexity:\s*)\d+",
                rf"\g<1>{complexity_threshold}",
                modified_template
            )
            
            # Prepend the auto-detected project name to the template string
            final_content = f"project_name: \"{project_name}\"\n" + modified_template
            
            with open(config_file, "w") as f:
                f.write(final_content)
            click.echo(f"\n{click.style('✔', fg='green')} Initialized {config_file} with full production DDD rules!")
            
            if auto_hook:
                click.echo("Installing git pre-commit hook...")
                self.install_hook(path)
        except Exception as e:
            click.echo(f"  Warning: failed to load production templates, falling back to minimal: {e}")

    def _write_standard_mvc_or_flat(self, config_file, config, arch_choice, auto_hook, path) -> None:
        if arch_choice == 2:
            config["adapters"].append("architecture")
            config["architecture"] = {
                "enabled": True,
                "layers": {
                    "controllers": {
                        "path": "src-python/controllers",
                        "recursive": True,
                        "suffix": [{"strict": ["controller"]}]
                    },
                    "services": {
                        "path": "src-python/services",
                        "recursive": True,
                        "suffix": [{"strict": ["service"]}]
                    },
                    "repositories": {
                        "path": "src-python/repositories",
                        "recursive": True,
                        "suffix": [{"strict": ["repository"]}]
                    },
                    "models": {
                        "path": "src-python/models",
                        "recursive": True,
                        "suffix": [{"strict": ["model"]}]
                    }
                }
            }

        import yaml
        with open(config_file, "w") as f:
            yaml.dump(config, f, sort_keys=False)
        click.echo(f"\n{click.style('✔', fg='green')} Initialized {config_file}")

        if auto_hook:
            click.echo("Installing git pre-commit hook...")
            self.install_hook(path)

    @click.argument("config_file", type=click.Path(exists=True))
    def import_config(self, config_file) -> None:
        """Import configurations from a JSON/YAML file."""
        config_path = Path(config_file)
        if config_path.suffix in (".yaml", ".yml"):
            import yaml

            config = yaml.safe_load(config_path.read_text())
        elif config_path.suffix == ".json":
            config = json.loads(config_path.read_text())
        else:
            click.echo(
                f" Unsupported format: {config_path.suffix}. Use .yaml, .yml, or .json"
            )
            return

        # Determine appropriate language-specific config filename
        adapters = config.get("adapters", [])
        adapter_names = []
        for a in adapters:
            if isinstance(a, dict):
                adapter_names.append(a.get("name", ""))
            elif isinstance(a, str):
                adapter_names.append(a)

        if any(name in ["ruff", "mypy", "bandit", "radon"] for name in adapter_names):
            target_name = "auto_linter.config.python.yaml"
        elif any(name in ["eslint", "prettier", "tsc"] for name in adapter_names):
            target_name = "auto_linter.config.javascript.yaml"
        else:
            target_name = "auto_linter.config.rust.yaml"

        target = Path(target_name)
        import yaml

        target.write_text(yaml.dump(config, sort_keys=False))
        click.echo(f" Imported config from {config_file} → {target}")

    @click.option("--path", default=".", help="Project root directory")
    def install_hook(self, path) -> None:
        """Install git pre-commit hook."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(os.path.abspath(path))
        if container.hook_capability.install():
            click.echo(" Pre-commit hook installed successfully.")
        else:
            click.echo(" Failed to install pre-commit hook.")

    @click.option("--path", default=".", help="Project root directory")
    def uninstall_hook(self, path) -> None:
        """Remove git pre-commit hook."""
        if self.container is None:
            raise RuntimeError("Container not initialized")
        container = self.container.get_for_path(os.path.abspath(path))
        if container.hook_capability.uninstall():
            click.echo(" Pre-commit hook removed successfully.")
        else:
            click.echo(" Failed to remove pre-commit hook.")


def register_dev_commands(cli, container: ServiceContainerAggregate) -> None:
    """Factory function to maintain backward compatibility."""
    surface = DevCommandsSurface(cli)
    surface.register_all(container)
