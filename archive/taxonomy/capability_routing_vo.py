"""capability_routing_vo — Value objects for capability and dispatch routing analysis.

Note: Parser algorithm state types (BraceDepthVO, ClassParsingStateVO, ScopeNameVO,
MethodArgsVO, IndentSizeVO) have been moved to capabilities/dispatch_parser_types.py
as they are implementation details of the parser, not domain concepts.
"""

from __future__ import annotations
from pydantic import BaseModel, Field

from .file_path_vo import FilePath
from .lint_position_vo import LineNumber


class ClassNameVO(BaseModel):
    """VO for a class name — domain identifier for a Python class."""

    value: str

    def __str__(self) -> str:
        return self.value


class CapabilityReference(BaseModel):
    """Represents a reference to a capability method in the dispatch catalog."""

    file: FilePath
    line: LineNumber
    class_name: str
    method_name: str


class CapabilityReferenceList(BaseModel):
    """A collection of capability references."""

    references: list[CapabilityReference] = Field(default_factory=list)


class ClassMethodsVO(BaseModel):
    """A collection of method names for a class."""

    methods: list[str] = Field(default_factory=list)


class ClassDefinitionMap(BaseModel):
    """Mapping of class names to their method definitions."""

    definitions: dict[str, ClassMethodsVO] = Field(default_factory=dict)


class ClassFileMap(BaseModel):
    """Mapping of class names to their source files."""

    mapping: dict[str, FilePath] = Field(default_factory=dict)


class ClassUsageItem(BaseModel):
    """Represents a usage of a class method."""

    file: FilePath
    line: LineNumber
    method: str


class ClassUsageItemList(BaseModel):
    """A collection of class usage items."""

    items: list[ClassUsageItem] = Field(default_factory=list)


class ClassUsageMap(BaseModel):
    """Mapping of class names to their usage details."""

    usage: dict[str, ClassUsageItemList] = Field(default_factory=dict)


class CapabilityRoutingContext(BaseModel):
    """Aggregated context for capability routing analysis."""

    references: CapabilityReferenceList = Field(default_factory=CapabilityReferenceList)
    definitions: ClassDefinitionMap = Field(default_factory=ClassDefinitionMap)
    files: ClassFileMap = Field(default_factory=ClassFileMap)
