from pydantic import BaseModel, ConfigDict, Field

from .lint_severity_vo import Severity
from .path_collection_vo import FilePathSet


class ImportGraph(BaseModel):
    """Mapping of files to their imports."""

    model_config = ConfigDict(frozen=True)
    mapping: dict[str, list[str]] = Field(default_factory=dict)


class InboundLinkMap(BaseModel):
    """Mapping of files to files that import them."""

    model_config = ConfigDict(frozen=True)
    mapping: dict[str, list[str]] = Field(default_factory=dict)


class InheritanceMap(BaseModel):
    """Mapping of classes to their implementers' file paths."""

    model_config = ConfigDict(frozen=True)
    mapping: dict[str, list[str]] = Field(default_factory=dict)


class FileDefinitionMap(BaseModel):
    """Mapping of files to classes defined within them."""

    model_config = ConfigDict(frozen=True)
    mapping: dict[str, list[str]] = Field(default_factory=dict)


class ReachabilityResult(BaseModel):
    """Set of reachable file paths."""

    model_config = ConfigDict(frozen=True)
    paths: FilePathSet = Field(default_factory=FilePathSet)

    def __contains__(self, item: object) -> bool:
        return item in self.paths


class ModuleToFileMap(BaseModel):
    """Mapping of module names to their absolute file paths."""

    model_config = ConfigDict(frozen=True)
    mapping: dict[str, str] = Field(default_factory=dict)


class GraphAnalysisContext(BaseModel):
    """Container for all graph-related analysis data."""

    model_config = ConfigDict(frozen=True)
    import_graph: ImportGraph
    inbound_links: InboundLinkMap
    inheritance_map: InheritanceMap
    file_definitions: FileDefinitionMap


class OrphanIndicatorResult(BaseModel):
    """Result of an orphan indicator evaluation."""

    model_config = ConfigDict(frozen=True)
    is_orphan: bool
    reason: str = ""
    severity: Severity = Severity.HIGH
