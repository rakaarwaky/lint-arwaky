"""app_config_vo — Unified configuration for the application."""

from __future__ import annotations
import os
from typing import TYPE_CHECKING
from pydantic import BaseModel, ConfigDict

from .log_suggestion_vo import BooleanVO
from .file_path_vo import DirectoryPath
from .adapter_name_vo import AdapterName
from .adapter_collection_vo import AdapterNameList

if TYPE_CHECKING:
    from .config_setting_vo import ProjectConfig, Thresholds, AdapterStatus


class AppConfig(BaseModel):
    """Unified configuration — transport, paths, and project settings."""

    model_config = ConfigDict(arbitrary_types_allowed=True, frozen=True)

    phantom_root: DirectoryPath
    project_root: DirectoryPath
    project: "ProjectConfig"

    @classmethod
    def create(
        cls,
        phantom_root: str | None = None,
        project_root: str | None = None,
        project: "ProjectConfig" | None = None,
    ) -> AppConfig:
        from .config_setting_vo import ProjectConfig

        p_root = phantom_root or os.environ.get("PHANTOM_ROOT", os.path.expanduser("~"))
        proj_root = project_root or os.environ.get("PROJECT_ROOT", os.getcwd())
        proj = project or ProjectConfig.defaults()

        return cls(
            phantom_root=DirectoryPath(value=p_root),
            project_root=DirectoryPath(value=proj_root),
            project=proj,
        )

    @property
    def thresholds(self) -> "Thresholds":
        return self.project.thresholds

    def adapter_status(self, name: str) -> "AdapterStatus":
        """Get status for a named adapter."""
        from .config_setting_vo import AdapterStatus

        for entry in self.project.adapters:
            if entry.name == name:
                return entry.status
        return AdapterStatus.NOT_INSTALLED

    def is_adapter_enabled(self, name: str | AdapterName) -> BooleanVO:
        from .config_setting_vo import AdapterStatus

        status = self.adapter_status(str(name))
        return BooleanVO(value=status == AdapterStatus.ENABLED)

    def active_adapters(self) -> AdapterNameList:
        """Names of enabled adapters."""
        return AdapterNameList(
            values=[
                AdapterName(value=e.name) for e in self.project.adapters if e.is_active
            ]
        )

    def __repr__(self) -> str:
        return (
            f"AppConfig("
            f"phantom={self.phantom_root!r}, "
            f"adapters={self.active_adapters()})"
        )
