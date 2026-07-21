# FRD — code-analysis

## System Overview

The code-analysis crate enforces general code quality, formatting limits, and clean-coding policies. It protects the codebase from bloated files, empty structures, and duplicate blocks, while guaranteeing zero tolerance for warning/error bypasses.

## Functional Requirements

### FR-001: Maximum File Line Count (AES301)

- **Description**: Files must not exceed the maximum allowed line count.
- **Input**: Source file path
- **Output**: AES301 diagnostic if exceeded
- **Business Rules**:
  - Default max: 1000 lines (configurable per rule)
  - Applies to: Rust, Python, TypeScript, JavaScript
- **Edge Cases**: Files with long comments, generated code
- **Error Handling**: Emit AES301 with actual vs max line count

### FR-002: Minimum File Line Count (AES302)

- **Description**: Files must have minimum length to avoid empty placeholders.
- **Input**: Source file path
- **Output**: AES302 diagnostic if too short
- **Business Rules**:
  - Default min: 10 lines
  - Applies to: Rust, Python, TypeScript, JavaScript
- **Edge Cases**: Config files, entry points
- **Error Handling**: Emit AES302 with actual vs min line count

### FR-003: Mandatory Definitions (AES303)

- **Description**: Source files must declare at least one primary symbol.
- **Input**: Source file path
- **Output**: AES303 diagnostic if no definition found
- **Business Rules**:
  - Rust: struct, enum, trait, type
  - Python: class, def, async def
  - TypeScript: class, interface, type, enum, function
  - JavaScript: class, function, async function
- **Edge Cases**: Empty impl blocks, unit structs
- **Error Handling**: Emit AES303 with expected symbol types

### FR-004: Bypass Detection (AES304)

- **Description**: Detects and flags any attempt to suppress warnings/errors.
- **Input**: Source file path
- **Output**: AES304 diagnostic for each bypass found
- **Business Rules**:
  - Comment bypasses: noqa, type: ignore, eslint-disable
  - Attribute bypasses: #[allow(...)], #[warn(...)]
  - Fatal operations: unwrap(), expect(), panic!, todo!
  - Safe variants NOT flagged: unwrap_or(), unwrap_or_else()
- **Edge Cases**: Nested attributes, conditional compilation
- **Error Handling**: Emit AES304 with bypass type and location

### FR-005: Duplicate Code Detection (AES305)

- **Description**: Compares code blocks and flags identical/highly similar segments.
- **Input**: All workspace source files
- **Output**: AES305 diagnostic for duplicate blocks
- **Business Rules**:
  - Min duplicate lines: 5
  - Threshold: 50% similarity
  - Algorithm: Window-based hashing with normalized lines
- **Edge Cases**: Generated code, boilerplate
- **Error Handling**: Emit AES305 with duplicate file locations

### FR-006: File Read Error Diagnostics (AES000)

- **Description**: Emit diagnostic when file cannot be read or exceeds size limit.
- **Input**: File path
- **Output**: AES000 diagnostic
- **Business Rules**:
  - Max file size: 2 MiB
  - Emit diagnostic instead of silent skip
- **Edge Cases**: Binary files, permission errors
- **Error Handling**: Emit AES000 with error reason

## Data Model / Entity Relationship

```
CodeAnalysisRuleVO {
    rule_code: String
    max_lines: Option<u32>
    min_lines: Option<u32>
    threshold_pct: f64
}

Diagnostic {
    file_path: String
    line: u32
    column: u32
    rule_code: String
    message: String
    severity: Severity
}
```

## API Contract

| Function | Input | Output | Description |
|----------|-------|--------|-------------|
| `check_max_line_count()` | File path, content | Option<Diagnostic> | Check AES301 |
| `check_min_line_count()` | File path, content | Option<Diagnostic> | Check AES302 |
| `check_mandatory_definitions()` | File path, content | Option<Diagnostic> | Check AES303 |
| `check_forbidden_bypass()` | File path, content | Vec<Diagnostic> | Check AES304 |
| `handle_duplicates()` | All files | Vec<Diagnostic> | Check AES305 |

## Integration Points

- **Internal**: config-system (YAML rules), shared (taxonomy VOs)
- **External**: None

## Non-functional Requirements (Detailed)

- Performance: Analyze 1000 files in < 3 seconds
- Memory: O(n) where n = file size
- Accuracy: Zero false positives for valid code

## Test Scenarios / QA Checklist

- [ ] File exceeding max lines fails with AES301
- [ ] File below min lines fails with AES302
- [ ] File without definitions fails with AES303
- [ ] `unwrap()` detected with AES304
- [ ] `#[allow(...)]` detected with AES304
- [ ] Duplicate code detected with AES305
- [ ] Oversized file emits AES000

## Assumptions & Constraints

- Rules are configurable via YAML
- File reading uses memory-mapped I/O for large files
- Duplicate detection uses hash-based comparison

## Glossary

- **AES**: Agentic Engineering System
- **Bypass**: Attempt to suppress warnings/errors
- **Diagnostic**: Violation report with location and rule code

## Reference

- PRD: [PRD.md](../../PRD.md)
