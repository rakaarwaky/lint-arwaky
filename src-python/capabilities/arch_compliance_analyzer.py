"""ArchitectureComplianceUseCase — The "How to Cut" logic for architectural rules."""
from __future__ import annotations

from ..taxonomy import (
    ArchitectureConfig,
    FilePath,
    FilePathList,
    LayerMapVO,
    LayerNameVO,
    LintResultList,
    PatternList,
    SymbolName,
    ContentString
)

from ..contract import (
    IFileSystemPort,
    ISourceParserPort,
    INamingCheckerProtocol,
    IInternalCheckerProtocol,
    IMetricCheckerProtocol,
    IRoleCheckerProtocol,
    IArchImportProtocol,
    ICodeQualityProtocol,
    IArchOrphanProtocol,
    IDispatchRoutingProtocol,
    IArchAnalyzerProtocol,
)


class ArchComplianceAnalyzer(IArchAnalyzerProtocol):
    """Capability that defines HOW to check architectural compliance."""

    def __init__(
        self,
        config: ArchitectureConfig,
        layer_map: LayerMapVO,
        fs: IFileSystemPort,
        parser: ISourceParserPort,
        ignored_paths: FilePathList | None = None,
        naming_checker: INamingCheckerProtocol | None = None,
        internal_checker: IInternalCheckerProtocol | None = None,
        metric_checker: IMetricCheckerProtocol | None = None,
        role_checker: IRoleCheckerProtocol | None = None,
        import_checker: IArchImportProtocol | None = None,
        quality_checker: ICodeQualityProtocol | None = None,
        orphan_detector: IArchOrphanProtocol | None = None,
        dispatch_checker: IDispatchRoutingProtocol | None = None,
    ):
        self.config = config
        self.layer_map = layer_map.values
        self.fs = fs
        self.parser = parser
        self.ignored_paths = ignored_paths or FilePathList()

        self._init_checkers(
            naming_checker=naming_checker,
            internal_checker=internal_checker,
            metric_checker=metric_checker,
            role_checker=role_checker,
            import_checker=import_checker,
            quality_checker=quality_checker,
            orphan_detector=orphan_detector,
            dispatch_checker=dispatch_checker,
        )

    def _init_checkers(
        self,
        naming_checker: INamingCheckerProtocol | None = None,
        internal_checker: IInternalCheckerProtocol | None = None,
        metric_checker: IMetricCheckerProtocol | None = None,
        role_checker: IRoleCheckerProtocol | None = None,
        import_checker: IArchImportProtocol | None = None,
        quality_checker: ICodeQualityProtocol | None = None,
        orphan_detector: IArchOrphanProtocol | None = None,
        dispatch_checker: IDispatchRoutingProtocol | None = None,
    ) -> None:
        """Initialize all checker instances with defaults."""
        from .arch_naming_checker import ArchNamingChecker
        from .arch_internal_checker import ArchInternalChecker
        from .arch_metric_checker import ArchMetricChecker
        from .arch_role_checker import ArchRoleChecker
        from .arch_import_checker import ArchImportRuleChecker
        from .code_quality_checker import CodeQualityRuleChecker
        from .dispatch_routing_checker import DispatchRoutingChecker
        from .orphan_indicator_evaluator import OrphanIndicatorEvaluator
        from .mandatory_inheritance_checker import MandatoryInheritanceChecker

        self.naming_checker = naming_checker or ArchNamingChecker()
        self.internal_checker = internal_checker or ArchInternalChecker()
        self.metric_checker = metric_checker or ArchMetricChecker()
        self.role_checker = role_checker or ArchRoleChecker()
        self.import_checker = import_checker or ArchImportRuleChecker()
        self.quality_checker = quality_checker or CodeQualityRuleChecker()
        self.orphan_detector = orphan_detector or OrphanIndicatorEvaluator()
        self.dispatch_checker = dispatch_checker or DispatchRoutingChecker()
        self.inheritance_checker = MandatoryInheritanceChecker()

        if orphan_detector is not None:
            self.orphan_detector = orphan_detector
        else:
            from .arch_orphan_analyzer import ArchOrphanAnalyzer
            self.orphan_detector = ArchOrphanAnalyzer()

        self.dispatch_checker = dispatch_checker or DispatchRoutingChecker()

        # Surface hierarchy (AES018/AES019)
        from .surface_hierarchy_checker import SurfaceHierarchyChecker
        self.surface_hierarchy_checker = SurfaceHierarchyChecker()

    def execute(self, path: FilePath) -> LintResultList:
        """Orchestrates the architectural scan logic."""
        if not self.config.enabled:
            return LintResultList()

        results = LintResultList()
        root_dir = self._resolve_root(path)

        # Capability knows WHAT to collect (python files), but FS knows HOW to walk the disk
        python_files = list(self.fs.walk(path, PatternList(values=[str(p) for p in self.ignored_paths])))
        supported_exts = self.parser.get_supported_extensions()
        ext = ".py"
        for e in supported_exts:
            if any(str(f).endswith(e) for f in python_files):
                if e in [".ts", ".tsx", ".js", ".jsx"]:
                    ext = (".ts", ".tsx", ".js", ".jsx")
                else:
                    ext = e
                break
        python_files = [f for f in python_files if str(f).endswith(ext)]
        python_files_fp = FilePathList(values=python_files)

        # Architectural Rules (The "How to Cut" Logic)
        self.naming_checker.check_file_naming(self, python_files_fp, root_dir, results)
        self.naming_checker.check_domain_suffixes(self, python_files_fp, root_dir, results)

        self.internal_checker.check_layer_internal_rules(self, python_files_fp, root_dir, results)

        self.metric_checker.check_line_counts(self, python_files_fp, root_dir, results)
        self.metric_checker.check_mandatory_class_definition(self, python_files_fp, root_dir, results)

        self.role_checker.check_agent_roles(self, python_files_fp, root_dir, results)
        self.role_checker.check_surface_roles(self, python_files_fp, root_dir, results)

        self.import_checker.check_mandatory_imports(self, python_files_fp, root_dir, results)
        self.import_checker.check_forbidden_imports(self, python_files_fp, root_dir, results)
        self.import_checker.check_legacy_import_rules(self, python_files_fp, root_dir, results)

        for f in python_files:
            layer_vo = self._detect_layer(f, root_dir)
            if not layer_vo:
                continue

            if layer_vo not in self.layer_map:
                continue

            definition = self.layer_map[layer_vo]
            
            basename = str(self.fs.get_basename(f))
            if definition.exceptions.values and basename in definition.exceptions.values:
                continue

            # 1. check_unused_mandatory_imports
            if definition.check_unused_mandatory_imports:
                self.quality_checker.check_unused_mandatory_imports(
                    FilePathList(values=[f]),
                    self.parser,
                    results,
                    violation_message=definition.check_unused_mandatory_imports_violation_message,
                    mandatory_imports=definition.mandatory_import,
                    layer_resolver=self._detect_module_layer
                )

            # 2. check_no_bypass_comments
            forbidden_words = definition.forbidden_bypass
            violation_msg = definition.forbidden_bypass_violation_message
            custom_messages = definition.forbidden_bypass_custom_messages

            self.quality_checker.check_no_bypass_comments(
                f, self.fs, results,
                forbidden_words=forbidden_words,
                violation_message=violation_msg,
                custom_messages=custom_messages
            )

        # Enforce Mandatory Inheritance (AES027): agent/capabilities/infrastructure
        # files that import contracts MUST inherit from at least one
        self.inheritance_checker.check_mandatory_inheritance(
            self, python_files_fp, root_dir, results
        )

        # Enforce Dead Inheritance Bypass (empty classes that inherit contracts)
        self.quality_checker.check_dead_inheritance_bypass(
            self, python_files_fp, root_dir, results
        )

        # Enforce Forbidden Inheritance (e.g. Contract Aggregate cannot inherit from Port)
        self.quality_checker.check_forbidden_inheritance(
            self, python_files_fp, root_dir, results
        )

        # Enforce Orphan Detection (multi-indicator)
        self.orphan_detector.check_orphans(
            self, python_files_fp, root_dir, results
        )

        # Enforce Dispatch Routing (COMMAND_CATALOG vs capability methods)
        self.dispatch_checker.check_capability_routing(
            self, python_files_fp, root_dir, results
        )

        # Enforce Surface Hierarchy (AES018/AES019)
        self.surface_hierarchy_checker.check_surface_hierarchy(
            self, python_files_fp, root_dir, results,
        )

        # Enforce MCP Tool JSON Schema Compliance (AES025)
        from .mcp_tool_schema_checker import McpToolSchemaChecker
        mcp_schema_checker = McpToolSchemaChecker()
        mcp_schema_checker.check_mcp_tool_schema(
            self, python_files_fp, root_dir, results,
        )

        return results

    def _resolve_root(self, path: FilePath) -> FilePath:
        """Logic to find the project root containing the configuration file."""
        current = path
        # If path is a file, start from its parent
        if not self.fs.is_directory(current):
            current = self.fs.get_parent(current)

        # Search upwards for config file (only language-specific configs are valid)
        for _ in range(5):  # Limit depth
            for name in [
                "auto_linter.config.python.yaml",
                "auto_linter.config.javascript.yaml",
                "auto_linter.config.rust.yaml",
            ]:
                config_path = FilePath(value=f"{str(current)}/{name}")
                if self.fs.exists(config_path):
                    return current
            parent = self.fs.get_parent(current)
            if parent == current:
                break
            current = parent

        return path  # Fallback to input path


    def _detect_layer(self, file_path: FilePath, root_dir: FilePath) -> LayerNameVO | None:
        rel_path = self.fs.get_relative_path(file_path, root_dir)
        rel = ContentString(value=str(rel_path))

        cwd = self.fs.get_cwd()
        full_rel = ContentString(value=str(self.fs.get_relative_path(file_path, cwd)))

        sorted_layers = sorted(self.layer_map.items(), key=lambda x: len(str(x[1].path)), reverse=True)

        detected_base = self._find_matching_layer(
            sorted_layers, rel, full_rel, rel_path, file_path
        )
        if detected_base is None:
            return None

        return self._resolve_specialized_layer(detected_base, file_path)

    def _find_matching_layer(
        self,
        sorted_layers,
        rel: ContentString,
        full_rel: ContentString,
        rel_path: FilePath,
        file_path: FilePath,
    ) -> LayerNameVO | None:
        """Iterate sorted layers and return the first matching layer name."""
        for name, definition in sorted_layers:
            if "(" in str(name):
                continue

            path_def = definition.path_str
            is_match = False

            if bool(definition.recursive):
                is_match = self._match_layer_recursive(rel, path_def, full_rel)
            else:
                is_match = self._match_layer_nonrecursive(rel_path, file_path, path_def)

            if is_match:
                return name
        return None

    def _match_layer_recursive(self, rel: ContentString, path_def: str, full_rel: ContentString) -> bool:
        """Check if a relative path matches a recursive layer definition."""
        rel_val = str(rel)
        if rel_val.startswith(path_def) or rel_val.startswith(path_def.split("/")[-1]):
            return True
        if str(full_rel).startswith(path_def):
            return True
        return False

    def _match_layer_nonrecursive(
        self, rel_path: FilePath, file_path: FilePath, path_def: str
    ) -> bool:
        """Check if a file path matches a non-recursive layer definition.

        Handles three matching strategies:
          1. Standard: parent directory equals layer path
          2. In-layer: analysis run inside the layer directory
          3. Absolute: fallback using absolute path
        """
        parent_dir = str(self.fs.get_parent(rel_path)).rstrip("/")
        norm_path_def = path_def.rstrip("/")

        # Case 1: Standard match (e.g. rel_path is src/file.py and norm_path_def is src)
        if parent_dir == norm_path_def:
            return True
        # Case 2: Analysis run inside the layer (e.g. rel_path is file.py and we are in src)
        if parent_dir == "." and (
            norm_path_def == "" or norm_path_def == "." or str(rel_path).endswith(norm_path_def)
        ):
            return True
        # Case 3: Fallback for absolute paths
        full_parent_dir = str(self.fs.get_parent(file_path)).rstrip("/")
        if full_parent_dir == norm_path_def:
            return True
        return False

    def _resolve_relative_import(self, current_file: FilePath, module_path: str) -> str:
        """Resolve a relative module path (e.g. ..contract) to its absolute path."""
        if not module_path.startswith("."):
            return module_path

        dots = 0
        for char in module_path:
            if char == ".":
                dots += 1
            else:
                break

        # Split current path and remove filename
        parts = str(current_file).split("/")[:-1]

        # 1 dot = same dir, 2 dots = parent, 3 dots = grandparent, etc.
        if dots > 1:
            # remove (dots - 1) levels
            parts = parts[:-(dots - 1)]

        remaining = module_path[dots:]
        if remaining:
            return ".".join(parts + remaining.split("."))
        return ".".join(parts)

    def _resolve_specialized_layer(
        self, detected_base: LayerNameVO, file_path: FilePath
    ) -> LayerNameVO:
        """Resolve a specialized layer name based on filename suffix.

        E.g. file 'user_command.py' in 'capabilities' layer becomes 'capabilities(command)'.
        """
        basename = str(self.fs.get_basename(file_path)).rsplit(".", 1)[0]
        if "_" in basename:
            suffix = basename.rsplit("_", 1)[1]
            specialized_name = LayerNameVO(value=f"{detected_base}({suffix})")
            if specialized_name in self.layer_map:
                return specialized_name

        return detected_base

    def _detect_module_layer(self, module_path: SymbolName | str) -> LayerNameVO | None:
        """Determine layer from module path (dotted string)."""
        raw = module_path.value if hasattr(module_path, "value") else str(module_path)

        # If relative import, we try to detect by parts first.
        # Ideally we'd resolve it, but this method is often called without file context.
        parts = raw.split(".")
        meaningful_parts = [p for p in parts if p]
        if not meaningful_parts:
            return None

        # 1. Direct match with layer names
        for name in self.layer_map:
            name_str = str(name)
            base_name = name_str.split("(")[0]
            if base_name in meaningful_parts:
                # Resolve specialized layer if possible
                return self._refine_module_layer(name, meaningful_parts)

        # 2. Match with definition paths
        for name, definition in self.layer_map.items():
            def_path = str(definition.path_str).strip("/")
            if def_path in raw.replace(".", "/"):
                return self._refine_module_layer(name, meaningful_parts)

        return None

    def _refine_module_layer(self, base_layer: LayerNameVO, parts: list[str]) -> LayerNameVO:
        """Try to find a more specific specialized layer from module parts."""
        base_name = str(base_layer).split("(")[0]

        # Look for the segment after the base layer name
        try:
            idx = parts.index(base_name)
            if idx + 1 < len(parts):
                next_part = parts[idx + 1]
                if "_" in next_part:
                    suffix = next_part.rsplit("_", 1)[1]
                    specialized = LayerNameVO(value=f"{base_name}({suffix})")
                    if specialized in self.layer_map:
                        return specialized
        except ValueError:
            pass

        return LayerNameVO(value=base_name)
