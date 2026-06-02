"""git_diff_vo — VO for git diff summary."""

from pydantic import BaseModel, ConfigDict
from .path_collection_vo import FilePathList, RenamedFileList
from .message_status_vo import Count


class GitDiffResultVO(BaseModel):
    """Aggregated diff result for surface display."""

    model_config = ConfigDict(frozen=True)

    added: FilePathList
    modified: FilePathList
    deleted: FilePathList
    renamed: RenamedFileList
    lintable_files: FilePathList
    all_files: FilePathList
    total_changed: Count
