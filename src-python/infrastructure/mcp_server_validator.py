"""MCP Server Validator — Input validation for MCP tools."""

from __future__ import annotations

import logging
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from mcp.server.fastmcp.exceptions import ToolError

from ..taxonomy import (
    ErrorMessage,
    BooleanVO,
    FieldName,
    ContentString,
)
from .mcp_server_constants import (
    MAX_PATH_LENGTH,
    MAX_PATH_DEPTH,
    MAX_BATCH_SIZE,
    MAX_STRING_LENGTH,
)

logger = logging.getLogger("infrastructure.mcp_server_validator")


class ValidationErrorType(str, Enum):
    PATH_TRAVERSAL = "path_traversal"
    PATH_TOO_LONG = "path_too_long"
    PATH_TOO_DEEP = "path_too_deep"
    PATH_DEPTH_EXCEEDED = "path_depth_exceeded"
    TOO_MANY_FILES = "too_many_files"
    INVALID_EXTENSION = "invalid_extension_for_analysis"
    STRING_TOO_LONG = "string_too_long"
    UNSAFE_INPUT = "unsafe_input"
    MISSING_REQUIRED_INPUT = "missing_required_input"


@dataclass
class ValidationError:
    """Structured validation error for MCP tool input."""

    type: ValidationErrorType
    message: ErrorMessage
    field: FieldName = FieldName(value="")

    def to_text(self) -> ContentString:
        msg = str(self.message.value)
        fld = str(self.field.value)
        if fld:
            return ContentString(value=f"Validation error ({self.type.value}) in '{fld}': {msg}")
        return ContentString(value=f"Validation error ({self.type.value}): {msg}")


@dataclass
class ValidationResult:
    """Result of input validation."""

    is_valid: BooleanVO
    errors: list[ValidationError] = field(default_factory=list)

    @property
    def error_text(self) -> ContentString:
        if self.is_valid.value:
            return ContentString(value="")
        return ContentString(value="\n".join(str(e.to_text().value) for e in self.errors))

    def raise_if_invalid(self) -> None:
        if not self.is_valid.value:
            raise ToolError(str(self.error_text.value))


# ── Path validation ────────────────────────────────────────────────────────


def _check_path_length(user_input: ContentString, field: FieldName) -> ValidationError | None:
    """Check path length constraint."""
    input_str = str(user_input.value)
    if len(input_str) > MAX_PATH_LENGTH:
        return ValidationError(
            type=ValidationErrorType.PATH_TOO_LONG,
            message=ErrorMessage(value=f"Path length {len(input_str)} exceeds max {MAX_PATH_LENGTH}"),
            field=field,
        )
    return None


def _check_path_depth(user_input: ContentString, field: FieldName) -> ValidationError | None:
    """Check path depth constraint."""
    input_str = str(user_input.value)
    depth = len(Path(input_str).parts)
    if depth > MAX_PATH_DEPTH:
        return ValidationError(
            type=ValidationErrorType.PATH_DEPTH_EXCEEDED,
            message=ErrorMessage(value=f"Path depth {depth} exceeds max {MAX_PATH_DEPTH}"),
            field=field,
        )
    return None


def _check_path_traversal(user_input: ContentString, allowed_root: Path, field: FieldName) -> ValidationError | None:
    """Check path does not escape allowed root."""
    try:
        resolved = (allowed_root / str(user_input.value)).resolve()
        if not resolved.is_relative_to(allowed_root.resolve()):
            return ValidationError(
                type=ValidationErrorType.PATH_TRAVERSAL,
                message=ErrorMessage(value=f"Path escapes allowed root: {user_input.value}"),
                field=field,
            )
    except (OSError, ValueError):
        return ValidationError(
            type=ValidationErrorType.UNSAFE_INPUT,
            message=ErrorMessage(value=f"Cannot resolve path: {user_input.value}"),
            field=field,
        )
    return None


def _check_path_exists(user_input: ContentString, allowed_root: Path | None, field: FieldName) -> ValidationError | None:
    """Check that the path exists."""
    try:
        p_str = str(user_input.value)
        if allowed_root is not None and not Path(p_str).is_absolute():
            resolved = (allowed_root / p_str).resolve()
        else:
            resolved = Path(p_str).resolve()
        if not resolved.exists():
            return ValidationError(
                type=ValidationErrorType.UNSAFE_INPUT,
                message=ErrorMessage(value=f"Path does not exist: {p_str}"),
                field=field,
            )
    except (OSError, ValueError):
        pass
    return None


def validate_path(
    user_input: ContentString,
    allowed_root: Path | None = None,
    must_exist: bool = True,
    field_name: FieldName = FieldName(value="path"),
) -> ValidationResult:
    """Validate a single path input for MCP tool safety."""
    errors: list[ValidationError] = []

    checks = [
        _check_path_length(user_input, field_name),
        _check_path_depth(user_input, field_name),
        _check_path_traversal(user_input, allowed_root, field_name) if allowed_root else None,
        _check_path_exists(user_input, allowed_root, field_name) if must_exist else None,
    ]

    for err in checks:
        if err is not None:
            errors.append(err)

    return ValidationResult(is_valid=BooleanVO(value=len(errors) == 0), errors=errors)


def validate_paths_batch(
    paths: list[ContentString],
    allowed_root: Path | None = None,
    must_exist: bool = False,
) -> ValidationResult:
    """Validate a batch of paths at once."""
    errors: list[ValidationError] = []
    if len(paths) > MAX_BATCH_SIZE:
        errors.append(
            ValidationError(
                type=ValidationErrorType.TOO_MANY_FILES,
                message=ErrorMessage(value=f"Too many paths: {len(paths)} > {MAX_BATCH_SIZE}"),
                field=FieldName(value="paths"),
            )
        )
    for i, p in enumerate(paths):
        result = validate_path(
            p,
            allowed_root=allowed_root,
            field_name=FieldName(value=f"paths[{i}]"),
            must_exist=must_exist,
        )
        errors.extend(result.errors)
    return ValidationResult(is_valid=BooleanVO(value=len(errors) == 0), errors=errors)


def validate_string_input(
    value: str,
    max_length: int = MAX_STRING_LENGTH,
    field_name: str = "input",
) -> ValidationResult:
    """Validate a string input for length limits."""
    if len(value) > max_length:
        return ValidationResult(
            is_valid=BooleanVO(value=False),
            errors=[
                ValidationError(
                    type=ValidationErrorType.STRING_TOO_LONG,
                    message=ErrorMessage(value=f"Input length {len(value)} exceeds max {max_length}"),
                    field=FieldName(value=field_name),
                )
            ],
        )
    # Check for null bytes
    if "\x00" in value:
        return ValidationResult(
            is_valid=BooleanVO(value=False),
            errors=[
                ValidationError(
                    type=ValidationErrorType.UNSAFE_INPUT,
                    message=ErrorMessage(value="Input contains null bytes"),
                    field=FieldName(value=field_name),
                )
            ],
        )
    return ValidationResult(is_valid=BooleanVO(value=True))
