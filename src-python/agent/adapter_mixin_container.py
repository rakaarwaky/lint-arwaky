from __future__ import annotations
import os
import sys
from typing import TYPE_CHECKING

from ..contract import AdapterContainerAggregate, ServiceContainerAggregate
from ..taxonomy import FilePath
from ..infrastructure import (
    RuffAdapter,
    MyPyAdapter,
    BanditAdapter,
    ComplexityAdapter,
    DependencyAdapter,
    DuplicateAdapter,
    TrendsAdapter,
    PrettierAdapter,
    TSCAdapter,
    ESLintAdapter,
    ArchComplianceAdapter,
    RustLinterAdapter,
)

if TYPE_CHECKING:
    pass


class AdapterMixinContainer(ServiceContainerAggregate, AdapterContainerAggregate):
    """Logic for initializing and managing linter adapters."""

    def _init_adapters(self):
        vbin = os.path.dirname(sys.executable)
        self.venv_bin = vbin
        self.adapters = []
        config = self.config

        # 1. Base Linter Adapters
        base_adapters = [
            RuffAdapter(
                executor=self.executor,
                path_norm=self.path_normalization,
                bin_path=FilePath(value=vbin),
            ),
            MyPyAdapter(
                executor=self.executor,
                path_norm=self.path_normalization,
                bin_path=FilePath(value=vbin),
            ),
            BanditAdapter(
                executor=self.executor,
                path_norm=self.path_normalization,
                bin_path=FilePath(value=vbin),
            ),
            ComplexityAdapter(
                executor=self.executor,
                path_norm=self.path_normalization,
                bin_path=FilePath(value=vbin),
                threshold=config.thresholds.complexity,
            ),
            DependencyAdapter(
                executor=self.executor,
                path_norm=self.path_normalization,
                bin_path=FilePath(value=vbin),
            ),
            DuplicateAdapter(
                executor=self.executor,
                path_norm=self.path_normalization,
                bin_path=FilePath(value=vbin),
            ),
            TrendsAdapter(executor=self.executor, path_norm=self.path_normalization),
            PrettierAdapter(executor=self.executor, path_norm=self.path_normalization),
            TSCAdapter(executor=self.executor, path_norm=self.path_normalization),
            ESLintAdapter(executor=self.executor, path_norm=self.path_normalization),
            RustLinterAdapter(executor=self.executor, path_norm=self.path_normalization),
        ]

        # Filter by config
        self.adapters = [
            a for a in base_adapters if config.is_adapter_enabled(str(a.name()))
        ]

        # 2. Architecture Compliance (Special Case)
        if config.project.architecture and config.project.architecture.enabled:
            self.arch_compliance_adapter = ArchComplianceAdapter(self.arch_compliance_coordinator)
            self.adapters.append(self.arch_compliance_adapter)
