"""CLI setup commands — init, doctor, mcp-config."""

import json
import os
import platform
import shutil
from pathlib import Path

import click
from typing import Any

from .cli_setup_controller import (
    generate_env,
    generate_mcp_config,
    mcp_config_claude,
    mcp_config_hermes,
    mcp_config_vscode,
    register_setup_management,
)
from ..taxonomy import DirectoryPath
from ..contract import ServiceContainerAggregate


class SetupCommandsSurface:
    """Surface for environment setup CLI commands."""

    setup: Any = None
    container: ServiceContainerAggregate | None = None

    def __init__(self, cli=None, container: ServiceContainerAggregate | None = None):
        self.cli = cli
        self.container = container
        self.setup = self._build_group()
        # Register management surface first
        if container:
            register_setup_management(container)
        self._register_commands()

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Setup command registration fulfillment."""
        self.container = container
        # Register management surface first
        register_setup_management(container)
        self._register_commands()

    def _build_group(self) -> click.Group:
        """Build the setup command group."""

        @click.group()
        def setup():
            """Setup and configuration commands."""
            ...

        return setup

    def _register_commands(self) -> None:
        """Register all subcommands to the setup group."""

        @self.setup.command("init")
        def init_cmd():
            """Auto-configure auto-linter for your system."""
            self.init()

        @self.setup.command("doctor")
        def doctor_cmd():
            """Diagnose configuration and dependencies."""
            self.doctor()

        @self.setup.command("mcp-config")
        @click.option(
            "--client",
            type=click.Choice(["claude", "hermes", "vscode", "all"]),
            default="all",
        )
        def mcp_config_cmd(client):
            """Print MCP server configuration for various clients."""
            self.mcp_config(client)

        @self.setup.command("hermes")
        @click.option(
            "--remove", is_flag=True, help="Remove auto-linter from Hermes config"
        )
        def hermes_cmd(remove):
            """Auto-install auto-linter into Hermes Agent."""
            self.hermes(remove)

    def _find_binary(self, name: str) -> str | None:
        # 1. Check global PATH
        path = shutil.which(name)
        if path:
            return path
            
        # 2. Check virtualenv bin
        import sys
        vbin = os.path.dirname(sys.executable)
        path = shutil.which(name, path=vbin)
        if path:
            return path

        # 3. Check parent directories of current working directory
        curr = os.path.abspath(os.getcwd())
        while True:
            local_bin = os.path.join(curr, "node_modules", ".bin", name)
            if os.path.exists(local_bin) and os.path.isfile(local_bin):
                return local_bin
            parent = os.path.dirname(curr)
            if parent == curr:
                break
            curr = parent

        # 4. Check specific subdirectories like test-project-javascript in workspace root
        test_project_bin = os.path.join(os.getcwd(), "test-project-javascript", "node_modules", ".bin", name)
        if os.path.exists(test_project_bin) and os.path.isfile(test_project_bin):
            return test_project_bin

        # 5. Check if npx is available for JS binaries
        if name in ("eslint", "prettier", "tsc"):
            if shutil.which("npx"):
                return f"npx {name}"

        return None

    def init(self) -> None:
        """Auto-configure auto-linter for your system."""
        click.echo("Auto-Linter Setup")
        click.echo("=" * 50)

        # 1. Detect environment
        home = self._detect_environment()

        # 2. Check linters
        self._check_linters()

        # 3. Create .env
        self._create_env(home)

        # 4. Generate MCP config snippets & integrate
        mcp_vo = generate_mcp_config()
        mcp_json = (
            mcp_vo.value.value if hasattr(mcp_vo.value, "value") else mcp_vo.value
        )
        self._mcp_integration(mcp_json)

        # 5. Git pre-commit hook
        self._git_hook_integration()

        click.echo("\n" + "=" * 50)
        click.echo("Setup complete!")
        click.echo("\nUsage:")
        click.echo("  auto-lint check ./src/          # run lint")
        click.echo("  auto-linter                     # start MCP server")
        click.echo("  auto-lint doctor                # diagnose issues")

    def _detect_environment(self) -> str:
        click.echo("\n[1/4] Detecting environment...")
        home = str(Path.home())
        click.echo(f"  Python: {platform.python_version()}")
        click.echo(f"  OS: {platform.system()} {platform.release()}")
        click.echo(f"  Home: {home}")
        return home

    def _check_linters(self) -> None:
        click.echo("\n[2/4] Checking linters...")
        linters = ["ruff", "mypy", "eslint", "prettier"]
        missing_py = []
        missing_js = []
        for name in linters:
            path = self._find_binary(name)
            if path:
                click.echo(f"  {name}: {click.style('found', fg='green')} ({path})")
            else:
                click.echo(f"  {name}: {click.style('not found', fg='yellow')}")
                if name in ("ruff", "mypy"):
                    missing_py.append(name)
                else:
                    missing_js.append(name)

        if missing_py or missing_js:
            click.echo("\n  Tips to install missing linters:")
            if missing_py:
                click.echo("    Python: pip install " + " ".join(missing_py))
            if missing_js:
                click.echo("    Node.js: npm install -g " + " ".join(missing_js))

    def _create_env(self, home: str) -> None:
        click.echo("\n[3/4] Creating .env...")
        env_path = Path.cwd() / ".env"
        if env_path.exists():
            click.echo("  .env already exists — skipping")
        else:
            env_vo = generate_env(DirectoryPath(value=home))
            env_content = env_vo.value
            env_path.write_text(env_content)
            click.echo(f"  Created: {env_path}")

    def _mcp_integration(self, mcp_json) -> None:
        click.echo("\n[4/4] MCP server integration:")
        click.echo("\n  For Claude Desktop / VS Code (mcp.json):")
        click.echo("  " + "-" * 45)
        for line in json.dumps(mcp_json, indent=4).split("\n"):
            click.echo(f"  {line}")

        if click.confirm("\nWould you like to install the auto-linter MCP server directly into Claude Desktop?", default=False):
            system = platform.system()
            home_path = Path.home()
            config_path = None
            if system == "Darwin":
                config_path = home_path / "Library" / "Application Support" / "Claude" / "claude_desktop_config.json"
            elif system == "Windows":
                appdata = os.environ.get("APPDATA")
                if appdata:
                    config_path = Path(appdata) / "Claude" / "claude_desktop_config.json"
                else:
                    config_path = home_path / "AppData" / "Roaming" / "Claude" / "claude_desktop_config.json"
            else:  # Linux
                config_path = home_path / ".config" / "Claude" / "claude_desktop_config.json"

            if config_path:
                try:
                    config_path.parent.mkdir(parents=True, exist_ok=True)
                    if config_path.exists():
                        try:
                            data = json.loads(config_path.read_text())
                        except json.JSONDecodeError:
                            data = {}
                    else:
                        data = {}

                    if "mcpServers" not in data:
                        data["mcpServers"] = {}

                    data["mcpServers"]["auto-linter"] = mcp_json.get("auto-linter", mcp_json)

                    config_path.write_text(json.dumps(data, indent=4))
                    click.echo(f"  {click.style('✔', fg='green')} Successfully injected configuration into:\n  {config_path}")
                except Exception as e:
                    click.echo(f"  {click.style('✘', fg='red')} Could not write to Claude Desktop config: {e}")

    def _git_hook_integration(self) -> None:
        if Path(".git").exists():
            if click.confirm("\nWould you like to install the pre-commit git hook in this repository?", default=False):
                if self.container:
                    try:
                        container = self.container.get_for_path(os.path.abspath("."))
                        if container.hook_capability.install():
                            click.echo(f"  {click.style('✔', fg='green')} Git pre-commit hook installed successfully!")
                        else:
                            click.echo(f"  {click.style('✘', fg='red')} Failed to install git pre-commit hook.")
                    except Exception as e:
                        click.echo(f"  {click.style('✘', fg='red')} Error: {e}")

    def doctor(self) -> None:
        """Diagnose configuration and dependencies."""
        click.echo("Auto-Linter Doctor")
        click.echo("=" * 50)
        issues: list = []

        self._check_python(issues)
        self._check_core_deps(issues)
        self._check_linters(issues)
        self._check_config(issues)

        if issues:
            click.echo("\n[ISSUES FOUND]")
            for issue in issues:
                click.echo(f"  - {issue}")
        else:
            click.echo("\nAll checks passed.")

    def _check_python(self, issues) -> None:
        ver = platform.python_version_tuple()
        if int(ver[0]) < 3 or (int(ver[0]) == 3 and int(ver[1]) < 12):
            issues.append(f"Python >= 3.12 required, got {platform.python_version()}")
            click.echo(f"[!!] Python {platform.python_version()}")
        else:
            click.echo(f"[OK] Python {platform.python_version()}")

    def _check_core_deps(self, issues) -> None:
        for pkg in ["mcp", "pydantic", "click", "yaml", "dotenv"]:
            try:
                __import__(pkg.replace("-", "_"))
                click.echo(f"[OK] {pkg}")
            except ImportError:
                issues.append(f"Missing dependency: {pkg}")
                click.echo(f"[!!] {pkg} — MISSING")

    def _check_linters(self, issues: object) -> None:
        for name in ["ruff", "mypy", "eslint", "prettier"]:
            path = self._find_binary(name)
            if path:
                click.echo(f"[OK] {name} ({path})")
            else:
                click.echo(f"[--] {name} — not installed (optional)")

    def _check_config(self, issues: object) -> None:
        if Path(".env").exists():
            click.echo("[OK] .env exists")
        else:
            click.echo("[--] .env not found — run: auto-linter init")
        if Path("auto_linter.config.yaml").exists():
            click.echo("[OK] auto_linter.config.yaml exists")
        else:
            click.echo("[--] auto_linter.config.yaml not found (using defaults)")

    def mcp_config(self, client: str = "all") -> None:
        """Print MCP server configuration for various clients."""
        configs = {
            "claude": mcp_config_claude(),
            "hermes": mcp_config_hermes(),
            "vscode": mcp_config_vscode(),
        }
        for name, config_vo in configs.items():
            if client != "all" and client != name:
                continue
            click.echo(f"\n{'=' * 50}")
            click.echo(f"  {name.upper()} MCP Config")
            click.echo(f"{'=' * 50}")
            config_data = (
                config_vo.value.value
                if hasattr(config_vo.value, "value")
                else config_vo.value
            )
            for line in json.dumps(config_data, indent=2).split("\n"):
                click.echo(f"  {line}")

    def hermes(self, remove) -> None:
        """Auto-install auto-linter into Hermes Agent."""
        import subprocess  # nosec — trusted hermes/auto-linter commands
        import shutil

        click.echo("Auto-Linter + Hermes Setup")
        click.echo("=" * 50)

        # Check if hermes is installed
        hermes_bin = shutil.which("hermes")
        if not hermes_bin:
            click.echo("\n[ERROR] hermes command not found!")
            click.echo("Install Hermes Agent first:")
            click.echo("  pip install hermes-agent")
            click.echo("  or: curl -sSL https://hermes.ai/install | bash")
            return

        click.echo(f"\n  Hermes: {hermes_bin}")

        # Check if auto-linter is installed
        auto_lint_bin = shutil.which("auto-linter")
        if not auto_lint_bin:
            click.echo("[ERROR] auto-linter command not found!")
            click.echo("Install auto-linter first:")
            click.echo("  pip install auto-linter")
            return

        click.echo(f"  auto-linter: {auto_lint_bin}")

        # Remove mode
        if remove:
            click.echo("\nRemoving auto-linter from Hermes...")
            result = subprocess.run(  # nosec — trusted hermes binary from shutil.which
                [hermes_bin, "mcp", "remove", "auto-linter"],
                capture_output=True,
                text=True,
            )
            click.echo(result.stdout or result.stderr)
            click.echo("Done!")
            return

        # Detect transport (no longer used, direct mode always)
        env_vars: list = []

        # Add via hermes mcp add
        click.echo("\nAdding auto-linter to Hermes config...")
        cmd = [hermes_bin, "mcp", "add", "auto-linter", "--command", "auto-linter"]
        for e in env_vars:
            cmd.extend(["--env", e])

        result = subprocess.run(cmd, capture_output=True, text=True)  # nosec — trusted hermes binary from shutil.which

        if result.returncode == 0:
            click.echo(result.stdout or "  Added successfully!")
            click.echo("\n" + "=" * 50)
            click.echo("Done! Restart Hermes to use auto-linter:")
            click.echo("  hermes chat")
            click.echo("\nOr test the connection:")
            click.echo("  hermes mcp test auto-linter")
        else:
            click.echo(f"[ERROR] {result.stderr}")
            click.echo("\nManual fallback:")
            click.echo("  hermes mcp add auto-linter --command auto-linter")


def register_setup_commands(cli: click.Group, container: ServiceContainerAggregate) -> None:
    """Register setup commands to the main CLI."""
    inst = _get_instance()
    inst.cli = cli
    inst.register_all(container)
    cli.add_command(inst.setup)


# Lazy singleton — created on first call to avoid import-time side effects
_Instance = None


def _get_instance():
    global _Instance
    if _Instance is None:
        _Instance = SetupCommandsSurface()
    return _Instance


def get_setup():
    return _get_instance().setup


setup = None  # placeholder; use get_setup() instead
