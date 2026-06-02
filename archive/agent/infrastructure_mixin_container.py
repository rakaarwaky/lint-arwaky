from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

from ..contract import InfrastructureContainerAggregate, ServiceContainerAggregate
from ..infrastructure import (
    SourceParserOrchestrator,
    ConfigDiscoveryProvider,
    ConfigJSONProvider,
    ConfigParserProvider,
    ConfigValidationProvider,
    ConfigYamlProvider,
    GitDiffScanner,
    GitHookAdapter,
    JavascriptNamingProvider,
    JSFlowTracer,
    JSScopeProvider,
    JSScopeTracer,
    JSTracer,
    MemoryJobRegistryAdapter,
    MetricsProvider,
    OSFileSystemAdapter,
    PathNormalizationProvider,
    PluginSystemProvider,
    PythonNamingVariantProvider,
    PythonTracer,
    StdioClient,
    SyncHttpProvider,
    WatchServiceProvider,
)
from ..taxonomy import DirectoryPath, FilePath

if TYPE_CHECKING:
    from .dependency_injection_container import Container


class InfrastructureMixinContainer(ServiceContainerAggregate, InfrastructureContainerAggregate):
    """Logic for initializing base infrastructure and configuration."""

    def _init_infrastructure(self: Container) -> None:
        # 1. Configuration Bootstrap
        root_vo = DirectoryPath(value=self.project_root)
        self.config_discovery = ConfigDiscoveryProvider()
        self.config_parser = ConfigParserProvider()
        self.config_json = ConfigJSONProvider()
        self.config_yaml = ConfigYamlProvider()
        self.config_validator = ConfigValidationProvider(
            discovery=self.config_discovery,
            parser=self.config_parser,
            json_provider=self.config_json,
        )

        # Load config — search upward from project root.
        # Only language-specific configs are supported:
        #   auto_linter.config.{python,javascript,rust}.yaml
        found_env = self.config_discovery.find_env_file(start=root_vo)
        found_yaml = self.config_discovery.find_yaml_config(start=root_vo)
        if found_yaml is None:
            auto_linter_root = Path(__file__).resolve().parent.parent.parent
            fallback_path = next(
                (
                    auto_linter_root / c
                    for c in [
                        "auto_linter.config.python.yaml",
                        "auto_linter.config.javascript.yaml",
                        "auto_linter.config.rust.yaml",
                    ]
                    if (auto_linter_root / c).is_file()
                ),
                None,
            )
            if fallback_path is not None:
                found_yaml = FilePath(value=str(fallback_path))
        self.config = self.config_validator.load_config(
            env_path=str(found_env) if found_env else None,
            yaml_path=str(found_yaml) if found_yaml else None,
        )

        # 2. Base Services
        self.fs_scanner = OSFileSystemAdapter()
        self.source_parser = SourceParserOrchestrator()
        self.http_provider = SyncHttpProvider()
        self.path_normalization = PathNormalizationProvider()
        self.executor = StdioClient()
        self.git_diff_scanner = GitDiffScanner(executor=self.executor)
        self.git_hook_manager = GitHookAdapter(root_dir=root_vo)
        self.plugin_manager = PluginSystemProvider()
        self.naming_variant_provider = PythonNamingVariantProvider()

        self.stdio_client = self.executor

        # 4. Tracers
        self.python_tracer = PythonTracer()
        self.js_tracer = JSTracer()
        self.js_naming_provider = JavascriptNamingProvider()
        self.memory_job_registry = MemoryJobRegistryAdapter()
        self.js_scope_provider = JSScopeProvider()
        self.js_scope_tracer = JSScopeTracer()
        self.js_flow_tracer = JSFlowTracer(scope_provider=self.js_scope_provider)
        self.tracers = {"python": self.python_tracer, "js": self.js_tracer}

        # 5. Specialized Infrastructure
        self.watch_service_provider = WatchServiceProvider(
            event_callback=lambda f: (
                self.pipeline.process_watch_event(f) if self.pipeline else None
            )
        )
        self.metrics_provider = MetricsProvider(path_norm=self.path_normalization)
