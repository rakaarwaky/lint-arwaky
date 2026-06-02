import re
import logging
from ..taxonomy import DirectoryPath, FilePath, SymbolName, FixResult
from ..contract import (
    ILinterAdapterPort,
    ISemanticTracerProtocol,
    IFileSystemPort,
    LintFixOrchestratorAggregate,
    ServiceContainerAggregate,
)

logger = logging.getLogger("auto_linter.agent")


class LintFixOrchestrator(LintFixOrchestratorAggregate):
    """Orchestrates automatic fixes (Agent layer)."""

    @property
    def _INTERFACE_PORT(self):
        return ILinterAdapterPort

    @property
    def _INTERFACE_FS(self):
        return IFileSystemPort

    def __init__(self, container: ServiceContainerAggregate):
        super().__init__(container=container)

    @property
    def adapters(self):
        return self.container.adapters

    @property
    def tracers(self):
        return self.container.tracers

    @property
    def fs_scanner(self) -> IFileSystemPort:
        return self.container.fs_scanner

    def _process_rename_rule(self, r, tracer: ISemanticTracerProtocol, root_dir):
        """Process a single lint result for renaming. Returns (modifications, log_line)."""
        code_str = str(r.code)
        if code_str not in ["N802", "N803", "N806", "N801"]:
            return 0, ""
        match = re.search(r"[`'](.*?)[`']", str(r.message))
        if not match:
            return 0, ""
        old_name = match.group(1)
        variant = tracer.get_variant_dict(SymbolName(value=old_name)).value
        new_name = (
            variant["pascal_case"] if code_str == "N801" else variant["snake_case"]
        )
        if old_name == new_name:
            return 0, ""
        mods = tracer.project_wide_rename(
            DirectoryPath(value=root_dir),
            SymbolName(value=old_name),
            SymbolName(value=new_name),
        )
        if isinstance(mods, int) and mods > 0:
            return (
                mods,
                f"Semantic Rename: Changed '{old_name}' -> '{new_name}' across {mods} files.\n",
            )
        return 0, ""

    async def _perform_semantic_renames(
        self,
        path_str,
        root_dir,
        tracer: ISemanticTracerProtocol,
        ruff_adapter: ILinterAdapterPort,
    ):
        """Apply semantic renaming based on Ruff naming violations. Returns (modifications, log)."""
        log = ""
        renamed = 0
        try:
            results = await ruff_adapter.scan(FilePath(value=path_str))
            for r in results:
                mods, part_log = self._process_rename_rule(r, tracer, root_dir)
                renamed += mods
                log += part_log
        except Exception as e:
            logger.warning(f"Semantic rename scan failed: {e}")
            log += "Warning: Semantic rename scan failed.\n"
        return renamed, log

    async def _apply_adapters_fixes(self, path: FilePath):
        """Apply automatic fixes via all adapters (except known analyzers). Returns log string."""
        log = ""
        for adapter in self.adapters:
            # Omit analyzers that don't support automatic fixes
            if str(adapter.name()) in ("import_violation", "architecture"):
                continue
            status = await adapter.apply_fix(path)
            if bool(status):
                log += f"[{adapter.name()}] Applied automatic fixes.\n"
            else:
                log += f"[{adapter.name()}] No fixes applied or not supported.\n"
        return log

    async def execute(self, path: FilePath) -> FixResult:
        """Executes fix application pipeline."""
        output_log = ""
        renamed_modifications = 0

        try:
            # Determine root directory for semantic renaming
            if self.fs_scanner.exists(path) and self.fs_scanner.is_directory(path):
                root_dir_vo = path
            else:
                root_dir_vo = self.fs_scanner.get_parent(path)

            root_dir = str(root_dir_vo)
            path_str = str(path)

            # Step 1: Pre-fix semantic renaming logic (Python specific)
            tracer = self.tracers.get("python")
            if tracer and path_str.endswith(".py"):
                ruff_adapter = next(
                    (a for a in self.adapters if str(a.name()) == "ruff"), None
                )
                if ruff_adapter:
                    renamed, rename_log = await self._perform_semantic_renames(
                        path_str, root_dir, tracer, ruff_adapter
                    )
                    renamed_modifications += renamed
                    output_log += rename_log

            # Step 2: Apply fixes via adapters
            output_log += await self._apply_adapters_fixes(path)

            return FixResult(output=output_log)
        except Exception as e:
            logger.error(f"Error during fix application: {e}")
            return FixResult(output="", error=str(e))
