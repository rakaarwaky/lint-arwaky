# 📄 Feature Requirements Document (FRD)
**Feature Name:** Generate Naming Variants — Case Conversion
**Product:** Lint Arwaky v1.10.2
**Author:** Raka
**Date:** 09/06/2026
**Version:** v1.0

## 1. Document Control
| Version | Date | Author | Description of Changes | Approved By |
|---------|------|--------|----------------------|-------------|
| v1.0 | 09/06/2026 | Raka | Initial document creation | [Stakeholder] |

## 2. Introduction
### 2.1 Purpose
Defines the naming variant generation feature that converts identifiers between five naming conventions: `snake_case`, `camelCase`, `PascalCase`, `SCREAMING_SNAKE_CASE`, and `kebab-case`.

### 2.2 Scope
**In-Scope:** Conversion between 5 naming conventions, batch conversion of multiple identifiers, single identifier conversion.
**Out-of-Scope:** Language-specific naming rules (Rust prefers snake_case for vars, etc.), style guide enforcement.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **snake_case** | Words separated by underscores, all lowercase |
| **camelCase** | First word lowercase, subsequent words capitalized |
| **PascalCase** | All words capitalized, no separators |
| **SCREAMING_SNAKE_CASE** | Words separated by underscores, all uppercase |
| **kebab-case** | Words separated by hyphens, all lowercase |

## 3. Feature Overview
### 3.1 Background & Problem
Developers frequently needed to convert identifiers between naming conventions when refactoring or when dealing with cross-language projects (Rust→JS→Python).

### 3.2 Business Goals
- Provide accurate conversion between all 5 common naming conventions
- Support batch conversion from CLI
- Enable programmatic access via MCP tools

### 3.3 Target Users
- Developers refactoring across naming conventions
- AI agents generating code in target language style

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to convert `my_function_name` to `camelCase` (→ `myFunctionName`).
- **US-002:** As a developer, I want to convert a list of variable names to `SCREAMING_SNAKE_CASE` for constants.

### 4.2 Use Cases & Workflow
```
Input:  convert "my_function_name" to PascalCase
Output: "MyFunctionName"

Input:  convert-batch ["user_id", "user_name", "user_email"] to camelCase
Output: ["userId", "userName", "userEmail"]
```

### 4.3 Business Rules
- Input convention auto-detected by analyzing separators and casing
- Output convention explicitly specified
- Unknown conventions → error with supported list
- Single-character identifiers passed through unchanged

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Single conversion | < 1ms |
| NFR-002 | Batch conversion (1000 names) | < 10ms |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli naming-variants "my_function_name" --to camelCase
 Input:  my_function_name
 Output: myFunctionName

 Variants:
   snake_case:        my_function_name
   camelCase:         myFunctionName
   PascalCase:        MyFunctionName
   SCREAMING_SNAKE:   MY_FUNCTION_NAME
   kebab-case:        my-function-name
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | snake_case input | Convert to camelCase | Correct camelCase output | Pending Review |
| AC-002 | camelCase input | Convert to snake_case | Correct snake_case output | Pending Review |
| AC-003 | PascalCase input | Convert to SCREAMING_SNAKE | Correct all-caps output | Pending Review |
| AC-004 | kebab-case input | Convert to PascalCase | Correct PascalCase output | Pending Review |
| AC-005 | Single letter identifier | Convert any | Pass through unchanged | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Variant generator | `semantic-analysis/capabilities_variant_generator.rs` | Pending Review |
| Variant CLI command | `semantic-analysis/surface_variant_command.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-003 (Parsing) | Identifier extraction from source | Purely string-based; no parsing needed for this feature | Self-contained conversion logic |
| Edge cases | Acronyms, abbreviations (e.g., `parseXML`) | Inconsistent conversion | Rule: consecutive capitals treated as one word |

## 10. Appendices
- `src-rust/semantic-analysis/capabilities_variant_generator.rs`
