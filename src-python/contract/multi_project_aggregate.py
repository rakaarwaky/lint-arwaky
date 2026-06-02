from abc import ABC
from pydantic import BaseModel, ConfigDict
from ..taxonomy import FilePath, FilePathList, BooleanVO

class MultiProjectAggregate(BaseModel, ABC):
    """AGGREGATE: Contract for multi-project scan requests."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    paths: FilePathList | None = None
    use_retry: BooleanVO | None = None
    config_path: FilePath | None = None
