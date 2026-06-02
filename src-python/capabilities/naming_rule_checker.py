import re
from typing import Callable
from ..taxonomy import (
    AdapterName,
    ColumnNumber,
    ErrorCode,
    FilePath,
    FilePathList,
    LineNumber,
    LintMessage,
    LintResult,
    LintResultList,
    Severity,
    Count,
    Identity,
    LayerMapVO,
    PatternList,
    LayerNameVO,
)

from ..contract import INamingRuleProtocol, ISourceParserPort

# Regex: snake_case allows leading _, lowercase letters, digits, and underscores
_SNAKE_CASE_RE = re.compile(r"^_?[a-z][a-z0-9]*(_[a-z0-9]+)*$")


class NamingRuleChecker(INamingRuleProtocol):
    """File naming convention checks."""

    @property
    def rule_name(self) -> Identity:
        return Identity(value="naming")

    @staticmethod
    def _has_snake_case(name: str) -> bool:
        """Return True if *name* follows snake_case convention."""
        return bool(_SNAKE_CASE_RE.match(name))

    @staticmethod
    def _resolve_layer_config(
        f: FilePath,
        root_dir: FilePath,
        layer_map: LayerMapVO,
        global_expected: Count,
        global_exceptions: PatternList,
        detect_layer_fn: Callable[[FilePath, FilePath], LayerNameVO | None],
    ):
        """Resolve the expected word count and exception list for *f*.

        Returns (expected, layer_exceptions, layer_name) tuple.
        """
        layer_vo = detect_layer_fn(f, root_dir)
        layer_name = str(layer_vo) if layer_vo else None

        expected = global_expected  # Fixed: global_expected is Count VO with __int__
        layer_exceptions = list(global_exceptions.values)
        if layer_name and layer_name in layer_map:
            layer_cfg = layer_map[layer_name]
            if layer_cfg.word_count is not None:
                expected = layer_cfg.word_count  # This is int, not Count
            if hasattr(layer_cfg, "exceptions") and layer_cfg.exceptions:
                layer_exceptions.extend(layer_cfg.exceptions.values)
        return expected, layer_exceptions, layer_name

    @staticmethod
    def _should_skip_file(basename: str, layer_exceptions: list[str]) -> bool:
        """Return True if *basename* is an exception or a known special file."""
        if basename in layer_exceptions:
            return True
        if basename in ("__init__.py", "main.py", "py.typed"):
            return True
        return False

    def check_file_naming(
        self,
        files: FilePathList,
        root_dir: FilePath,
        layer_map: LayerMapVO,
        global_expected: Count,
        global_exceptions: PatternList,
        results: LintResultList,
        detect_layer_fn: Callable[[FilePath, FilePath], LayerNameVO | None],
    ) -> None:
        """Check that files have the correct number of underscores."""
        for f in files:
            expected, layer_exceptions, layer_name = self._resolve_layer_config(
                f,
                root_dir,
                layer_map,
                global_expected,
                global_exceptions,
                detect_layer_fn,
            )

            basename = str(f).split("/")[-1]
            if self._should_skip_file(basename, layer_exceptions):
                continue

            stem = basename.replace(".py", "")
            # Word count is underscores + 1
            actual_words = stem.count("_") + 1
            if actual_words != int(expected):
                results.append(
                    LintResult(
                        file=f,
                        line=LineNumber(value=1),
                        column=ColumnNumber(value=1),
                        message=LintMessage(
                            value=f"File '{basename}' has {actual_words} words, "
                            f"expected {expected} for layer '{layer_name}'"
                        ),
                        severity=Severity.HIGH,
                        code=ErrorCode(code="NAMING_WORD_COUNT"),
                        source=AdapterName(value="architecture"),
                    )
                )

    def _validate_class_name(
        self,
        f: FilePath,
        cls_info: dict,
        results: LintResultList,
    ) -> None:
        """Append a LintResult if *cls_info* name is not PascalCase."""
        name = cls_info["name"]
        if not name[0].isupper() or "_" in name:
            results.append(
                LintResult(
                    file=f,
                    line=LineNumber(value=cls_info["line"]),
                    column=ColumnNumber(value=cls_info["column"]),
                    message=LintMessage(value=f"Class '{name}' should be PascalCase"),
                    severity=Severity.HIGH,
                    code=ErrorCode(code="NAMING_CLASS_PASCAL_CASE"),
                    source=AdapterName(value="architecture"),
                )
            )

    def check_class_naming(
        self,
        files: FilePathList,
        results: LintResultList,
        source_parser: ISourceParserPort,
    ) -> None:
        """Check that classes follow PascalCase and match file name if single class."""
        for f in files:
            classes = source_parser.get_class_definitions(f)
            for cls_info in classes:
                self._validate_class_name(f, cls_info, results)

    def _validate_function_name(
        self,
        f: FilePath,
        func_info: dict,
        results: LintResultList,
    ) -> None:
        """Append a LintResult if *func_info* name is not snake_case."""
        name = func_info["name"]
        if name.startswith("__") and name.endswith("__"):
            return

        if not self._has_snake_case(name):
            results.append(
                LintResult(
                    file=f,
                    line=LineNumber(value=func_info["line"]),
                    column=ColumnNumber(value=func_info["column"]),
                    message=LintMessage(
                        value=f"Function '{name}' should be snake_case"
                    ),
                    severity=Severity.HIGH,
                    code=ErrorCode(code="NAMING_FUNCTION_SNAKE_CASE"),
                    source=AdapterName(value="architecture"),
                )
            )

    def check_function_naming(
        self,
        files: FilePathList,
        results: LintResultList,
        source_parser: ISourceParserPort,
    ) -> None:
        """Check that functions follow snake_case."""
        for f in files:
            functions = source_parser.get_function_definitions(f)
            for func_info in functions:
                self._validate_function_name(f, func_info, results)
