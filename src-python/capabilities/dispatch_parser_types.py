"""dispatch_parser_types — Local parser state types for dispatch routing analysis.

These types represent the INTERNAL PARSING STATE of the dispatch routing parser
algorithm. They are implementation details of how the parser tracks class
boundaries and brace depth while scanning Python source code.

They are NOT domain concepts and should not be in the shared taxonomy.
"""

from __future__ import annotations
from typing import Any
from pydantic import BaseModel


class BraceDepthVO(BaseModel):
    """VO for brace depth — tracks current { } nesting during parsing."""

    value: Any = 0


class MethodArgsVO(BaseModel):
    """VO for method arguments string captured during parsing."""

    value: Any = None


class ScopeNameVO(BaseModel):
    """VO for a scope name (class/method) during parse traversal."""

    value: Any = None


class IndentSizeVO(BaseModel):
    """VO for indentation size — tracks indent level during parsing."""

    value: Any = 0


class ClassParsingStateVO(BaseModel):
    """Aggregate state of the class parser at any given line."""

    current_class: ScopeNameVO
    class_brace_depth: BraceDepthVO
    current_brace_depth: BraceDepthVO
