"""dispatch_routing_protocol — Contract for dispatch/capability routing analysis."""

from __future__ import annotations
from typing import Protocol, runtime_checkable

from ..taxonomy import FilePathList, LintResultList, ContentString, ClassDefinitionMap


@runtime_checkable
class IDispatchRoutingProtocol(Protocol):
    """Contract for checking dispatch routing in MCP/server projects.

    Detects:
    - AES030: Capability method referenced in COMMAND_CATALOG doesn't exist on class
    - AES031: Action routed to wrong capability (single-capability bottleneck)
    """

    def check_capability_routing(
        self,
        analyzer,
        files: FilePathList,
        root_dir,
        results: LintResultList,
    ) -> None:
        """Check dispatch routing across all scanned files."""
        ...


@runtime_checkable
class IDispatchRoutingParserProtocol(Protocol):
    """Contract for parsing source code to find capability classes and methods."""

    def strip_docstrings(self, text: ContentString) -> ContentString:
        """Remove comments and docstrings from source text."""
        ...

    def extract_class_methods(self, text: ContentString) -> ClassDefinitionMap:
        """Extract all class definitions and their methods from source text."""
        ...
