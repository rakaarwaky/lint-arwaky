"""git_hook_adapter — Infrastructure adapter for Git hook management."""

from __future__ import annotations

from ..taxonomy import FilePath, SuccessStatus, BooleanVO

import os
import stat
import logging

from ..contract import IHookManagerPort

logger = logging.getLogger("mcp.infrastructure.git_hooks")


class GitHookAdapter(IHookManagerPort):
    """Manages Git hooks for the project (Infrastructure)."""

    def __init__(self, root_dir: FilePath = FilePath(value=".")):
        self.root_dir = root_dir
        self.git_dir = os.path.join(str(root_dir), ".git")

    def is_git_repo(self) -> SuccessStatus:
        return SuccessStatus(
            value=BooleanVO(
                value=os.path.exists(self.git_dir) and os.path.isdir(self.git_dir)
            )
        )

    def install_pre_commit(
        self, executable_path: FilePath = FilePath(value="auto-lint")
    ) -> SuccessStatus:
        if not self.is_git_repo().value:
            logger.error(
                f"Cannot install hook: {self.root_dir} is not a git repository."
            )
            return SuccessStatus(value=BooleanVO(value=False))

        hooks_dir = os.path.join(self.git_dir, "hooks")
        os.makedirs(hooks_dir, exist_ok=True)

        hook_path = os.path.join(hooks_dir, "pre-commit")

        # Ensure executable_path is str for the template
        exe_str = str(executable_path)

        hook_content = f"""#!/bin/bash
# Auto-Linter Pre-Commit Hook
echo " Running Auto-Linter check..."
{exe_str} check .
if [ $? -ne 0 ]; then
 echo " Linting failed. Please fix issues before committing."
 exit 1
fi
echo " Linting passed."
exit 0
"""
        try:
            with open(hook_path, "w") as f:
                f.write(hook_content)

            # Make the hook executable
            st = os.stat(hook_path)
            os.chmod(hook_path, st.st_mode | stat.S_IEXEC)

            logger.info(f"Successfully installed pre-commit hook to {hook_path}")
            return SuccessStatus(value=BooleanVO(value=True))
        except Exception as e:
            logger.error(f"Failed to install pre-commit hook: {e}")
            return SuccessStatus(value=BooleanVO(value=False))

    def uninstall_pre_commit(self) -> SuccessStatus:
        if not self.is_git_repo().value:
            return SuccessStatus(value=BooleanVO(value=False))

        hook_path = os.path.join(self.git_dir, "hooks", "pre-commit")
        if os.path.exists(hook_path):
            try:
                os.remove(hook_path)
                logger.info(f"Successfully removed pre-commit hook from {hook_path}")
                return SuccessStatus(value=BooleanVO(value=True))
            except Exception as e:
                logger.error(f"Failed to remove pre-commit hook: {e}")
                return SuccessStatus(value=BooleanVO(value=False))
        return SuccessStatus(value=BooleanVO(value=True))
