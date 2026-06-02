"""MCP Tool: execute_command - Universal CLI executor via DesktopCommander.

Uses a dispatch pattern: execute_command validates + looks up action,
then delegates to a per-action handler method.
"""

import asyncio
import json
import os
import shutil
import sys
from typing import Any

from ..contract import ServiceContainerAggregate


# Resolve auto-lint binary path at import time
# Priority: venv bin → system PATH
_auto_lint_path = shutil.which("auto-lint", path=os.path.dirname(sys.executable))
if not _auto_lint_path:
    _auto_lint_path = shutil.which("auto-lint")
_auto_lint_cmd = _auto_lint_path or "auto-lint"  # fallback if not installed yet

# Shared state moved to mcp_desktop_client


class McpExecuteCommandSurface:
    """Surface for the execute_command MCP tool."""

    mcp: Any = None
    container: ServiceContainerAggregate | None = None
    _auto_lint_cmd: str | None = None

    def __init__(self, mcp=None):
        self.mcp = mcp
        self._auto_lint_cmd = _auto_lint_cmd

    def register_all(self, container: ServiceContainerAggregate) -> None:
        """Register the execute_command tool."""
        self.container = container
        if self.mcp:

            @self.mcp.tool()
            async def execute_command(action: str, args: dict | None = None):
                """Execute an auto-linter CLI command via MCP dispatch.

                Args:
                    action: Command name to execute (check, fix, report, etc.)
                    args: Optional command arguments as a dict

                Returns:
                    Command execution result as a dict
                """
                return await self.execute_command(container, action, args)

    # ── Dispatch ──────────────────────────────────────────────────────────

    async def execute_command(
        self, container: ServiceContainerAggregate, action: str, args: dict | None = None
    ):
        """Execute ANY CLI command via DesktopCommanderMCP.

        Validates action, resolves it against the command catalog,
        then dispatches to the appropriate handler.

        Args:
            action: Command name (check, fix, report, diff, suggest, etc.)
            args: Optional command arguments (path, format, options)

        Returns:
            JSON with command output from DesktopCommander
        """
        # Step 1: Validate input
        validation_error = self._validate_action(action)
        if validation_error:
            return json.dumps(validation_error)

        # Step 2: Resolve action name against command catalog
        target_action, resolve_error = await self._resolve_target_action(action)
        if resolve_error:
            return json.dumps(resolve_error)

        # Step 3: Dispatch to handler
        return await self._dispatch_to_handler(container, target_action, args or {})

    def _validate_action(self, action: object) -> dict | None:
        """Validate that action is a non-empty string.

        Returns an error dict if invalid, or None if valid.
        """
        if not action or not isinstance(action, str):
            return {"error": "Action must be a non-empty string"}
        return None

    async def _resolve_target_action(self, action: str) -> tuple:
        """Resolve the action name against the command catalog.

        Tries the action as-is, then with alternate separators (_ vs -).

        Returns:
            Tuple of (target_action, error_dict_or_None).
            On success: (action_name, None)
            On failure: (None, error_dict)
        """
        # Import list_commands locally to avoid circular imports
        from .mcp_command_handler import list_commands_func

        try:
            cmd_catalog = await list_commands_func()
            if isinstance(cmd_catalog, str):
                commands_dict = json.loads(cmd_catalog)
            else:
                commands_dict = dict(cmd_catalog)

            valid_actions = set(commands_dict.keys())

            # Find the actual action name from the catalog
            target_action = None
            for candidate in [
                action,
                action.replace("_", "-"),
                action.replace("-", "_"),
            ]:
                if candidate in valid_actions:
                    target_action = candidate
                    break

            if not target_action:
                return None, {
                    "error": f"Invalid action '{action}'",
                    "valid_actions_count": len(valid_actions),
                    "suggestion": "Use list_commands() for catalog",
                }
            return target_action, None
        except Exception as e:
            return None, {"error": f"Failed to validate action: {str(e)}"}

    async def _dispatch_to_handler(
        self, container: ServiceContainerAggregate, target_action: str, args: dict
    ) -> str:
        """Look up the handler for target_action in the dispatch table and call it.

        Falls back to _handle_generic if the action is not in the table.
        """
        handler_name = _DISPATCH_TABLE.get(target_action)
        if handler_name is None:
            return await self._handle_generic(container, target_action, args)
        handler = getattr(self, handler_name)
        return await handler(container, target_action, args)

    # ── Shared Execution Helpers ──────────────────────────────────────────

    async def _execute_cli(
        self, container: ServiceContainerAggregate, target_action: str, cli_cmd: list
    ) -> str:
        """Execute a pre-built CLI command array with job tracking."""
        job_id = await container.job_registry.create_job(target_action)
        try:
            proc = await asyncio.create_subprocess_exec(
                *cli_cmd,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
            )
            stdout, stderr = await asyncio.wait_for(proc.communicate(), timeout=300)
            result = {
                "stdout": stdout.decode(errors="replace"),
                "stderr": stderr.decode(errors="replace"),
                "return_code": proc.returncode,
                "protocol": "subprocess",
            }
            await container.job_registry.complete_job(job_id, result)
            result["job_id"] = str(job_id)
            return json.dumps(result)
        except asyncio.TimeoutError:
            proc.kill()
            await container.job_registry.fail_job(job_id, "Command timed out (300s)")
            return json.dumps({"error": "Command timed out", "job_id": str(job_id)})
        except Exception as e:
            await container.job_registry.fail_job(job_id, str(e))
            return json.dumps(
                {
                    "error": f"Command execution failed: {str(e)}",
                    "protocol": "subprocess",
                    "job_id": str(job_id),
                }
            )

    def _cli_base(self, target_action: str) -> list:
        """Return the base CLI command list for the given action."""
        return [self._auto_lint_cmd, target_action.replace("_", "-")]

    # ── Pattern-based handlers (shared across multiple actions) ───────────

    async def _handle_with_positional_path(self, container, target_action, args):
        """Actions where path is a positional argument (e.g. check ./src)."""
        cli_cmd = self._cli_base(target_action)
        path = args.get("path", ".")
        cli_cmd.append(path)
        return await self._execute_cli(container, target_action, cli_cmd)

    async def _handle_multi_path(self, container, target_action, args):
        """Actions that accept multiple positional paths (batch, multi-project)."""
        cli_cmd = self._cli_base(target_action)
        paths = args.get("paths")
        if isinstance(paths, list):
            cli_cmd.extend(paths)
        else:
            cli_cmd.append(str(paths) if paths else ".")
        return await self._execute_cli(container, target_action, cli_cmd)

    async def _handle_with_optional_path(self, container, target_action, args):
        """Actions that use --path as an option instead of positional (init, hooks, etc.)."""
        cli_cmd = self._cli_base(target_action)
        for key, value in args.items():
            if key == "path":
                cli_cmd.extend(["--path", str(value)])
            else:
                cli_cmd.extend([f"--{key.replace('_', '-')}", str(value)])
        return await self._execute_cli(container, target_action, cli_cmd)

    async def _handle_generic(self, container, target_action, args):
        """Fallback: pass all args as --key value options."""
        cli_cmd = self._cli_base(target_action)
        for key, value in args.items():
            cli_cmd.extend([f"--{key.replace('_', '-')}", str(value)])
        return await self._execute_cli(container, target_action, cli_cmd)

    # ── Individual action handlers ────────────────────────────────────────
    # Each handler delegates to a shared pattern-based method above.
    # Having one method per action keeps the dispatch table explicit and
    # makes it trivial to add custom logic per action in the future.

    # -- Positional path handlers --

    async def _handle_check(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_scan(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_fix(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_report(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_ci(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_watch(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_security(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_complexity(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_duplicates(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_trends(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_dependencies(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    async def _handle_suggest(self, container, action, args):
        return await self._handle_with_positional_path(container, action, args)

    # -- Multi-path handlers --

    async def _handle_batch(self, container, action, args):
        return await self._handle_multi_path(container, action, args)

    async def _handle_multi_project(self, container, action, args):
        return await self._handle_multi_path(container, action, args)

    # -- Optional-path handlers --

    async def _handle_init(self, container, action, args):
        return await self._handle_with_optional_path(container, action, args)

    async def _handle_install_hook(self, container, action, args):
        return await self._handle_with_optional_path(container, action, args)

    async def _handle_uninstall_hook(self, container, action, args):
        return await self._handle_with_optional_path(container, action, args)

    async def _handle_stats(self, container, action, args):
        return await self._handle_with_optional_path(container, action, args)

    async def _handle_ignore(self, container, action, args):
        return await self._handle_with_optional_path(container, action, args)


# ── Dispatch table: action string → handler method name ───────────────────
# Looked up at runtime via getattr(self, handler_name)
_DISPATCH_TABLE = {
    "check": "_handle_check",
    "scan": "_handle_scan",
    "fix": "_handle_fix",
    "report": "_handle_report",
    "ci": "_handle_ci",
    "batch": "_handle_batch",
    "watch": "_handle_watch",
    "security": "_handle_security",
    "complexity": "_handle_complexity",
    "duplicates": "_handle_duplicates",
    "trends": "_handle_trends",
    "dependencies": "_handle_dependencies",
    "suggest": "_handle_suggest",
    "multi-project": "_handle_multi_project",
    "init": "_handle_init",
    "install-hook": "_handle_install_hook",
    "uninstall-hook": "_handle_uninstall_hook",
    "stats": "_handle_stats",
    "ignore": "_handle_ignore",
}


def register_execute_commands(mcp, container: ServiceContainerAggregate) -> None:
    """Factory function for container-aware registration."""
    surface = McpExecuteCommandSurface(mcp)
    surface.register_all(container)
