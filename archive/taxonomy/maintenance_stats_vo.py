from pydantic import BaseModel, ConfigDict
from .file_path_vo import FilePath


class MaintenanceStatsVO(BaseModel):
    """Value Object for maintenance statistics."""

    model_config = ConfigDict(frozen=True)

    project_path: FilePath
    total_files: int
    test_files: int
    test_ratio: float
    python_files: int
