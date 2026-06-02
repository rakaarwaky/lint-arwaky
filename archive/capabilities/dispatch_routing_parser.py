"""dispatch_routing_parser — Parser logic for capability class extraction."""

from __future__ import annotations
import re

from ..taxonomy import (
    ClassDefinitionMap,
    ClassMethodsVO,
    ContentString,
)
from .dispatch_parser_types import (
    BraceDepthVO,
    ClassParsingStateVO,
    ScopeNameVO,
)
from ..contract import IDispatchRoutingParserProtocol


class DispatchRoutingParser(IDispatchRoutingParserProtocol):
    """Helper to parse source code and extract class/method structures."""

    def strip_docstrings(self, text: ContentString) -> ContentString:
        """Remove triple-quoted strings and comments from text."""
        content = str(text.value)
        # Remove """...""" docstrings
        content = re.sub(r'""".*?"""', "", content, flags=re.DOTALL)
        # Remove '''...''' docstrings
        content = re.sub(r"'''[\s\S]*?'''", "", content)
        # Remove single-line comments
        content = re.sub(r"^\s*#.*$", "", content, flags=re.MULTILINE)
        return ContentString(value=content)

    def extract_class_methods(self, text: ContentString) -> ClassDefinitionMap:
        """Parse text to find class definitions and their methods."""
        result = ClassDefinitionMap()
        current_class = ScopeNameVO()
        class_brace_depth = BraceDepthVO()
        current_brace_depth = BraceDepthVO()

        for line in text.value.split("\n"):
            stripped = line.strip()
            state = self._process_class_line(
                ContentString(value=stripped),
                ContentString(value=line),
                ClassParsingStateVO(
                    current_class=current_class,
                    class_brace_depth=class_brace_depth,
                    current_brace_depth=current_brace_depth,
                ),
                result,
            )
            current_class = state.current_class
            class_brace_depth = state.class_brace_depth
            current_brace_depth = state.current_brace_depth

        return result

    def _process_class_line(
        self,
        stripped: ContentString,
        line: ContentString,
        state: ClassParsingStateVO,
        result: ClassDefinitionMap,
    ) -> ClassParsingStateVO:
        """Process a single line of text for class/method extraction."""
        current_class = state.current_class
        class_brace_depth = state.class_brace_depth
        current_brace_depth = state.current_brace_depth

        # Detect class definition
        class_result = self._handle_class_definition(
            stripped, current_class, result, class_brace_depth, current_brace_depth
        )
        if class_result is not None:
            return class_result

        # Detect method definition inside a class
        if current_class.value is not None:
            self._handle_method_definition(stripped, line, current_class, result)

            # Track braces for scope tracking
            current_brace_depth.value += stripped.value.count(
                "{"
            ) - stripped.value.count("}")

            # Check if we left the class scope (dedented back to class level)
            current_class = self._handle_scope_exit(
                line, current_class, class_brace_depth, current_brace_depth
            )

        return ClassParsingStateVO(
            current_class=current_class,
            class_brace_depth=class_brace_depth,
            current_brace_depth=current_brace_depth,
        )

    def _handle_class_definition(
        self,
        stripped: ContentString,
        current_class: ScopeNameVO,
        result: ClassDefinitionMap,
        class_brace_depth: BraceDepthVO,
        current_brace_depth: BraceDepthVO,
    ) -> ClassParsingStateVO | None:
        """Detect a class definition line and update state."""
        class_match = re.match(r"^class\s+([A-Za-z_][\w]*)\s*[\(:]", stripped.value)
        if not class_match:
            return None
        new_class = ScopeNameVO(value=class_match.group(1))
        result.definitions[new_class.value] = ClassMethodsVO()
        class_brace_depth.value = current_brace_depth.value
        current_brace_depth.value += stripped.value.count("{") - stripped.value.count(
            "}"
        )
        return ClassParsingStateVO(
            current_class=new_class,
            class_brace_depth=class_brace_depth,
            current_brace_depth=current_brace_depth,
        )

    def _handle_method_definition(
        self,
        stripped: ContentString,
        line: ContentString,
        current_class: ScopeNameVO,
        result: ClassDefinitionMap,
    ) -> None:
        """Detect and record a method definition if found inside current class."""
        method_match = re.match(
            r"^(?:async\s+)?def\s+([A-Za-z_][\w]*)\s*\(", stripped.value
        )
        if method_match and current_class.value is not None:
            indent = len(line.value) - len(line.value.lstrip())
            if indent <= 8:
                result.definitions[current_class.value].methods.append(
                    method_match.group(1)
                )

    def _handle_scope_exit(
        self,
        line: ContentString,
        current_class: ScopeNameVO,
        class_brace_depth: BraceDepthVO,
        current_brace_depth: BraceDepthVO,
    ) -> ScopeNameVO:
        """Detect when we've left the class scope due to dedent."""
        stripped = line.value.strip()
        if stripped == "" or stripped.startswith("#"):
            return current_class

        # Check indentation of original line
        is_indented = line.value.startswith(" ") or line.value.startswith("\t")

        if not is_indented:
            if current_brace_depth.value <= class_brace_depth.value:
                if (
                    not stripped.startswith("@")
                    and not stripped.startswith("class ")
                    and not stripped.startswith("def ")
                    and not stripped.startswith("async ")
                ):
                    return ScopeNameVO(value=None)
        return current_class
