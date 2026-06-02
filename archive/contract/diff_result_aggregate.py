from abc import ABC
from pydantic import BaseModel, ConfigDict
from ..taxonomy import Count, FilePathList, RenamedFileList

class GitDiffResultAggregate(BaseModel, ABC):
    """AGGREGATE: Contract for Git diff results."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    added: FilePathList
    modified: FilePathList
    deleted: FilePathList
    renamed: RenamedFileList
    lintable_files: FilePathList
    all_files: FilePathList
    total_changed: Count
