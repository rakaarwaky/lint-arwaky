"""Git diff result implementation (Agent Layer)."""

from __future__ import annotations
from dataclasses import dataclass
from ..contract import GitDiffResultAggregate
from ..taxonomy import Count, FilePathList, RenamedFileList


@dataclass
class GitDiffResult(GitDiffResultAggregate):
    """Concrete implementation of GitDiffResultAggregate."""

    added: FilePathList
    modified: FilePathList
    deleted: FilePathList
    renamed: RenamedFileList
    lintable_files: FilePathList
    all_files: FilePathList
    total_changed: Count
