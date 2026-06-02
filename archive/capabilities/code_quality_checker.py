"""Architecture rule checking: bypass comments and unused imports."""

from __future__ import annotations
from typing import Callable

import logging

from ..taxonomy import (
    AdapterName,
    ColumnNumber,
    ErrorCode,
    FilePath,
    FilePathList,
    LayerDefinition,
    LayerNameVO,
    LineNumber,
    LintMessage,
    LintResult,
    LintResultList,
    PatternList,
    Severity,
    Identity,
    ErrorMessage,
    CustomMessageVO,
    ModuleName,
)
from ..contract import ICodeQualityProtocol, IFileSystemPort, ISourceParserPort

logger = logging.getLogger(__name__)


class CodeQualityRuleChecker(ICodeQualityProtocol):
    """Checks for bypass comments and unused mandatory imports."""

    @property
    def rule_name(self) -> Identity:
        return Identity(value="code_quality")

    def _find_bypass_matches(
        self,
        line,
        line_number,
        forbidden,
        base_msg,
        custom_messages: list[CustomMessageVO] | None = None,
    ) -> tuple[int, str] | None:
        """Check a single line for bypass patterns.

        Returns (column, message) if a match is found, else None.
        """
        lower_line = line.lower()
        for bypass in forbidden:
            if bypass.lower() in lower_line:
                final_msg = base_msg
                if custom_messages:
                    for entry in custom_messages:
                        pattern = str(entry.pattern)
                        message = str(entry.message)
                        if pattern and message and self._match_pattern(bypass, pattern):
                            final_msg = message
                            break

                col = line.find("#")
                if col < 0:
                    col = 0
                return (col, final_msg)
        return None

    def check_no_bypass_comments(
        self,
        file_path: FilePath,
        fs: IFileSystemPort,
        results: LintResultList,
        forbidden_words: PatternList | None = None,
        violation_message: ErrorMessage | None = None,
        custom_messages: list[CustomMessageVO] | None = None,
    ) -> None:
        """Ensure no bypass comments are present in source files."""
        if forbidden_words is None or not forbidden_words.values:
            return
        forbidden = forbidden_words.values

        content = fs.read_text(file_path).value
        lines = content.splitlines()

        # Base violation message
        base_msg = (
            str(violation_message)
            if violation_message
            else (
                "STOP CHEATING! You are strictly forbidden "
                "from using bypass comments. Immediate architectural "
                "remediation is mandated."
            )
        )

        for i, line in enumerate(lines):
            match = self._find_bypass_matches(
                line, i + 1, forbidden, base_msg, custom_messages
            )
            if match is None:
                continue
            col, final_msg = match

            results.append(
                LintResult(
                    file=file_path,
                    line=LineNumber(value=i + 1),
                    column=ColumnNumber(value=col),
                    code=ErrorCode(code="AES014"),
                    message=LintMessage(value=final_msg),
                    source=AdapterName(value="architecture"),
                    severity=Severity.CRITICAL,
                )
            )

    def check_dead_inheritance_bypass(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Detect empty classes that inherit from contracts solely to satisfy linter."""
        for f in files:
            layer_vo = analyzer._detect_layer(f, root_dir)
            if not layer_vo:
                continue
            if layer_vo not in analyzer.layer_map:
                continue
            definition = analyzer.layer_map[layer_vo]
            if not definition.dead_inheritance_bypass:
                continue

            metadata = analyzer.parser.get_class_definitions(f)
            classes = metadata.value.get("classes", [])
            for cls in classes:
                if not isinstance(cls, dict):
                    continue
                if not cls.get("is_dead"):
                    continue
                name = str(cls.get("name", ""))
                if not self._is_bypass_candidate_name(name):
                    continue
                bases = cls.get("bases", [])
                if not bases:
                    continue

                msg = self._resolve_dead_inheritance_message(definition, name, bases)
                results.append(
                    LintResult(
                        file=f,
                        line=LineNumber(value=cls.get("line", 0)),
                        column=ColumnNumber(value=cls.get("column", 0)),
                        code=ErrorCode(code="AES016"),
                        message=LintMessage(value=msg),
                        source=AdapterName(value="architecture"),
                        severity=Severity.CRITICAL,
                    )
                )

    def _resolve_dead_inheritance_message(
        self,
        definition: LayerDefinition,
        class_name,
        bases,
    ):
        """Select violation message based on pattern-specific custom messages."""
        custom_messages = definition.dead_inheritance_bypass_custom_messages
        if custom_messages:
            for entry in custom_messages:
                pattern = str(entry.pattern)
                message = str(entry.message)
                if pattern and message and self._match_pattern(class_name, pattern):
                    # Format placeholders: {name}, {bases}
                    try:
                        return str(message).format(
                            name=class_name,
                            bases=", ".join(str(b) for b in bases) if bases else "",
                        )
                    except Exception:
                        logger.warning(
                            "Failed to format dead inheritance message", exc_info=True
                        )
                        return message  # fallback to raw if format fails

        if definition.dead_inheritance_bypass_violation_message:
            return definition.dead_inheritance_bypass_violation_message

        default = (
            f"DEAD INHERITANCE BYPASS: Class '{class_name}' is empty (pass/docstring only) but inherits from {bases}. "
            "This is a compliance marker with no real implementation. "
            "Implement the required methods or remove the class immediately."
        )
        return default

    def _is_bypass_candidate_name(self, name):
        """Determines if a class name is a candidate for dead inheritance bypass checks.
        In this architecture, any empty class inheriting from a contract is a candidate.
        """
        return True

    def _match_pattern(self, name: str, pattern: str) -> bool:
        """Check if name matches the pattern (supports specialized layer patterns)."""
        if name.lower() == pattern.lower():
            return True

        if "(" in name:
            base = name.split("(")[0]
            if pattern == base:
                return True

        if "(" in pattern and "(" in name:
            p_base, p_subs_raw = pattern.split("(", 1)
            n_base, n_sub_raw = name.split("(", 1)

            if p_base.lower() != n_base.lower():
                return False

            p_subs = [
                s.strip().lower()
                for s in p_subs_raw.rstrip(")").replace(",", "|").split("|")
            ]
            n_sub = n_sub_raw.rstrip(")").lower()
            return n_sub in p_subs

        import fnmatch

        return fnmatch.fnmatch(name.lower(), pattern.lower())

    def _is_mandatory_import(
        self,
        module_name,
        mandatory_imports: PatternList,
        layer_resolver=None,
    ):
        """Check if a module (or its resolved layer) matches any mandatory import pattern."""
        # Determine layer of the import if resolver provided
        layer = None
        if layer_resolver:
            layer = (
                str(layer_resolver(module_name))
                if layer_resolver(module_name)
                else None
            )

        for pattern in mandatory_imports.values:
            if self._match_pattern(module_name, pattern):
                return True
            if layer and self._match_pattern(layer, pattern):
                return True
        return False

    def check_unused_mandatory_imports(
        self,
        files: FilePathList,
        parser: ISourceParserPort,
        results: LintResultList,
        violation_message: ErrorMessage | None = None,
        mandatory_imports: PatternList | None = None,
        layer_resolver: Callable[[ModuleName], LayerNameVO | None] | None = None,
    ) -> None:
        """Ensure mandatory imports are actually used."""
        if mandatory_imports is None or not mandatory_imports.values:
            return

        for f in files:
            unused_infos = parser.find_unused_imports(f)
            if not unused_infos:
                continue

            actual_unused = []
            for imp in unused_infos:
                module_name = str(imp.module)
                if self._is_mandatory_import(
                    module_name, mandatory_imports, layer_resolver
                ):
                    actual_unused.append(module_name)

            if not actual_unused:
                continue

            results.append(
                LintResult(
                    file=f,
                    line=LineNumber(value=0),
                    column=ColumnNumber(value=0),
                    code=ErrorCode(code="AES015"),
                    message=LintMessage(
                        value=str(violation_message)
                        if violation_message
                        else (
                            f"ARCHITECTURAL FRAUD DETECTED: Symbols {actual_unused} "
                            f"are imported but never used. If you import a mandatory "
                            f"layer, you MUST implement logic using it. "
                            f"No ghost imports allowed."
                        )
                    ),
                    source=AdapterName(value="architecture"),
                    severity=Severity.CRITICAL,
                )
            )

    def check_forbidden_inheritance(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        """Ensure classes do not inherit from forbidden ports/protocols."""
        for f in files:
            self._check_file_inheritance(f, analyzer, root_dir, results)

    def _check_file_inheritance(self, f, analyzer, root_dir, results) -> None:
        """Check inheritance rules for a single file."""
        layer_vo = analyzer._detect_layer(f, root_dir)
        if not layer_vo or layer_vo not in analyzer.layer_map:
            return
        definition = analyzer.layer_map[layer_vo]
        forbidden_patterns = definition.forbidden_inheritance
        if not forbidden_patterns or not forbidden_patterns.values:
            return

        metadata = analyzer.parser.get_class_definitions(f)
        classes = metadata.value.get("classes", [])
        aliases = analyzer.parser.get_raw_symbols(f).value.get("aliases", {})

        for cls in classes:
            if not isinstance(cls, dict):
                continue
            for base in cls.get("resolved_bases") or cls.get("bases") or []:
                self._check_base_class(cls, base, aliases, layer_vo, definition,
                                       forbidden_patterns, analyzer, f, results)

    def _check_base_class(self, cls, base, aliases, layer_vo, definition,
                          forbidden_patterns, analyzer, f, results) -> None:
        """Check a single base class against forbidden inheritance."""
        full_base_name = self._resolve_base_name(base, aliases, layer_vo)
        base_layer = analyzer._detect_module_layer(full_base_name)
        if not base_layer:
            return
        self._check_forbidden_match(cls, base, base_layer, forbidden_patterns,
                                    definition, analyzer, f, results)

    def _resolve_base_name(self, base, aliases, layer_vo) -> str:
        full_base_name = aliases.get(base, base)
        if str(full_base_name).startswith("."):
            layer_str = str(layer_vo)
            base_layer_name = layer_str.split("(")[0]
            return f"{base_layer_name}{full_base_name}"
        return str(full_base_name)

    def _check_forbidden_match(self, cls, base, base_layer, forbidden,
                               definition, analyzer, f, results) -> None:
        for pattern in forbidden.values:
            if self._match_pattern(str(base_layer), pattern):
                msg = (
                    str(definition.forbidden_inheritance_violation_message)
                    if definition.forbidden_inheritance_violation_message
                    else f"AES026 INHERITANCE_VIOLATION: Class '{cls.get('name')}' inherits from '{base}' which belongs to forbidden layer '{base_layer}'."
                )
                results.append(
                    LintResult(
                        file=f,
                        line=LineNumber(value=cls.get("line", 0)),
                        column=ColumnNumber(value=cls.get("column", 0)),
                        code=ErrorCode(code="AES026"),
                        message=LintMessage(value=msg),
                        source=AdapterName(value="architecture"),
                        severity=Severity.CRITICAL,
                    )
                )
                break

