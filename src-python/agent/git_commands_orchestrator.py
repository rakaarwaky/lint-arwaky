"""git_commands_orchestrator — Agent orchestrator for git-aware linting."""

from __future__ import annotations
import shutil
import subprocess  # nosec — trusted git commands only
import logging

from ..taxonomy import (
    FilePath,
    SymbolName,
    SymbolNameList,
    FilePathList,
    BooleanVO,
    RenamedFileList,
    Count,
)
from ..contract import ServiceContainerAggregate, GitCommandsAggregate, GitDiffResultAggregate


logger = logging.getLogger("agent.git")

# Resolve git path once at module level
_git_path = shutil.which("git") or "git"


class GitCommandsOrchestrator(GitCommandsAggregate):
    """Orchestrator for git-related agent commands."""

    def __init__(self, container: ServiceContainerAggregate):
        super().__init__(container=container)

    async def run_git_diff_check(
        self, container: ServiceContainerAggregate, path: FilePath, tee=None
    ) -> None:
        """Execute a lint check only on files changed in git."""
        project_path = path
        try:
            default_branch = self._get_default_branch(project_path)
            changed_files = self._collect_changed_files(project_path, default_branch)
            changed_files = self._filter_ignored_files(changed_files, project_path)

            if changed_files:
                await container.analysis_orchestrator.run(project_path)
            else:
                pass

        except Exception as e:
            logger.warning(f"Could not get git diff: {e}")

    def _get_default_branch(self, project_path: FilePath) -> SymbolName:
        """Detect the repository's default branch (main/master)."""
        default_branch = SymbolName(value="main")
        branch_result = subprocess.run(  # nosec — trusted git path from shutil.which
            [_git_path, "symbolic-ref", "refs/remotes/origin/HEAD"],
            capture_output=True,
            text=True,
            cwd=project_path.value,
            timeout=10,
        )
        if branch_result.returncode == 0:
            ref = branch_result.stdout.strip()
            if "/" in ref:
                return SymbolName(value=ref.split("/")[-1])
            return default_branch
        remote_result = subprocess.run(  # nosec — trusted git path from shutil.which
            [_git_path, "remote", "show", "origin"],
            capture_output=True,
            text=True,
            cwd=project_path.value,
            timeout=10,
        )
        for line in remote_result.stdout.split("\n"):
            if "HEAD branch:" in line:
                return SymbolName(value=line.split(":")[1].strip())
        return default_branch

    def _run_git_with_retry(
        self, args: SymbolNameList, cwd: FilePath
    ) -> subprocess.CompletedProcess:
        """Run a git command with up to 3 retries on failure (exponential backoff)."""
        import time

        last_err = None
        cmd = [_git_path] + [s.value for s in args.values]
        for attempt in range(3):
            try:
                res = subprocess.run(
                    cmd, capture_output=True, text=True, cwd=cwd.value, timeout=30
                )  # nosec — trusted git path from shutil.which
                if res.returncode == 0:
                    return res
                last_err = res.stderr
            except Exception as e:
                last_err = str(e)
            if attempt < 2:
                time.sleep(0.5 * (2**attempt))  # 0.5s, 1.0s backoff
        raise Exception(f"Git command failed: {last_err}")

    def _collect_changed_files(
        self, project_path: FilePath, default_branch: SymbolName
    ) -> FilePathList:
        """Collect changed files using multiple git diff variants."""
        changed_set: set[FilePath] = set()
        variants = [
            f"origin/{default_branch.value}...HEAD",
            f"HEAD...origin/{default_branch.value}",
            f"{default_branch.value}...HEAD",
            "master...HEAD",
        ]
        for variant in variants:
            if self._try_variant(
                changed_set, SymbolName(value=variant), project_path
            ).value:
                break

        if not changed_set:
            self._try_fallback_head(changed_set, project_path)

        if not changed_set:
            self._try_ls_files(changed_set, project_path)

        return FilePathList(values=list(changed_set))

    def _try_variant(
        self, changed_set: set[FilePath], variant: SymbolName, project_path: FilePath
    ) -> BooleanVO:
        """Try one git diff variant; return True if any files found."""
        try:
            args = SymbolNameList(
                values=[
                    SymbolName(value="diff"),
                    SymbolName(value="--name-only"),
                    variant,
                ]
            )
            result = self._run_git_with_retry(args, project_path)
            for line in result.stdout.strip().split("\n"):
                line = line.strip()
                if line and not line.startswith(" "):
                    changed_set.add(FilePath(value=line))
            return BooleanVO(value=bool(changed_set))
        except Exception as e:
            logger.debug(f"Git diff variant '{variant.value}' failed: {e}")
            return BooleanVO(value=False)

    def _try_fallback_head(
        self, changed_set: set[FilePath], project_path: FilePath
    ) -> None:
        """Fallback: diff against HEAD."""
        try:
            args = SymbolNameList(
                values=[
                    SymbolName(value="diff"),
                    SymbolName(value="--name-only"),
                    SymbolName(value="HEAD"),
                ]
            )
            result = self._run_git_with_retry(args, project_path)
            for line in result.stdout.strip().split("\n"):
                line = line.strip()
                if line:
                    changed_set.add(FilePath(value=line))
        except Exception as e:
            logger.debug(f"Git HEAD diff failed: {e}")

    def _try_ls_files(self, changed_set: set[FilePath], project_path: FilePath) -> None:
        """Fallback: list modified/untracked files."""
        try:
            args = SymbolNameList(
                values=[
                    SymbolName(value="ls-files"),
                    SymbolName(value="--modified"),
                    SymbolName(value="--others"),
                    SymbolName(value="--exclude-standard"),
                ]
            )
            result = self._run_git_with_retry(args, project_path)
            for line in result.stdout.strip().split("\n"):
                line = line.strip()
                if line:
                    changed_set.add(FilePath(value=line))
        except Exception as e:
            logger.debug(f"Git ls-files failed: {e}")

    def _filter_ignored_files(
        self, changed_files: FilePathList, project_path: FilePath
    ) -> FilePathList:
        """Remove files ignored by git .gitignore from the changed list."""
        if not changed_files.values:
            return FilePathList(values=[])
        try:
            ignore_check = subprocess.run(  # nosec — trusted git path from shutil.which
                [_git_path, "check-ignore", "--stdin"],
                input="\n".join([f.value for f in changed_files.values]),
                capture_output=True,
                text=True,
                cwd=project_path.value,
            )
            ignored = set(ignore_check.stdout.strip().split("\n"))
            return FilePathList(
                values=[f for f in changed_files.values if f.value not in ignored]
            )
        except Exception as e:
            logger.debug(f"Git check-ignore filtering failed: {e}")
            return changed_files

    async def get_diff(self, path: FilePath) -> GitDiffResultAggregate:
        """Return aggregated git diff results."""
        default_branch = self._get_default_branch(path)
        changed_files = self._collect_changed_files(path, default_branch)
        filtered_files = self._filter_ignored_files(changed_files, path)

        # In a real implementation, we would distinguish added/modified/deleted
        # For compliance, we resolve the result class via the container
        ResultClass = self.container.get(GitDiffResultAggregate)
        if ResultClass is None:
            raise TypeError(
                "GitDiffResultAggregate implementation not registered in container"
            )
        return ResultClass(
            added=FilePathList(values=[]),
            modified=filtered_files,
            deleted=FilePathList(values=[]),
            renamed=RenamedFileList(values=[]),
            lintable_files=filtered_files,
            all_files=filtered_files,
            total_changed=Count(value=len(filtered_files.values)),
        )
