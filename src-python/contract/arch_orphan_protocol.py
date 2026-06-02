from abc import ABC, abstractmethod
from .arch_rule_protocol import IArchRuleProtocol, IAnalyzer
from ..taxonomy import (
    FilePath,
    FilePathList,
    LintResultList,
    ModuleName,
    ImportGraph,
    ReachabilityResult,
    InboundLinkMap,
    GraphAnalysisContext,
    OrphanIndicatorResult,
    FileDefinitionMap,
    InheritanceMap,
    ModuleToFileMap,
)
from ..taxonomy.layer_definition_vo import LayerDefinition


class IArchOrphanProtocol(IArchRuleProtocol):
    """Protocol for detecting orphan files and dead code chains."""

    @abstractmethod
    def check_orphans(
        self,
        analyzer: IAnalyzer,
        all_files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Analyzes all files to find those unreachable from entry points."""
        ...


class IOrphanGraphProtocol(ABC):
    """Interface for resolving import graphs and reachability."""

    @abstractmethod
    def build_graph_context(
        self,
        analyzer: IAnalyzer,
        full_project_files: FilePathList,
        root_dir: FilePath,
    ) -> GraphAnalysisContext:
        """Builds all comprehensive maps for the project context."""
        ...

    @abstractmethod
    def resolve_import_to_file(
        self,
        analyzer: IAnalyzer,
        current_file: FilePath,
        module_path: ModuleName,
        root_dir: FilePath,
        module_to_file: ModuleToFileMap | None = None,
    ) -> FilePath | None:
        """Resolves a dotted module path to an absolute file path."""
        ...

    @abstractmethod
    def identify_entry_points(
        self,
        analyzer: IAnalyzer,
        all_files: FilePathList,
        root_dir: FilePath,
    ) -> FilePathList:
        """Identifies project entry points (surfaces, main, etc.)."""
        ...

    @abstractmethod
    def trace_reachability(
        self, entry_points: FilePathList, graph: ImportGraph
    ) -> ReachabilityResult:
        """Traces all reachable files from entry points via BFS."""
        ...


class IOrphanIndicatorProtocol(ABC):
    """Interface for evaluating orphan indicators across different layers."""

    @abstractmethod
    def is_taxonomy_orphan(
        self,
        analyzer: IAnalyzer,
        f: FilePath,
        root_dir: FilePath,
        definition: LayerDefinition | None,
        inbound_links: InboundLinkMap,
    ) -> OrphanIndicatorResult:
        """Evaluate if a taxonomy file is orphaned."""
        ...

    @abstractmethod
    def is_contract_orphan(
        self,
        analyzer: IAnalyzer,
        f: FilePath,
        root_dir: FilePath,
        file_definitions: FileDefinitionMap,
        inheritance_map: InheritanceMap,
    ) -> OrphanIndicatorResult:
        """Evaluate if a contract file is orphaned (no grounded heirs)."""
        ...

    @abstractmethod
    def is_infra_cap_orphan(
        self,
        analyzer: IAnalyzer,
        f: FilePath,
        root_dir: FilePath,
        alive_files: ReachabilityResult,
    ) -> OrphanIndicatorResult:
        """Evaluate if an infra/capability file is orphaned (not wired/unreachable)."""
        ...

    @abstractmethod
    def is_agent_orphan(
        self, analyzer: IAnalyzer, f: FilePath, root_dir: FilePath
    ) -> OrphanIndicatorResult:
        """Evaluate if an agent component is orphaned (not wired in container)."""
        ...

    @abstractmethod
    def is_surface_orphan(
        self,
        f: FilePath,
        alive_files: ReachabilityResult,
        definition: LayerDefinition | None = None,
    ) -> OrphanIndicatorResult:
        """Evaluate if a surface component is orphaned (unreachable)."""
        ...

    @abstractmethod
    def is_generic_orphan(
        self,
        f: FilePath,
        alive_files: ReachabilityResult,
        inbound_links: InboundLinkMap,
    ) -> OrphanIndicatorResult:
        """Evaluate if a generic file is orphaned."""
        ...
