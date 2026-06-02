"""
Cycle detection (DFS) for directed graphs.

"""

from __future__ import annotations
from typing import Any
import logging

from ..taxonomy import (
    AdapterName,
    ArchitectureConfig,
    ColumnNumber,
    ComplianceStatus,
    ContentString,
    DirectoryPath,
    ErrorCode,
    FilePath,
    Identity,
    LayerNameVO,
    LineNumber,
    LintMessage,
    LintResult,
    LintResultList,
    Location,
    LocationList,
    PatternList,
    Severity,
    SymbolName,
    LAYER_AGENT,
)
from ..contract import (
    ICycleAnalysisProtocol,
    IFileSystemPort,
    ISourceParserPort,
    ISemanticTracerProtocol,
)

logger = logging.getLogger(__name__)

try:
    from auto_linter import auto_linter_rust
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False


def _detect_cycle_edges(edges: list[dict[str, str]]) -> PatternList:
    """Detect cycles in a directed graph adjacency list.

    Returns list of "source->target" edge keys that are part of cycles.
    Uses DFS-based cycle detection.
    """
    if RUST_AVAILABLE:
        try:
            res_list = auto_linter_rust.detect_cycle_edges(edges)
            return PatternList(values=res_list)
        except Exception as e:
            logger.warning(f"Rust cycle detector failed: {e}. Falling back to Python.")

    from collections import defaultdict

    graph: dict[ContentString, set[ContentString]] = defaultdict(set)
    for e in edges:
        graph[e["source"]].add(e["target"])

    WHITE, GRAY, BLACK = 0, 1, 2
    color: dict[ContentString, int] = defaultdict(lambda: WHITE)
    cycle_edges: PatternList = PatternList(values=[])

    def dfs(node: ContentString, path: set[ContentString]) -> None:
        color[node] = GRAY
        for neigh in list(graph.get(node, [])):
            if color[neigh] == GRAY:
                cycle_edges.values.append(f"{node}->{neigh}")
            elif color[neigh] == WHITE:
                dfs(neigh, path)
        color[node] = BLACK

    for node in list(graph):
        if color[node] == WHITE:
            dfs(node, set())

    return cycle_edges


class CycleAnalyzer(ICycleAnalysisProtocol):
    """Detects circular imports and dependency cycles (Capability)."""

    def __init__(
        self,
        config: ArchitectureConfig,
        layer_map: dict[Identity, Any],
        fs: IFileSystemPort,
        parser: ISourceParserPort,
        tracer: ISemanticTracerProtocol | None = None,
    ):
        self.config = config
        self.fs = fs
        self.parser = parser
        self.tracer = tracer
        self.layer_map = layer_map

    def name(self) -> AdapterName:
        return AdapterName(value="architecture")

    async def scan(self, path: FilePath) -> LintResultList:
        """Scan path (file or directory) for circular import violations."""
        if not self.config.enabled or not self.config.governance_rules:
            return LintResultList()

        results: LintResultList = LintResultList(values=[])
        root_dir = self._resolve_root(path)

        # Walk all Python files
        for file_path in self.fs.walk(path):
            if not str(file_path).endswith(".py"):
                continue
            file_layer = self._detect_layer(file_path, root_dir)
            if not file_layer or file_layer == LAYER_AGENT:
                continue

            results.values.extend(self._scan_file(file_path, file_layer, root_dir))

        # Record history for trends analysis
        self._record_history(str(path), results)
        return results

    def _scan_file(
        self, file_path: FilePath, file_layer: LayerNameVO, root_dir: FilePath
    ) -> list[LintResult]:
        """Process a single Python file for circular import violations."""
        violations: list[LintResult] = []
        imports = self.parser.extract_imports(file_path)

        # Build dependency graph from imports
        edges: list[dict[str, str]] = []
        for imp in imports:
            target_layer = self._detect_module_layer(imp.module)
            if target_layer:
                edges.append({"source": str(file_layer), "target": str(target_layer)})

        # Detect cycles in the dependency graph
        cycle_edges = _detect_cycle_edges(edges)
        if cycle_edges:
            # Create violation for each cycle edge
            for edge_key in cycle_edges:
                source, target = edge_key.split("->")
                violation = self._create_violation(
                    file_path=file_path,
                    line_no=1,  # We don't have line info for graph-level violations
                    module_path=f"{source} -> {target}",
                    description="Circular import detected",
                    file_layer=str(file_layer),
                    target_layer=target,
                    root_dir=root_dir,
                    imported_name=None,
                )
                violations.append(violation)

        return violations

    def _create_violation(
        self,
        file_path: FilePath,
        line_no: LineNumber,
        module_path: ContentString,
        description: ContentString,
        file_layer: Identity,
        target_layer: Identity,
        root_dir: FilePath,
        imported_name: Identity | None = None,
    ) -> LintResult:
        """Construct a LintResult for a circular import violation."""
        base_msg: ContentString = ContentString(
            value=(
                f"[AES Circular Import] {description}. "
                f"Dependency cycle: {file_layer} -> {target_layer} via '{module_path}'."
            )
        )

        # Optional: extend with tracer information if available
        related_locations: list[ContentString] = []
        if self.tracer and imported_name and str(imported_name) != "*":
            try:
                func_candidate = str(imported_name)
                callers = self.tracer.trace_call_chain(
                    root_dir=DirectoryPath(value=str(root_dir)),
                    target_name=SymbolName(value=func_candidate),
                )
                related_locations = [
                    ContentString(value=f"CallSite: {c}") for c in callers.values[:3]
                ]
            except Exception as e:
                logger.debug(f"Tracer error: {e}")

        return LintResult(
            file=file_path,
            line=line_no,
            column=ColumnNumber(value=0),
            code=ErrorCode(code="AES020"),  # Assuming AES020 for circular imports
            message=LintMessage(value=str(base_msg)),
            source=AdapterName(value=str(self.name())),
            severity=Severity.CRITICAL,
            related_locations=LocationList(
                values=[Location(description=loc) for loc in related_locations]
            ),
        )

    def apply_fix(self, path: FilePath) -> ComplianceStatus:
        """Circular import violations require manual architectural refactoring."""
        return ComplianceStatus(value=False)

    def _resolve_root(self, path: FilePath) -> FilePath:
        """Find project root (directory containing 'src')."""
        current = path
        for _ in range(5):
            src_dir = self.fs.path_join(str(current), "src")
            if self.fs.is_directory(src_dir).value:
                return current
            parent = self.fs.get_parent(current)
            if parent == current:
                break
            current = parent
        return path

    def _detect_layer(
        self, file_path: FilePath, root_dir: FilePath
    ) -> LayerNameVO | None:
        rel: ContentString = ContentString(
            value=str(self.fs.get_relative_path(file_path, root_dir))
        )
        sorted_layers = sorted(
            self.layer_map.items(), key=lambda x: len(str(x[1].path)), reverse=True
        )
        for name, definition in sorted_layers:
            path_def = definition.path_str
            if rel.startswith(path_def) or f"/{path_def}/" in f"/{rel}/":
                return LayerNameVO(value=str(name))
        return None

    def _detect_module_layer(self, module_path: str) -> LayerNameVO | None:
        parts = str(module_path).split(".")
        for part in parts:
            for name, definition in self.layer_map.items():
                if part == name or part == definition.path_str:
                    return LayerNameVO(value=str(name))
        return None
