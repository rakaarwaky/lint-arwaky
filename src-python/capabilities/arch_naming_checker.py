"""arch_naming_checker — Architectural naming convention checks."""

import re
from ..taxonomy import (
    ColumnNumber,
    ErrorCode,
    FilePath,
    FilePathList,
    LintMessage,
    LintResult,
    LintResultList,
    LineNumber,
    Severity,
    AdapterName,
    SymbolName,
    Count,
    LayerNameVO,
    SuffixVO,
)
from ..contract import INamingCheckerProtocol


class ArchNamingChecker(INamingCheckerProtocol):
    """Handles naming-related architectural checks."""

    def check_file_naming(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        global_expected = analyzer.config.naming.word_count
        for f in files:
            self._check_file_naming_one(f, analyzer, root_dir, results, global_expected)

    def _detect_layer_for_naming(
        self,
        f: FilePath,
        analyzer,
        root_dir: FilePath,
        global_expected: Count,
    ) -> tuple:
        """Detect layer info and determine expected word count for naming check."""
        layer_vo = analyzer._detect_layer(f, root_dir)
        expected = global_expected
        layer_cfg = None
        if layer_vo and layer_vo in analyzer.layer_map:
            layer_cfg = analyzer.layer_map[layer_vo]
            if layer_cfg.word_count is not None:
                expected = Count(value=int(layer_cfg.word_count))
        return layer_vo, layer_cfg, expected

    def _check_layer_patterns(
        self,
        stem: SymbolName,
        expected_words: int,
        analyzer,
        layer_cfg,
    ) -> str | None:
        """Check if filename stem matches the naming regex pattern. Returns violation message or None."""
        word_pattern = r"[a-z0-9]+"
        naming_regex = rf"^{word_pattern}(_{word_pattern}){{{expected_words - 1}}}$"
        if re.match(naming_regex, str(stem)):
            return None
        violation_msg = analyzer.config.naming.word_count_violation_message
        if layer_cfg and layer_cfg.word_count_violation_message:
            violation_msg = layer_cfg.word_count_violation_message
        return violation_msg

    def _create_naming_violation(
        self,
        f: FilePath,
        expected_words: int,
        violation_msg: str | None,
        results: LintResultList,
    ) -> None:
        """Append a naming convention violation result."""
        default_msg = LintMessage(
            value=(
                f"AES003 NAMING_CONVENTION: Filename does not follow the {expected_words}-word underscore-separated pattern.\n"
                "WHY? Strict three-word names ensure architectural consistency and prevent naming ambiguity.\n"
                f"FIX: Rename the file to exactly {expected_words} words separated by underscores (e.g., word1_word2_word3.py)."
            )
        )
        results.append(
            LintResult(
                file=f,
                line=LineNumber(value=0),
                column=ColumnNumber(value=0),
                code=ErrorCode(code="AES003"),
                message=violation_msg or default_msg,
                source=AdapterName(value="architecture"),
                severity=Severity.HIGH,
            )
        )

    def _check_file_naming_one(
        self,
        f: FilePath,
        analyzer,
        root_dir: FilePath,
        results: LintResultList,
        global_expected: Count,
    ) -> None:
        layer_name, layer_cfg, expected = self._detect_layer_for_naming(
            f, analyzer, root_dir, global_expected
        )

        basename = SymbolName(value=str(analyzer.fs.get_basename(f)))
        if analyzer.parser.is_entry_point(f) or analyzer.parser.is_barrel_file(f):
            return

        # Check if file is in global naming exceptions
        global_exceptions = (
            analyzer.config.naming.exceptions
            if hasattr(analyzer.config.naming, "exceptions")
            else None
        )
        if global_exceptions and str(basename) in global_exceptions.values:
            return

        # Check if file is in layer-specific exceptions
        if (
            layer_cfg
            and layer_cfg.exceptions.values
            and str(basename) in layer_cfg.exceptions.values
        ):
            return

        stem = analyzer.parser.get_stem(f)
        expected_words = int(expected)  # Fixed: expected is Count VO with __int__
        violation_msg = self._check_layer_patterns(
            stem, expected_words, analyzer, layer_cfg
        )
        if violation_msg is not None:
            self._create_naming_violation(f, expected_words, violation_msg, results)

    def _collect_suffix_candidates(
        self,
        f: FilePath,
        analyzer,
        root_dir: FilePath,
    ) -> tuple | None:
        """Extract suffix candidates from a file for domain suffix checking.
        Returns (layer_vo, definition, basename, suffix) or None if file should be skipped.
        """
        layer_vo = analyzer._detect_layer(f, root_dir)
        if not layer_vo:
            return None
        definition = analyzer.layer_map[layer_vo]
        if analyzer.parser.is_barrel_file(f) or analyzer.parser.is_entry_point(f):
            return None
        basename = analyzer.parser.get_stem(f)
        # Omit files in the layer's exceptions list
        basename_with_ext = SymbolName(value=str(analyzer.fs.get_basename(f)))
        if basename_with_ext.value in definition.exceptions.values:
            return None
        parts = str(basename).rsplit("_", 1)
        suffix = SuffixVO(value=parts[1]) if len(parts) == 2 else None
        return layer_vo, definition, basename, suffix

    def _verify_suffix_against_policy(
        self,
        f: FilePath,
        layer_name: LayerNameVO,
        definition,
        suffix: SuffixVO | None,
        results: LintResultList,
    ) -> None:
        """Check suffix against forbidden, allowed, and strict policies. Appends violations as needed."""
        # Check forbidden suffix (elif to avoid double-reporting with strict enum check)
        if suffix and str(suffix) in definition.forbidden_suffix.values:
            default_msg = LintMessage(
                value=(
                    "AES011 SUFFIX_MISMATCH: File uses a forbidden suffix for this layer.\n"
                    "WHY? Forbidden suffixes prevent technical concepts from leaking into domain layers.\n"
                    "FIX: Rename the file to use an allowed suffix or move it to the correct layer."
                )
            )
            self._append_suffix_violation(
                f, layer_name, definition, default_msg, results
            )

        # Check strict suffix policy (elif: only when no forbidden match)
        elif definition.suffix_policy.value == "strict" and (
            not suffix or str(suffix) not in definition.allowed_suffix.values
        ):
            default_msg = LintMessage(
                value=(
                    "AES011 SUFFIX_MISMATCH: File is missing a required strict suffix for this layer.\n"
                    "WHY? Strict suffixes ensure that every component in this layer has a clear, standardized role.\n"
                    f"FIX: Add one of the required suffixes: {', '.join(definition.allowed_suffix.values)}."
                )
            )
            self._append_suffix_violation(
                f, layer_name, definition, default_msg, results
            )

        # Check suffix policy string (non-enum) strict matching
        if definition.suffix_policy == "strict" and (
            not suffix or str(suffix) not in definition.allowed_suffix.values
        ):
            default_msg = LintMessage(
                value="Suffix does not match allowed patterns for this layer."
            )
            violation_msg = definition.suffix_violation_message
            results.append(
                LintResult(
                    file=f,
                    line=LineNumber(value=0),
                    column=ColumnNumber(value=0),
                    code=ErrorCode(code="AES011"),
                    message=violation_msg or default_msg,
                    source=AdapterName(value="architecture"),
                    severity=Severity.HIGH,
                )
            )

    def check_domain_suffixes(
        self,
        analyzer,
        files: FilePathList,
        root_dir: FilePath,
        results: LintResultList,
    ) -> None:
        for f in files:
            candidates = self._collect_suffix_candidates(f, analyzer, root_dir)
            if candidates is None:
                continue
            layer_name, definition, basename, suffix = candidates
            self._verify_suffix_against_policy(
                f, layer_name, definition, suffix, results
            )

    def _append_suffix_violation(
        self,
        f: FilePath,
        layer_name: LayerNameVO,
        definition,
        default_msg: LintMessage,
        results: LintResultList,
    ) -> None:
        violation_msg = definition.suffix_violation_message
        results.append(
            LintResult(
                file=f,
                line=LineNumber(value=0),
                column=ColumnNumber(value=0),
                code=ErrorCode(code="AES010"),
                message=violation_msg or default_msg,
                source=AdapterName(value="architecture"),
                severity=Severity.HIGH,
            )
        )
