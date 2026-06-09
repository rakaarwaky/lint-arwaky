# 📄 Feature Requirements Document (FRD)
**Feature Name:** Code Duplication Detection (`duplicates` subcommand)
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
This document defines the code duplication detection feature that identifies duplicate or near-duplicate code blocks across the project. It supports token-based and AST-based comparison to find exact and semantic duplicates.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli duplicates [path]` — detect duplicate code blocks
- Token-based comparison (normalized source tokens)
- AST-based comparison (structure-based, ignores formatting)
- Minimum duplicate length: configurable (default 10 lines / 50 tokens)
- Report: file pairs with similarity percentage and duplicated lines

**Out-of-Scope:**
- Third-party duplicate detection tools (e.g., PMD CPD)
- Cross-project duplication analysis
- Automated refactoring suggestions

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Duplicate** | Two or more code blocks with identical or near-identical content |
| **Token** | Lexical unit after normalization (whitespace/comment stripped) |
| **AST** | Abstract Syntax Tree — structure-based representation of code |
| **Similarity %** | Jaccard similarity of token sets between two code blocks |
| **Minimum length** | Minimum lines or tokens to consider a block a duplicate |

## 3. Feature Overview
### 3.1 Background & Problem
Duplicated code increases maintenance burden — bugs fixed in one location must be fixed in all copies. Manual detection is impractical for large codebases. Existing tools (PMD, Simian) are Java-centric and hard to integrate.

### 3.2 Business Goals
- Reduce code duplication across the codebase
- Enforce DRY (Don't Repeat Yourself) principle
- Integrate duplication detection into CI/CD pipeline
- Provide actionable reports with file:line references

### 3.3 Target Users
- **Developers**: Find and eliminate duplicated code
- **Tech Leads**: Set duplication thresholds for code reviews
- **Code Reviewers**: Verify PRs don't introduce new duplicates

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `lint-arwaky-cli duplicates .` to find duplicate code in my project, so I can refactor to DRY.
- **US-002:** As a tech lead, I want to set a maximum duplication threshold in config, so CI fails when duplication exceeds the limit.
- **US-003:** As a developer, I want to see both files and line numbers for each duplicate block, so I can find and fix them.

### 4.2 Use Cases & Workflow
**Duplication Detection Pipeline:**
```
lint-arwaky-cli duplicates .
  │
  ├─► 1. Collect all source files (by language)
  │
  ├─► 2. Tokenize each file (strip whitespace/comments)
  │
  ├─► 3. Sliding window comparison
  │     ├── Compare file A vs file B token sequences
  │     └── Identify runs of matching tokens ≥ min_length
  │
  ├─► 4. Compute similarity
  │     └── Jaccard similarity = intersection / union
  │
  └─► 5. Report duplicates
        "src/utils.rs:10-25  src/helpers.rs:30-45  Similarity: 85%"
```

**Example Output:**
```
$ lint-arwaky-cli duplicates .
🔁 Code Duplication Detection
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Duplicate Block #1 — Similarity: 92%
    src/utils.rs:10-25  ───  src/helpers.rs:30-45
    ┌─────────────────────────────────────────┐
    │ fn validate_email(email: &str) -> bool  │  ← utils.rs:10
    │     email.contains('@')                  │
    │         && email.contains('.')           │
    │ }                                        │
    └─────────────────────────────────────────┘

  Duplicate Block #2 — Similarity: 78%
    src/parser.rs:55-70  ───  src/lexer.rs:20-35

  Summary: 2 duplicate blocks found
  Total duplicated lines: 30 / 1500 (2.0%)
```

### 4.3 Business Rules
- Min duplicate length: 10 lines or 50 tokens (configurable via config file)
- Similarity threshold: ≥ 80% to flag as duplicate (configurable)
- Exact duplicates (100% similarity) are always flagged
- Comments and blank lines stripped before comparison
- Generated files (protobuf, bindings) excluded

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Analysis time for 100 files (10K LOC) | < 10s |
| NFR-002 | Tokenization throughput | > 1 MB/s |
| NFR-003 | Pairwise comparison (N=100) | < 5s |
| NFR-004 | Memory usage for 100 files | < 256 MB |

## 6. UI/UX Requirements
```
$ lint-arwaky-cli duplicates .
🔁 Code Duplication Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Duplicate blocks: 3
  Duplicated lines:  42 / 2100 (2.0%)
  ─────────────────────────────────────────────────
  #1  92%  src/utils.rs:10-25  ↔  src/helpers.rs:30-45
  #2  85%  src/api/handler.rs:60-78  ↔  src/api/controller.rs:15-33
  #3  81%  src/db/query.rs:5-18  ↔  src/db/cache.rs:40-53
  ─────────────────────────────────────────────────
  Threshold: ≥ 80% similarity | Min length: 10 lines
  Status: 1 new duplicate block introduced since last run
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Two files with 90% identical code blocks | `duplicates .` runs | Duplicate block reported with 90% similarity | Pending Review |
| AC-002 | Code blocks differ only in whitespace/comments | Tokenization step runs | Blocks identified as exact duplicates (100%) | Pending Review |
| AC-003 | No duplicate blocks exist | `duplicates .` runs | "No duplicate blocks found" message | Pending Review |
| AC-004 | Duplicate block is 8 lines (below 10-line threshold) | `duplicates .` runs | Block not reported | Pending Review |
| AC-005 | Config overrides min-length to 5 lines | `duplicates .` runs | 8-line block now reported | Pending Review |

## 8. Empirical Findings (Code Audit)
### 8.1 Current Implementation
| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| Duplicates subcommand | `code-analysis/` | — | **NOT IMPLEMENTED** |
| Tokenizer | `source-parsing/` | — | **NOT IMPLEMENTED** |
| Duplicate finder engine | `code-analysis/` | — | **NOT IMPLEMENTED** |

### 8.2 What Needs to Be Added
- `handle_duplicates()` handler
- Language-aware tokenizer (whitespace/comment stripping)
- Sliding window comparison algorithm
- Similarity computation (Jaccard)
- Configurable thresholds (min-length, similarity %)

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Lint Core) | Core lint engine used for file collection | Engine changes may affect file collection | Interface-based dependency |
| Source parser | Tokenization requires language-aware parsing | Complex edge cases in token normalization | Incremental improvement, start with simple tokenizer |

## 10. Appendices
- Source: `code-analysis/` feature folder
- Tokenizer: `source-parsing/infrastructure_tokenizer.rs`
- Config keys: `duplication.min_lines`, `duplication.similarity_threshold`
