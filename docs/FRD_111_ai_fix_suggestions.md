# 📄 Feature Requirements Document (FRD)
**Feature Name:** AI Fix Suggestions — `suggest` Subcommand
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
Defines the `suggest` subcommand that uses AI (local LLM or API) to generate fix suggestions for detected violations beyond simple auto-fixes.

### 2.2 Scope
**In-Scope:** `lint-arwaky-cli suggest <path>`, local LLM support (ollama), API support (OpenAI-compatible), violation context packaging, fix suggestion formatting.
**Out-of-Scope:** Automatic fix application, model training, code generation without review.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **LLM** | Large Language Model for generating fix suggestions |
| **Fix suggestion** | AI-generated code diff to resolve a violation |

## 3. Feature Overview
### 3.1 Background & Problem
Many violations (AES001 imports, AES016 dead inheritance) lack auto-fix. Developers need AI-powered suggestions for complex fixes.

### 3.2 Business Goals
- Reduce manual fix effort for complex violations
- Support both local (ollama) and cloud (OpenAI) LLM backends
- Provide context-rich violation data for accurate suggestions

### 3.3 Target Users
- Developers seeking fix guidance for complex violations
- Teams using local LLMs for data privacy

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a developer, I want to run `suggest .` to get AI-generated fix suggestions for all violations.
- **US-002:** As a privacy-conscious team, I want to use local ollama for suggestions.

### 4.2 Use Cases & Workflow
```
lint-arwaky-cli suggest ./ --model ollama --ollama-url http://localhost:11434
  │
  ├─► Scan project → collect violations
  ├─► For each violation, package context (file, line, surrounding code, rule description)
  ├─► Send to LLM: "Suggest a fix for AES001 import violation at file.rs:42..."
  └─► Display suggestions as code diffs
```

### 4.3 Business Rules
- Default model: ollama (local); override: `--model openai`
- Requires `OPENAI_API_KEY` env var for OpenAI backend
- Suggestions are advisory — never auto-applied

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Suggestion latency (local) | < 10s per violation |
| NFR-002 | Suggestion latency (API) | < 5s per violation |

## 6. UI/UX Requirements
```
 Suggestion for AES001 (layer-rules/capabilities_checker.rs:42):
 ┌─ Before ─────────────────────────────────────┐
 │ use crate::infrastructure_scanner::scan_file; │
 └──────────────────────────────────────────────┘
 ┌─ Suggested Fix ──────────────────────────────┐
 │ use crate::contract_scanner_port::scan_file;  │
 └──────────────────────────────────────────────┘
 Reason: Infrastructure imports forbidden from capabilities.
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Project with violations | `suggest .` | Returns fix suggestions per violation | Pending Review |
| AC-002 | Local ollama model | `suggest . --model ollama` | Uses ollama endpoint | Pending Review |
| AC-003 | No LLM configured | `suggest .` | Error with setup instructions | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| Suggest CLI command | `cli-commands/surface_suggest_command.rs` | Pending Review |
| LLM client adapter | `infrastructure/llm_adapter.rs` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-057 (AI Service) | LLM integration service | API downtime | Fallback to suggestion templates |
| ollama/OpenAI | External LLM | Cost for API | Default to local ollama |

## 10. Appendices
- `src-rust/cli-commands/surface_suggest_command.rs`
