import fnmatch
import logging

from ..taxonomy import (
    FilePath,
    FilePathList,
    FilePathSet,
    ModuleName,
    ImportGraph,
    ReachabilityResult,
    InboundLinkMap,
    InheritanceMap,
    FileDefinitionMap,
    GraphAnalysisContext,
    ModuleToFileMap,
)
from ..contract import IOrphanGraphProtocol


logger = logging.getLogger(__name__)


class OrphanGraphResolver(IOrphanGraphProtocol):
    """Helper to resolve import graphs and reachability for orphan detection."""

    def _discover_files(
        self, analyzer, full_project_files: FilePathList, root_dir: FilePath
    ) -> ModuleToFileMap:
        """Pre-map modules to files to avoid FS hits during import resolution."""
        module_to_file = {}
        for f in full_project_files.values:
            abs_path = str(f)
            rel_path = analyzer.fs.get_relative_path(f, root_dir)
            mod_name = str(rel_path).replace(".py", "").replace("/", ".")
            if mod_name.endswith(".__init__"):
                mod_name = mod_name[:-9]
            for prefix in ("src.", "lib.", "app.", "core."):
                if mod_name.startswith(prefix):
                    mod_name = mod_name[len(prefix) :]
                    break
            module_to_file[mod_name] = abs_path
        return ModuleToFileMap(mapping=module_to_file)

    def _collect_import_map(
        self,
        analyzer,
        full_project_files: FilePathList,
        root_dir: FilePath,
        module_to_file: ModuleToFileMap,
    ) -> tuple[ImportGraph, InboundLinkMap]:
        """Extract imports and resolve them to file targets, building import + inbound maps."""
        import_map: dict[str, list[str]] = {}
        inbound_map: dict[str, list[str]] = {
            str(f): [] for f in full_project_files.values
        }

        for f in full_project_files.values:
            abs_path = str(f)
            import_map[abs_path] = []
            try:
                imports = analyzer.parser.extract_imports(f)
                for imp in imports.values:
                    mod_val = str(imp.module.value)
                    target_fp = self.resolve_import_to_file(
                        analyzer, f, ModuleName(value=mod_val), root_dir, module_to_file
                    )
                    if target_fp:
                        target_path = str(target_fp)
                        import_map[abs_path].append(target_path)
                        if target_path in inbound_map:
                            inbound_map[target_path].append(abs_path)
            except Exception:
                logger.debug("Failed to extract imports from %s", f)
                pass

        return ImportGraph(mapping=import_map), InboundLinkMap(mapping=inbound_map)

    def _collect_inheritance_chain(
        self, analyzer, full_project_files: FilePathList
    ) -> tuple[InheritanceMap, FileDefinitionMap]:
        """Extract class definitions and inheritance relationships from project files."""
        inheritance_mapping: dict = {}
        file_defs = {}

        for f in full_project_files.values:
            abs_path = str(f)
            try:
                bases_map = analyzer.parser.get_class_bases_map(f)
                file_defs[abs_path] = [str(k) for k in bases_map.keys()]
                for _, bases in bases_map.items():
                    for base in bases:
                        base_str = str(base)
                        if base_str not in inheritance_mapping:
                            inheritance_mapping[base_str] = []
                        inheritance_mapping[base_str].append(abs_path)
            except Exception:
                logger.debug("Failed to collect inheritance chain from %s", f)
                pass

        return InheritanceMap(mapping=inheritance_mapping), FileDefinitionMap(
            mapping=file_defs
        )

    def _collect_orphans_from_graph(
        self,
        analyzer,
        full_project_files: FilePathList,
        root_dir: FilePath,
        module_to_file: ModuleToFileMap,
    ) -> tuple[ImportGraph, InboundLinkMap, InheritanceMap, FileDefinitionMap]:
        """Extract imports, resolve targets, collect inheritance and definitions."""
        import_graph, inbound_map = self._collect_import_map(
            analyzer, full_project_files, root_dir, module_to_file
        )
        inheritance_map, file_defs = self._collect_inheritance_chain(
            analyzer, full_project_files
        )
        return import_graph, inbound_map, inheritance_map, file_defs

    def build_graph_context(
        self, analyzer, full_project_files: FilePathList, root_dir: FilePath
    ) -> GraphAnalysisContext:
        """Builds all comprehensive maps for the project context."""
        module_to_file = self._discover_files(analyzer, full_project_files, root_dir)
        import_graph, inbound_links, inheritance_map, file_definitions = (
            self._collect_orphans_from_graph(
                analyzer, full_project_files, root_dir, module_to_file
            )
        )
        return GraphAnalysisContext(
            import_graph=import_graph,
            inbound_links=inbound_links,
            inheritance_map=inheritance_map,
            file_definitions=file_definitions,
        )

    def _resolve_direct_import(self, module_str, module_to_file):
        """Check if module is already in the pre-mapped cache."""
        if module_to_file and module_str in module_to_file.mapping:
            return module_to_file.mapping[module_str]
        return None

    def _resolve_relative_import(
        self, analyzer, module_str, current_file: FilePath
    ) -> FilePath | None:
        """Resolve a relative dotted module path (e.g. '.foo.bar' or '..baz')."""
        dot_count = 0
        for char in module_str:
            if char == ".":
                dot_count += 1
            else:
                break

        rel_module = module_str[dot_count:]
        parts = rel_module.split(".") if rel_module else []

        current_dir = analyzer.fs.get_parent(current_file)
        for _ in range(dot_count - 1):
            current_dir = analyzer.fs.get_parent(current_dir)

        base = str(current_dir)
        path_attempt = (
            f"{base}/{'/'.join(parts)}.py" if parts else f"{base}/__init__.py"
        )
        fp = FilePath(value=path_attempt)
        if analyzer.fs.exists(fp):
            return fp

        if parts:
            dir_attempt = f"{base}/{'/'.join(parts)}/__init__.py"
            dfp = FilePath(value=dir_attempt)
            if analyzer.fs.exists(dfp):
                return dfp

        return None

    def _resolve_absolute_import(
        self, analyzer, module_str, root_dir: FilePath
    ) -> FilePath | None:
        """Resolve an absolute dotted module path by probing known source subdirectories."""
        parts = module_str.split(".")
        subdirs = ["", "src", "lib", "app", "core"]

        for sd in subdirs:
            base_path = f"{str(root_dir)}/{sd}".rstrip("/")
            path_attempt = f"{base_path}/{'/'.join(parts)}.py"
            fp = FilePath(value=path_attempt)
            if analyzer.fs.exists(fp):
                return fp

            dir_attempt = f"{base_path}/{'/'.join(parts)}/__init__.py"
            dfp = FilePath(value=dir_attempt)
            if analyzer.fs.exists(dfp):
                return dfp

        return None

    def _resolve_indirect_import(
        self, analyzer, module_str, current_file: FilePath, root_dir: FilePath
    ) -> FilePath | None:
        """Dispatch relative or absolute import resolution based on module prefix."""
        if module_str.startswith("."):
            return self._resolve_relative_import(analyzer, module_str, current_file)
        return self._resolve_absolute_import(analyzer, module_str, root_dir)

    def resolve_import_to_file(
        self,
        analyzer,
        current_file: FilePath,
        module_path: ModuleName,
        root_dir: FilePath,
        module_to_file: ModuleToFileMap | None = None,
    ) -> FilePath | None:
        """Resolves a dotted module path to an absolute file path."""
        module_str = str(module_path.value)

        # Phase 1: Direct cache lookup
        cached = self._resolve_direct_import(module_str, module_to_file)
        if cached:
            return FilePath(value=cached)

        # Phase 2: Indirect resolution (relative + absolute)
        return self._resolve_indirect_import(
            analyzer, module_str, current_file, root_dir
        )

    def identify_entry_points(
        self, analyzer, all_files: FilePathList, root_dir: FilePath
    ) -> FilePathList:
        """Entry points are surfaces, main.py, or explicitly configured paths."""
        entry_points = []

        for f in all_files:
            basename = str(analyzer.fs.get_basename(f))
            if basename in [
                "cli_main_entry.py",
                "mcp_main_entry.py",
                "mcp_server_handler.py",
                "main.py",
            ]:
                entry_points.append(f)

        for definition in analyzer.layer_map.values():
            if definition.orphan_entry_points:
                for pattern in definition.orphan_entry_points.values:
                    for f in all_files:
                        if fnmatch.fnmatch(str(f), f"*{str(pattern)}*"):
                            entry_points.append(f)

        return FilePathList(values=entry_points)

    def trace_reachability(
        self, entry_points: FilePathList, graph: ImportGraph
    ) -> ReachabilityResult:
        """BFS/DFS to find all reachable files from entry points."""
        reachable = set(entry_points.values)
        queue = list(entry_points.values)

        while queue:
            current = queue.pop(0)
            neighbors = graph.mapping.get(str(current), [])
            for neighbor_str in neighbors:
                neighbor = FilePath(value=neighbor_str)
                if neighbor not in reachable:
                    reachable.add(neighbor)
                    queue.append(neighbor)
        return ReachabilityResult(paths=FilePathSet(values=reachable))
