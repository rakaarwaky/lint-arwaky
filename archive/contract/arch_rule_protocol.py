"""arch_rule_protocol — Interfaces for architectural rule checking."""

from __future__ import annotations

from typing import Protocol, runtime_checkable, Callable
from abc import ABC, abstractmethod
from ..taxonomy import (
    ArchitectureConfig,
    FilePath,
    FilePathList,
    LintResultList,
    PatternList,
    LayerMapVO,
    LayerNameVO,
    Count,
    Identity,
    ModuleName,
    ErrorMessage,
    CustomMessageVO,
)
from .file_system_port import IFileSystemPort
from .source_parser_port import ISourceParserPort


@runtime_checkable
class IAnalyzer(Protocol):
    """Protocol for the analyzer passed to rule checkers."""

    config: ArchitectureConfig
    layer_map: LayerMapVO
    fs: IFileSystemPort
    parser: ISourceParserPort

    def _detect_layer(self, f: FilePath, root_dir: FilePath) -> LayerNameVO | None: ...
    def _detect_module_layer(self, module_path: FilePath) -> LayerNameVO | None: ...


class IArchRuleProtocol(ABC):
    """Base interface for all architectural rule protocols."""

    @property
    @abstractmethod
    def rule_name(self) -> Identity:
        """The unique identifier for this rule."""
        ...


class INamingCheckerProtocol(ABC):
    """Interface for naming-related architectural checks."""

    @abstractmethod
    def check_file_naming(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Check that files follow the project's underscore-separated pattern."""
        ...

    @abstractmethod
    def check_domain_suffixes(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Check that files have appropriate suffixes for their layer."""
        ...


class IInternalCheckerProtocol(ABC):
    """Interface for internal layer rules (barrels, primitives)."""

    @abstractmethod
    def check_layer_internal_rules(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Enforce barrel completeness and forbid raw primitives."""
        ...


class IMetricCheckerProtocol(ABC):
    """Interface for architectural metric checks."""

    @abstractmethod
    def check_line_counts(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Ensure files are within size boundaries (AES004, AES005)."""
        ...

    @abstractmethod
    def check_mandatory_class_definition(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Verify that mandatory layers contain at least one class."""
        ...


class IRoleCheckerProtocol(ABC):
    """Interface for role-based architectural checks."""

    @abstractmethod
    def check_agent_roles(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Check statelessness, coordination, and domain logic for agents."""
        ...

    @abstractmethod
    def check_surface_roles(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Check dependency restrictions and logic for surfaces."""
        ...


class IArchImportProcessorProtocol(ABC):
    """Interface for processing and validating architectural imports."""

    @abstractmethod
    def process_file_imports(
        self,
        analyzer: IAnalyzer,
        file_path: FilePath,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Evaluate imports against forbidden/allowed rules."""
        ...

    @abstractmethod
    def validate_imports_present(
        self,
        analyzer: IAnalyzer,
        file_path: FilePath,
        root_dir: FilePath,
        required_layers: PatternList,
        results: LintResultList,
        message_template: ErrorMessage,
        layer_name: LayerNameVO,
        layers_display: PatternList,
    ) -> None:
        """Ensure all mandatory imports for a layer are present."""
        ...


class INamingRuleProtocol(IArchRuleProtocol):
    """Interface for naming convention checks."""

    @abstractmethod
    def check_file_naming(
        self,
        files: FilePathList,
        root_dir: FilePath,
        layer_map: LayerMapVO,
        global_expected: Count,
        global_exceptions: PatternList,
        results: LintResultList,
        detect_layer_fn,
    ) -> None:
        """Check that files have the correct number of underscores."""
        ...

    @abstractmethod
    def check_class_naming(
        self,
        files: FilePathList,
        results: LintResultList,
        source_parser: ISourceParserPort,
    ) -> None:
        """Check PascalCase for classes."""
        ...

    @abstractmethod
    def check_function_naming(
        self,
        files: FilePathList,
        results: LintResultList,
        source_parser: ISourceParserPort,
    ) -> None:
        """Check snake_case for functions."""
        ...


class ICodeQualityProtocol(IArchRuleProtocol):
    """Interface for code quality and bypass checks."""

    @abstractmethod
    def check_no_bypass_comments(
        self,
        file_path: FilePath,
        fs: IFileSystemPort,
        results: LintResultList,
        forbidden_words: PatternList | None = None,
        violation_message: ErrorMessage | None = None,
        custom_messages: list[CustomMessageVO] | None = None,
    ) -> None:
        """Ensure no bypass comments."""
        ...

    @abstractmethod
    def check_unused_mandatory_imports(
        self,
        files: FilePathList,
        parser: ISourceParserPort,
        results: LintResultList,
        violation_message: ErrorMessage | None = None,
        mandatory_imports: PatternList | None = None,
        layer_resolver: Callable[[ModuleName], LayerNameVO | None] | None = None,
    ) -> None:
        """Ensure mandatory imports are used."""
        ...

    @abstractmethod
    def check_dead_inheritance_bypass(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Check for empty classes inheriting from contracts."""
        ...

    @abstractmethod
    def check_forbidden_inheritance(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Ensure classes do not inherit from forbidden ports/protocols."""
        ...


class IArchStructureProtocol(IArchRuleProtocol):
    """Interface for structural architectural checks."""

    @abstractmethod
    def check_file_naming(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None: ...

    @abstractmethod
    def check_domain_suffixes(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None: ...

    @abstractmethod
    def check_layer_internal_rules(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None: ...

    @abstractmethod
    def check_line_counts(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None: ...

    @abstractmethod
    def check_mandatory_class_definition(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None: ...

    @abstractmethod
    def check_agent_roles(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None: ...

    @abstractmethod
    def check_surface_roles(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None: ...


class IArchImportProtocol(IArchRuleProtocol):
    """Interface for import-related architectural checks."""

    @abstractmethod
    def check_mandatory_imports(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None: ...

    @abstractmethod
    def check_forbidden_imports(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None: ...

    @abstractmethod
    def check_legacy_import_rules(
        self,
        analyzer: IAnalyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None: ...
