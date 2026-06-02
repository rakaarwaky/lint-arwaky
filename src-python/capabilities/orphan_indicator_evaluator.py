import logging
from ..taxonomy import (
    FilePath,
    ReachabilityResult,
    InheritanceMap,
    FileDefinitionMap,
    OrphanIndicatorResult,
    Severity,
    InboundLinkMap,
    BooleanVO,
    LayerDefinition,
    LAYER_AGENT,
    LAYER_CAPABILITIES,
    LAYER_CONTRACT,
    LAYER_INFRASTRUCTURE,
    LAYER_SURFACES,
)

from ..contract import IOrphanIndicatorProtocol


logger = logging.getLogger(__name__)


class OrphanIndicatorEvaluator(IOrphanIndicatorProtocol):
    """Evaluates whether a file is an orphan based on layer-specific indicators."""

    def is_taxonomy_orphan(
        self,
        analyzer,
        f: FilePath,
        root_dir: FilePath,
        definition,
        inbound_links: InboundLinkMap,
    ) -> OrphanIndicatorResult:
        # Barrel files (__init__.py) are never orphans — they export types for the
        # system
        if str(analyzer.fs.get_basename(f)) == "__init__.py":
            return OrphanIndicatorResult(is_orphan=False)

        is_in_barrel = self._is_file_in_barrel(analyzer, f, root_dir, definition)
        path_str = str(f)

        consumers = set()
        inbound_for_file = inbound_links.mapping.get(path_str, [])
        for inbound in inbound_for_file:
            inbound_layer_vo = analyzer._detect_layer(FilePath(value=inbound), root_dir)
            if inbound_layer_vo in [
                LAYER_CONTRACT,
                LAYER_INFRASTRUCTURE,
                LAYER_CAPABILITIES,
                LAYER_SURFACES,
            ]:
                consumers.add(inbound)

        if not bool(is_in_barrel) and not consumers:
            return OrphanIndicatorResult(
                is_orphan=True,
                reason="TAXONOMY ACCOUNTABILITY: Missing from barrel AND no usage in Contract, Infra, Capability, or Surface layers.",
                severity=Severity.CRITICAL,
            )
        return OrphanIndicatorResult(is_orphan=False)

    def _has_barrel_export_for_file(self, basename):
        """Check if the given filename is a barrel export file (__init__.py)."""
        return basename == "__init__.py"

    def _check_contract_import_existence(
        self,
        analyzer,
        classes,
        inheritance_map: InheritanceMap,
        target_layers,
        root_dir: FilePath,
    ):
        """Return True if any class in *classes* has a heir in *target_layers*."""
        for cls in classes:
            heir_files = inheritance_map.mapping.get(cls, [])
            for heir_fp in heir_files:
                heir_layer_vo = analyzer._detect_layer(
                    FilePath(value=heir_fp), root_dir
                )
                if heir_layer_vo in target_layers:
                    return True
        return False

    def is_contract_orphan(
        self,
        analyzer,
        f: FilePath,
        root_dir: FilePath,
        file_definitions: FileDefinitionMap,
        inheritance_map: InheritanceMap,
    ) -> OrphanIndicatorResult:
        path_str = str(f)
        basename = str(analyzer.fs.get_basename(f))
        # Barrel files don't define classes directly; skip orphan check for them
        if self._has_barrel_export_for_file(basename):
            return OrphanIndicatorResult(is_orphan=False)
        classes = file_definitions.mapping.get(path_str, [])

        # Utility-only files (no classes defined) don't need heirs
        if not classes:
            return OrphanIndicatorResult(is_orphan=False)

        target_layers = []
        if basename.endswith("_port.py"):
            target_layers = [LAYER_INFRASTRUCTURE]
        elif basename.endswith("_protocol.py"):
            target_layers = [LAYER_CAPABILITIES]
        elif basename.endswith("_aggregate.py"):
            target_layers = [LAYER_AGENT]
        else:
            target_layers = [LAYER_INFRASTRUCTURE, LAYER_CAPABILITIES, LAYER_AGENT]

        has_heirs = self._check_contract_import_existence(
            analyzer,
            classes,
            inheritance_map,
            target_layers,
            root_dir,
        )

        if not has_heirs:
            target_desc = " / ".join(target_layers).title()
            return OrphanIndicatorResult(
                is_orphan=True,
                reason=f"CONTRACT ORPHAN: File '{basename}' has no grounded heirs. No implementations found in {target_desc}.",
                severity=Severity.HIGH,
            )
        return OrphanIndicatorResult(is_orphan=False)

    def is_infra_cap_orphan(
        self, analyzer, f: FilePath, root_dir: FilePath, alive_files: ReachabilityResult
    ) -> OrphanIndicatorResult:
        is_wired = self._is_wired_in_container(analyzer, f, root_dir)
        is_reachable = str(f) in alive_files

        if not bool(is_wired) and not is_reachable:
            return OrphanIndicatorResult(
                is_orphan=True,
                reason="REGISTRATION & EXECUTION: Not registered in Agent Container AND unreachable from any Surface 'button press'.",
                severity=Severity.CRITICAL,
            )
        elif not bool(is_wired):
            return OrphanIndicatorResult(
                is_orphan=True,
                reason="REGISTRATION ORPHAN: Reachable from logic but not registered in any Agent Container for Dependency Injection.",
                severity=Severity.MEDIUM,
            )
        return OrphanIndicatorResult(is_orphan=False)

    def is_agent_orphan(
        self, analyzer, f: FilePath, root_dir: FilePath
    ) -> OrphanIndicatorResult:
        is_wired = self._is_wired_in_container(analyzer, f, root_dir)
        if not bool(is_wired):
            return OrphanIndicatorResult(
                is_orphan=True,
                reason="AGENT ORPHAN: Internal agent component is not registered in the DI Container.",
                severity=Severity.HIGH,
            )
        return OrphanIndicatorResult(is_orphan=False)

    def is_surface_orphan(
        self,
        f: FilePath,
        alive_files: ReachabilityResult,
        definition: LayerDefinition | None = None,
    ) -> OrphanIndicatorResult:
        # Check if this file is a designated entry point (always alive)
        basename = str(f).split("/")[-1]
        if definition:
            epts = (
                definition.orphan_entry_points
                if hasattr(definition, "orphan_entry_points")
                else None
            )
            if epts and basename in epts:
                return OrphanIndicatorResult(is_orphan=False)
        # No definition — fall through to generic orphan check
        if str(f) not in alive_files:
            return OrphanIndicatorResult(
                is_orphan=True,
                reason="SURFACE ORPHAN: Surface component is not reachable from any main entry point (CLI/MCP).",
                severity=Severity.HIGH,
            )
        return OrphanIndicatorResult(is_orphan=False)

    def is_generic_orphan(
        self,
        f: FilePath,
        alive_files: ReachabilityResult,
        inbound_links: InboundLinkMap,
    ) -> OrphanIndicatorResult:
        path_str = str(f)
        if path_str not in alive_files and not inbound_links.mapping.get(path_str):
            return OrphanIndicatorResult(
                is_orphan=True,
                reason="GENERIC ORPHAN: Unreachable from entry points and has zero inbound imports.",
                severity=Severity.HIGH,
            )
        return OrphanIndicatorResult(is_orphan=False)

    def _is_file_in_barrel(
        self, analyzer, file_path: FilePath, root_dir: FilePath, definition
    ) -> BooleanVO:
        if not definition.barrel_completeness:
            return BooleanVO(value=True)

        path_def = str(definition.path_str)
        barrel_path = FilePath(value=f"{str(root_dir)}/{path_def}/__init__.py")

        if not analyzer.fs.exists(barrel_path):
            return BooleanVO(value=False)

        stem = str(analyzer.fs.get_basename(file_path)).replace(".py", "")
        imports = analyzer.parser.extract_imports(barrel_path)
        for imp in imports.values:
            if stem in str(imp.module.value).split("."):
                return BooleanVO(value=True)
        return BooleanVO(value=False)

    def _scan_container_file_names(self, analyzer, root_dir: FilePath):
        """Walk the agent container directory and return container-related .py files."""
        container_dir_str = f"{str(root_dir)}/src/{LAYER_AGENT}"
        if not analyzer.fs.exists(FilePath(value=container_dir_str)):
            if str(root_dir).endswith("/src"):
                container_dir_str = f"{str(root_dir)}/{LAYER_AGENT}"
            else:
                return []

        container_files = []
        for f in analyzer.fs.walk(FilePath(value=container_dir_str)):
            f_str = str(f)
            if "container" in f_str.lower() and f_str.endswith(".py"):
                container_files.append(f)
        return container_files

    def _check_module_pattern_present(
        self, analyzer, container_file: FilePath, stem, class_names
    ):
        """Check if a module stem or class name appears in a container file's imports or content."""
        imports = analyzer.parser.extract_imports(container_file)
        for imp in imports.values:
            module_parts = str(imp.module.value).split(".")
            if stem in module_parts:
                return True
            if imp.name and str(imp.name.value) in class_names:
                return True

        try:
            content = str(analyzer.fs.read_text(container_file))
            if stem in content:
                return True
            for cls in class_names:
                if cls in content:
                    return True
        except Exception:
            logger.debug("Failed to read container file content: %s", container_file)
        return False

    def _is_wired_in_container(
        self, analyzer, file_path: FilePath, root_dir: FilePath
    ) -> BooleanVO:
        stem = str(analyzer.fs.get_basename(file_path)).replace(".py", "")
        class_names = []
        try:
            bases_map = analyzer.parser.get_class_bases_map(file_path)
            class_names = [str(name) for name in bases_map.keys()]
        except Exception:
            logger.debug("Failed to get class bases for %s", file_path)

        container_files = self._scan_container_file_names(analyzer, root_dir)
        if not container_files:
            return BooleanVO(value=False)

        for container_file in container_files:
            if self._check_module_pattern_present(
                analyzer, container_file, stem, class_names
            ):
                return BooleanVO(value=True)
        return BooleanVO(value=False)
