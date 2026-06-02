from abc import ABC
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath, PatternList

class DirectoryWatchAggregate(BaseModel, ABC):
    """AGGREGATE: Contract for directory watch requests."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    path: FilePath
    recursive: bool = True
    ignore_patterns: PatternList | None = None
