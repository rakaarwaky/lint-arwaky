"""watch_result_vo — Watch mode execution result value object."""

from __future__ import annotations

from pydantic import BaseModel, ConfigDict
from .file_path_vo import FilePath
from .governance_report_entity import GovernanceReport


class WatchResult(BaseModel):
    """Result of a watch mode execution on a directory."""

    model_config = ConfigDict(frozen=True)

    file: FilePath
    """The root directory being watched."""
    report: GovernanceReport
    """The lint report for initial scan."""

    @property
    def score(self):
        """Convenience access to report score."""
        return self.report.score

    @property
    def is_passing(self):
        """Convenience access to compliance status."""
        return self.report.is_passing
