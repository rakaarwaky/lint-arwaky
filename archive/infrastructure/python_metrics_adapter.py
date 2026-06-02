"""python_metrics_adapter — Thin adapters for Python metrics (Radon, file sizes, trends)."""

from __future__ import annotations

from ..taxonomy import (
    FilePath,
    Count,
    ResponseData,
    ResponseDataList,
    MetricsError,
    ErrorMessage,
)

import logging
import os
import json


from ..contract import IMetricsProviderPort, IPathNormalizationPort

logger = logging.getLogger(__name__)


class MetricsProvider(IMetricsProviderPort):
    """Implementation of IMetricsProvider for file and quality history."""

    def __init__(
        self, path_norm: IPathNormalizationPort, history_path=".auto_lint_history"
    ):
        self.path_norm = path_norm
        self._history_path = history_path

    async def get_line_count(self, path: FilePath) -> Count | MetricsError:
        """Returns the raw line count of a file."""
        p = str(path)
        if not os.path.isfile(p):
            return MetricsError(message=ErrorMessage(value=f"File not found: {p}"))
        try:
            with open(p, "r", encoding="utf-8") as f:
                return Count(value=len(f.readlines()))
        except Exception as e:
            return MetricsError(message=ErrorMessage(value=f"Failed to read file lines: {e}"))

    async def get_history(self) -> ResponseDataList | MetricsError:
        """Reads the raw history log."""
        if not os.path.exists(self._history_path):
            return ResponseDataList(values=[])
        history = []
        try:
            with open(self._history_path, "r", encoding="utf-8") as f:
                for line in f:
                    if line.strip():
                        history.append(ResponseData(value=json.loads(line)))
        except Exception as e:
            logger.debug("Failed to read history file %s", self._history_path)
            return MetricsError(message=ErrorMessage(value=str(e)))
        return ResponseDataList(values=history)
