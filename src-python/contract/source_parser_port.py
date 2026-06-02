"""source_parser_port — Port interface for source code structure analysis."""

from abc import ABC, abstractmethod

from ..taxonomy import (
    PrimitiveViolationList,
    FilePath,
    ImportInfoList,
    PrimitiveTypeList,
    SuccessStatus,
    ResponseData,
    MetadataVO,
    SymbolName,
    Count,
    Identity,
    SourceParserError,
    PatternList,
)


class ISourceParserPort(ABC):
    """Abstraction for language-specific source code analysis (AST-less)."""

    @abstractmethod
    def extract_imports(self, path: FilePath) -> ImportInfoList | SourceParserError:
        """Returns import information from the file."""
        ...

    @abstractmethod
    def get_raw_symbols(self, path: FilePath) -> ResponseData | SourceParserError:
        """
        Returns categorized symbols (defined, used, exported, aliases).
        """
        ...

    @abstractmethod
    def get_class_attributes(self, path: FilePath) -> ResponseData:
        """Returns information about class attributes and their type annotations."""
        ...

    @abstractmethod
    def has_all_export(self, path: FilePath) -> SuccessStatus:
        """Returns True if the file defines an __all__ list."""
        ...

    @abstractmethod
    def find_primitive_violations(
        self, path: FilePath, primitive_types: PrimitiveTypeList
    ) -> PrimitiveViolationList:
        """Finds usage of primitive types (attributes, args, returns) instead of domain objects."""
        ...

    @abstractmethod
    def find_unused_imports(self, path: FilePath) -> ImportInfoList:
        """Finds imports that are declared but never used in the file."""
        ...

    @abstractmethod
    def get_class_definitions(self, path: FilePath) -> MetadataVO | SourceParserError:
        """Returns list of class definitions with metadata (name, line, column)."""
        ...

    @abstractmethod
    def get_function_definitions(self, path: FilePath) -> MetadataVO:
        """Returns list of function definitions with metadata (name, line, column)."""
        ...

    @abstractmethod
    def is_symbol_exported(
        self, path: FilePath, symbol: SymbolName | Identity
    ) -> SuccessStatus:
        """Returns True if the symbol is explicitly exported in __all__."""
        ...

    @abstractmethod
    def get_class_methods(self, path: FilePath) -> MetadataVO:
        """Returns {ClassName: [methods]} for all classes in file."""
        ...

    @abstractmethod
    def get_class_bases_map(self, path: FilePath) -> MetadataVO:
        """Returns {ClassName: [BaseClasses]} map."""
        ...

    @abstractmethod
    def get_assignment_targets(self, path: FilePath) -> MetadataVO:
        """Returns list of assignments (target, value_type) in file."""
        ...

    @abstractmethod
    def get_control_flow_count(self, path: FilePath) -> Count:
        """Count control flow statements (if, for, while, try) as a proxy for logic density."""
        ...

    @abstractmethod
    def is_barrel_file(self, path: FilePath) -> bool:
        """Returns True if the file acts as a module barrel/init file."""
        ...

    @abstractmethod
    def get_stem(self, path: FilePath) -> SymbolName:
        """Returns the module stem (filename without extension)."""
        ...

    @abstractmethod
    def is_entry_point(self, path: FilePath) -> bool:
        """Returns True if the file is a system entry point/special file (e.g., main, lib)."""
        ...

    @abstractmethod
    def get_supported_extensions(self) -> PatternList:
        """Returns list of file extensions supported by the current parser context."""
        ...
