"""git_diff_scanner — Git-aware file change detection for linting only modified files."""

from __future__ import annotations
import logging
import shutil
from dataclasses import dataclass
from pathlib import Path
from ..taxonomy import (
    FilePath,
    FilePathList,
    GitRef,
    RenamedFile,
    RenamedFileList,
    DirectoryPath,
)
from ..contract import IScannerProviderPort, ICommandExecutorPort
from ..contract import run_async


logger = logging.getLogger(__name__)


@dataclass
class DiffResult:
    """Result of a git diff scan."""

    added: FilePathList
    modified: FilePathList
    deleted: FilePathList
    renamed: RenamedFileList

    @property
    def all_files(self) -> FilePathList:
        """All changed files (added + modified + new names of renamed)."""
        combined = list(self.added) + list(self.modified)
        combined.extend(r.new_path for r in self.renamed)
        return FilePathList(values=combined)


class GitDiffScanner(IScannerProviderPort):
    """Implementation of IScannerProvider using Git."""

    def __init__(
        self,
        root: DirectoryPath | None = None,
        executor: ICommandExecutorPort | None = None,
    ):
        self.root = root
        self.executor = executor

    def scan_directory(self, path: DirectoryPath) -> FilePathList:
        """Scan directory using git diff."""
        diff = self.get_changed_files(root=FilePath(value=str(path)))
        if diff is None:
            return FilePathList()
        return diff.all_files

    def get_ignored_files(self) -> FilePathList:
        """Return list of files ignored by git."""
        # For now, return empty as git diff naturally ignores ignored files
        return FilePathList()

    def get_changed_files(
        self,
        base: GitRef = GitRef(value="HEAD"),
        target: GitRef = GitRef(value="working"),
        root: FilePath | None = None,
    ) -> DiffResult | None:
        """Get list of changed files between base and target."""
        root_path = Path(root.value) if root else Path.cwd()
        git_bin = shutil.which("git") or "git"

        # If no executor, we return empty to avoid security violations
        if not self.executor:
            return None

        async def _run():
            try:
                # Check if it's a git repo
                await self.executor.execute_command(
                    [git_bin, "rev-parse", "--git-dir"], str(root_path)
                )
            except Exception:
                logger.warning(
                    "Git repo check failed or command not supported", exc_info=True
                )
                return None

            base_ref = str(base)
            target_ref = str(target)

            if target_ref == "working":
                cmd = [git_bin, "diff", "--name-status", base_ref]
            elif target_ref == "staged":
                cmd = [git_bin, "diff", "--name-status", "--cached"]
            else:
                cmd = [git_bin, "diff", "--name-status", base_ref, target_ref]

            try:
                result = await self.executor.execute_command(cmd, str(root_path))
                if result.get("returncode") == 0:
                    return self._parse_diff_output(result.get("stdout", ""))

                # Fallback to general diff
                fallback_cmd = [git_bin, "diff", "--name-status"]
                res2 = await self.executor.execute_command(fallback_cmd, str(root_path))
                return self._parse_diff_output(res2.get("stdout", ""))
            except Exception:
                logger.warning("Git diff command failed", exc_info=True)
                return DiffResult(
                    added=FilePathList(),
                    modified=FilePathList(),
                    deleted=FilePathList(),
                    renamed=RenamedFileList(),
                )

        # Synchronous wrapper for the async executor call
        return run_async(_run())

    def _parse_diff_output(self, stdout) -> DiffResult:
        """Helper to parse 'git diff --name-status' output."""
        added_files = []
        modified_files = []
        deleted_files = []
        renamed_files = []

        for line in stdout.strip().splitlines():
            if not line:
                continue
            parts = line.split("\t")
            status = parts[0]
            if status == "A":
                added_files.append(FilePath(value=parts[1]))
            elif status == "M":
                modified_files.append(FilePath(value=parts[1]))
            elif status == "D":
                deleted_files.append(FilePath(value=parts[1]))
            elif status.startswith("R"):
                renamed_files.append(
                    RenamedFile(
                        old_path=FilePath(value=parts[1]),
                        new_path=FilePath(value=parts[2]),
                    )
                )

        return DiffResult(
            added=FilePathList(values=added_files),
            modified=FilePathList(values=modified_files),
            deleted=FilePathList(values=deleted_files),
            renamed=RenamedFileList(values=renamed_files),
        )

    def filter_by_extensions(
        self, files: FilePathList, extensions=(".py", ".js", ".ts", ".jsx", ".tsx")
    ) -> FilePathList:
        """Filter files by allowed extensions."""
        filtered = [f for f in files if any(str(f).endswith(ext) for ext in extensions)]
        return FilePathList(values=filtered)

    def get_changed_files_convenience(
        self,
        base: GitRef = GitRef(value="HEAD"),
        target: GitRef = GitRef(value="working"),
        root: FilePath | None = None,
    ) -> DiffResult | None:
        """Convenience method to get changed files."""
        return self.get_changed_files(base=base, target=target, root=root)

    def get_changed_files_filtered(
        self,
        base: GitRef = GitRef(value="HEAD"),
        root: FilePath | None = None,
        extensions=(".py", ".js", ".ts", ".jsx", ".tsx"),
    ) -> FilePathList:
        """Get changed files filtered by extensions."""
        diff = self.get_changed_files(base=base, root=root)
        if diff is None:
            return FilePathList()
        return self.filter_by_extensions(diff.all_files, extensions)
