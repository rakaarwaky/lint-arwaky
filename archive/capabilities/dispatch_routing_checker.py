"""dispatch_routing_checker — Static analysis for MCP/server dispatch routing.

Detects:
  AES030 — Capability method referenced in COMMAND_CATALOG doesn't exist on the class
  AES031 — Orchestrator routes ALL actions to a single capability when other options exist
  AES032 — Capability method called without required request VO parameter
"""

from __future__ import annotations
import re
from ..contract import IDispatchRoutingParserProtocol


from ..taxonomy import (
    AdapterName,
    CapabilityReference,
    CapabilityReferenceList,
    CapabilityRoutingContext,
    ClassDefinitionMap,
    ClassFileMap,
    ClassNameVO,
    ClassUsageItem,
    ClassUsageItemList,
    ClassUsageMap,
    ColumnNumber,
    ContentString,
    ErrorCode,
    FilePath,
    FilePathList,
    LineNumber,
    LintMessage,
    LintResult,
    LintResultList,
    Severity,
    SymbolNameList,
)
from .dispatch_parser_types import MethodArgsVO
from ..contract import IDispatchRoutingProtocol

# Pattern: "capability": "ClassName.method_name"
CAPABILITY_REF_PATTERN = re.compile(
    r"""["']capability["']\s*:\s*["']([A-Za-z_][\w]*)\.([A-Za-z_][\w]*)["']"""
)


class DispatchRoutingChecker(IDispatchRoutingProtocol):
    """Checks for dispatch/capability routing issues via static analysis."""

    def __init__(self, parser: IDispatchRoutingParserProtocol | None = None) -> None:
        """Inject dependency."""
        from .dispatch_routing_parser import DispatchRoutingParser

        self.parser = parser or DispatchRoutingParser()

    def check_capability_routing(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Main entry: scan all files for dispatch routing violations."""
        # Phase 1: Find all capability references and class definitions
        context = self._check_capability_by_layer(analyzer, files, self.parser)

        # Phase 2: Verify each capability reference
        self._verify_capability_references(
            context.references, context.definitions, results
        )

        # Phase 3: Check for single-capability routing bottleneck (AES031)
        self._check_single_capability_bottleneck(
            context.references, context.definitions, results, root_dir
        )

        # Phase 4: Check for missing VO construction in capability files (AES032)
        cap_files = {ref.file for ref in context.references.references}
        if cap_files:
            self._check_missing_vo_construction(analyzer, list(cap_files), results)

    def _check_capability_by_layer(
        self,
        analyzer,
        files: FilePathList,
        parser: IDispatchRoutingParserProtocol,
    ) -> CapabilityRoutingContext:
        """Scan files matching a layer pattern for capability refs and class defs."""
        capability_refs = CapabilityReferenceList()
        class_defs = ClassDefinitionMap()
        class_files = ClassFileMap()

        for f in files:
            path = str(f)
            if not path.endswith(".py"):
                continue

            text = self._read_file_content(analyzer, f)
            if text is None:
                continue

            # Omit triple-quoted docstrings to avoid false positives
            text = parser.strip_docstrings(text)

            # Find "capability": "Class.method" references
            self._collect_capability_refs(text, f, capability_refs)

            # Find class definitions and their methods
            class_info = parser.extract_class_methods(text)
            for cls_name, methods_vo in class_info.definitions.items():
                if cls_name not in class_defs.definitions:
                    class_defs.definitions[cls_name] = methods_vo
                    class_files.mapping[cls_name] = f

        return CapabilityRoutingContext(
            references=capability_refs,
            definitions=class_defs,
            files=class_files,
        )

    def _read_file_content(self, analyzer, file_path: FilePath) -> ContentString | None:
        """Read file content with proper error handling."""
        try:
            raw_content = (
                analyzer.fs.read_text(file_path)
                if hasattr(analyzer.fs, "read_text")
                else None
            )
            if raw_content is None:
                return None
            val = (
                raw_content.value if hasattr(raw_content, "value") else str(raw_content)
            )
            return ContentString(value=val)
        except Exception:
            return None

    def _collect_capability_refs(
        self,
        text: ContentString,
        file_path: FilePath,
        refs: CapabilityReferenceList,
    ) -> None:
        """Extract all 'capability' -> 'Class.method' references from stripped text."""
        content = text.value
        for match in CAPABILITY_REF_PATTERN.finditer(content):
            class_name = match.group(1)
            method_name = match.group(2)
            line_no = content[: match.start()].count("\n") + 1
            refs.references.append(
                CapabilityReference(
                    file=file_path,
                    line=LineNumber(value=line_no),
                    class_name=class_name,
                    method_name=method_name,
                )
            )

    def _verify_capability_references(
        self,
        capability_refs: CapabilityReferenceList,
        class_defs: ClassDefinitionMap,
        results: LintResultList,
    ) -> None:
        """Verify each capability reference resolves to an existing class+method."""
        for ref in capability_refs.references:
            # Check 1: Does the class exist?
            if ref.class_name not in class_defs.definitions:
                self._report(
                    results,
                    file=ref.file,
                    line=ref.line,
                    code=ErrorCode(code="AES030"),
                    message=LintMessage(
                        value=f"Capability class '{ref.class_name}' not found in any scanned file. "
                        f"Referenced from COMMAND_CATALOG but no class definition exists."
                    ),
                )
                continue

            # Check 2: Does the method exist on the class?
            methods = class_defs.definitions[ref.class_name].methods
            if ref.method_name not in methods:
                found_methods = ", ".join(methods) if methods else "(none)"
                self._report(
                    results,
                    file=ref.file,
                    line=ref.line,
                    code=ErrorCode(code="AES030"),
                    message=LintMessage(
                        value=f"Method '{ref.method_name}' not found on class '{ref.class_name}'. "
                        f"Defined methods: {found_methods}. "
                        f"Check for naming mismatch between catalog and capability."
                    ),
                )

    def _check_single_capability_bottleneck(
        self,
        capability_refs: CapabilityReferenceList,
        class_defs: ClassDefinitionMap,
        results: LintResultList,
        root_dir: FilePath,
    ) -> None:
        """Check if ALL capability routes go to a single class when other options exist."""
        if not capability_refs.references:
            return

        # Group capability refs by class
        class_usage = self._group_capabilities_by_class(capability_refs)

        # If all entries point to one class
        if len(class_usage.usage) == 1 and list(class_usage.usage.values())[0].items:
            single_class = list(class_usage.usage.keys())[0]
            usage_list = class_usage.usage[single_class]
            # Check if there ARE other capability classes in the project
            other_classes = [c for c in class_defs.definitions if c != single_class]
            if other_classes and len(usage_list.items) >= 3:
                self._report_class_bottleneck(
                    results,
                    ClassNameVO(value=single_class),
                    usage_list,
                    SymbolNameList(values=other_classes),
                )

    def _group_capabilities_by_class(
        self,
        capability_refs: CapabilityReferenceList,
    ) -> ClassUsageMap:
        """Group capability references by their target class name."""
        class_usage = ClassUsageMap()
        for ref in capability_refs.references:
            if ref.class_name not in class_usage.usage:
                class_usage.usage[ref.class_name] = ClassUsageItemList()
            class_usage.usage[ref.class_name].items.append(
                ClassUsageItem(
                    file=ref.file,
                    line=ref.line,
                    method=ref.method_name,
                )
            )
        return class_usage

    def _report_class_bottleneck(
        self,
        results: LintResultList,
        class_name: ClassNameVO,
        refs: ClassUsageItemList,
        other_classes: SymbolNameList,
    ) -> None:
        """Report AES031 bottleneck violations for a single class."""
        other_names = [str(c) for c in other_classes.values]
        for item in refs.items:
            self._report(
                results,
                file=item.file,
                line=item.line,
                code=ErrorCode(code="AES031"),
                message=LintMessage(
                    value=f"Action '{item.method}' routes to '{class_name}' but "
                    f"{len(other_names)} other capability classes exist "
                    f"({', '.join(other_names[:5])}). "
                    f"Actions should be distributed to the correct capability."
                ),
            )

    def _check_missing_vo_construction(
        self,
        analyzer,
        files: FilePathList,
        results: LintResultList,
    ) -> None:
        """Check capability methods that require 'request' or 'data' VOs.

        Looks for async def method calls where the signature expects a VO
        parameter but the call site passes nothing or uses wrong types.
        """
        for f in files:
            self._check_file_vo_construction(analyzer, f, results)

    def _check_file_vo_construction(
        self,
        analyzer,
        file_path: FilePath,
        results: LintResultList,
    ) -> None:
        """Check a single file for missing VO construction in capability calls."""
        path = str(file_path)
        if not path.endswith(".py"):
            return
        try:
            text = (
                analyzer.fs.read_text(file_path)
                if hasattr(analyzer.fs, "read_text")
                else None
            )
            if text is None:
                return
            content = text.value if hasattr(text, "value") else str(text)
        except Exception:
            return

        # Find capability method calls: await self.some_executor.method(...)
        # Check if they pass proper VO arguments
        call_pattern = re.compile(r"(?:await\s+)?self\.\w+\.(\w+)\s*\(")
        for match in call_pattern.finditer(content):
            method_name = match.group(1)
            paren_start = match.end() - 1  # position of (
            args_vo = self._extract_args(
                ContentString(value=content), ColumnNumber(value=paren_start)
            )
            if args_vo.value is None:
                continue
            args_text = args_vo.value.strip()
            if not args_text:
                line_no = content[:paren_start].count("\n") + 1
                self._report(
                    results,
                    file_path,
                    LineNumber(value=line_no),
                    ErrorCode(code="AES032"),
                    LintMessage(
                        value=f"Capability call 'self.some_executor.{method_name}()' "
                        f"missing required request/data VO parameter. "
                        f"Capability methods expect a typed Value Object argument."
                    ),
                )

    def _extract_args(
        self, text: ContentString, open_paren: ColumnNumber
    ) -> MethodArgsVO:
        """Extract content between parentheses at position open_paren."""
        content = text.value
        idx = open_paren.value
        if idx >= len(content) or content[idx] != "(":
            return MethodArgsVO(value=None)
        depth = 1
        i = idx + 1
        while i < len(content) and depth > 0:
            if content[i] == "(":
                depth += 1
            elif content[i] == ")":
                depth -= 1
            i += 1
        if depth == 0:
            return MethodArgsVO(value=content[idx + 1 : i - 1])
        return MethodArgsVO(value=None)

    def _report(
        self,
        results: LintResultList,
        file: FilePath,
        line: LineNumber,
        code: ErrorCode,
        message: LintMessage,
    ) -> None:
        results.values.append(
            LintResult(
                file=file,
                line=line,
                column=ColumnNumber(value=1),
                code=code,
                message=message,
                severity=Severity.MEDIUM,
                source=AdapterName(value="dispatch_routing"),
            )
        )
