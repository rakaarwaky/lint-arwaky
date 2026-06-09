# 📄 Feature Requirements Document (FRD)
**Feature Name:** CI Mode (`ci [path] --threshold <N>`)  
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
This document defines the CI mode CLI command `ci [path] --threshold <N>`. It runs a full architecture compliance check and exits with a non-zero code if the quality score falls below the specified threshold. Enables CI/CD pipeline gating to prevent merging code that degrades architecture quality.

### 2.2 Scope
**In-Scope:**
- `lint-arwaky-cli ci <path> --threshold <N>` — check with exit code gating
- Score computation (0–100) matching FR-055
- Exit code 0 if score >= threshold
- Exit code 1 if score < threshold
- Exit code 2 if errors occur (config not found, parse failure)
- Any CRITICAL violation → auto-fail (exit 1) regardless of score

**Out-of-Scope:**
- Report generation (handled by FR-058)
- Auto-fixing (handled by FR-057)
- External linter scanning (handled by FR-056)

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **Threshold** | Minimum acceptable quality score (0–100) |
| **Exit code** | Process exit code: 0=pass, 1=fail (score below threshold), 2=error |
| **Auto-fail** | Any CRITICAL violation causes immediate failure |
| **Score** | Quality score from 0–100 computed from all AES violations |

## 3. Feature Overview
### 3.1 Background & Problem
CI pipelines needed a way to block PRs that degrade architectural quality. The `check` command always exits 0 (success) even with violations. There was no threshold mechanism, no auto-fail for critical violations, and no way to enforce a minimum quality bar in CI.

### 3.2 Business Goals
- Enable quality gates in CI/CD pipelines
- Prevent merging code with CRITICAL architecture violations
- Support configurable thresholds per project
- Provide clear exit codes for pipeline integration

### 3.3 Target Users
- **DevOps Engineers**: Integrate into CI pipelines (GitHub Actions, GitLab CI, Jenkins)
- **Engineering Managers**: Set quality bars per project/repository
- **Developers**: Run locally to check PR compliance before pushing

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a CI pipeline, I want `ci . --threshold 80` to exit with code 1 when score is 75, so the pipeline fails and blocks the PR.
- **US-002:** As a developer, I want `ci . --threshold 80` to exit 0 when score is 85, so I know my changes pass the quality bar.
- **US-003:** As an architect, I want ANY CRITICAL violation to auto-fail regardless of score, so critical issues are never merged.

### 4.2 Use Cases & Workflow
**CI Pipeline Integration:**
```
# GitHub Actions
- name: Architecture Compliance
  run: lint-arwaky-cli ci . --threshold 80

# GitLab CI
lint-arwaky:
  script: lint-arwaky-cli ci . --threshold 80

# Jenkins Pipeline
stage('Architecture Check') {
  sh 'lint-arwaky-cli ci . --threshold 80'
}
```

**Exit Code Logic:**
```
lint-arwaky-cli ci /project --threshold 80
  │
  ├─► 1. Run full check (same as FR-055)
  │
  ├─► 2. Compute score
  │
  ├─► 3. Check CRITICAL violations:
  │     ├── Any CRITICAL → exit 1 (auto-fail)
  │     └── No CRITICAL → continue
  │
  ├─► 4. Check threshold:
  │     ├── score >= 80 → exit 0 (pass)
  │     └── score < 80  → exit 1 (fail)
  │
  └─► 5. On error:
        └── Config/parse error → exit 2
```

### 4.3 Business Rules
- Default threshold: 70 (configurable in config YAML under `ci.threshold`)
- CRITICAL auto-fail takes priority over threshold check
- Exit codes: 0 = pass, 1 = fail (below threshold or CRITICAL), 2 = error
- CI mode produces minimal output (score + pass/fail line) for pipeline logs
- Use `--verbose` for detailed violation list in CI output

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | CI check + exit code (1000 files) | < 5s |
| NFR-002 | Exit code correctness | 100% reliable |
| NFR-003 | CRITICAL auto-fail reliability | 100% |

## 6. UI/UX Requirements
CLI output:
```
$ lint-arwaky-cli ci /project --threshold 80
📊 Architecture Compliance CI
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Score: 75.0 / 100  ❌ FAIL

Threshold: 80.0
  ✗ Score below threshold (75.0 < 80.0)

CRITICAL: 0 | HIGH: 2 | MEDIUM: 4 | LOW: 1

Result: FAIL (exit code 1)

$ lint-arwaky-cli ci /project --threshold 80
📊 Architecture Compliance CI
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Score: 92.0 / 100  ✅ PASS

Threshold: 80.0
  ✓ Score meets threshold (92.0 >= 80.0)

CRITICAL: 0 | HIGH: 0 | MEDIUM: 1 | LOW: 2

Result: PASS (exit code 0)
```

With CRITICAL auto-fail:
```
$ lint-arwaky-cli ci /project --threshold 80
📊 Architecture Compliance CI
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Score: 95.0 / 100  ❌ AUTO-FAIL

Threshold: 80.0
  ✓ Score meets threshold (95.0 >= 80.0)

CRITICAL: 1 | HIGH: 0 | MEDIUM: 0 | LOW: 0

⚠️  CRITICAL VIOLATION(S) DETECTED — auto-fail triggered
Result: FAIL (exit code 1)
```

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Score 85, threshold 80 | `ci --threshold 80` runs | exit 0 (pass) | Pending Review |
| AC-002 | Score 75, threshold 80 | `ci --threshold 80` runs | exit 1 (fail) | Pending Review |
| AC-003 | CRITICAL violation, score 95 | `ci --threshold 80` runs | exit 1 (auto-fail) | Pending Review |
| AC-004 | Config parse error | `ci --threshold 80` runs | exit 2 (error) | Pending Review |
| AC-005 | Default threshold when not provided | `ci .` runs | Uses config default or 70 | Pending Review |

## 8. Empirical Findings (Code Audit)

### 8.1 Current Implementation

| Component | Location | Lines | Status |
|-----------|----------|-------|--------|
| CLI ci command | `cli-commands/surface_dev_command.rs` | — | **FULLY IMPLEMENTED** — CLI dispatch with --threshold |
| Score computation | `shared-common/taxonomy_score_vo.rs` | — | **FULLY IMPLEMENTED** |
| CRITICAL auto-fail | `code-analysis/agent_checking_coordinator.rs` | — | **FULLY IMPLEMENTED** |
| Exit code handling | `cli-commands/surface_main_handler.rs` | — | **FULLY IMPLEMENTED** |

### 8.2 Bugs Found

1. **Threshold from CLI overrides config default but config default is 0** — if no threshold flag and no config value, threshold defaults to 0, meaning `ci .` always passes
   - **Impact**: CI mode passes by default if misconfigured
   - **Fix**: Change default threshold to 70 if neither CLI flag nor config value provided

2. **Exit code 2 never returned** — the error handling path returns exit code 1 for all non-zero exits
   - **Impact**: Pipelines can't distinguish "score below threshold" from "runtime error"
   - **Fix**: Map config/parse errors to exit code 2

3. **CRITICAL auto-fail prints warning but still checks threshold** — if CRITICAL present AND score < threshold, only one reason is reported
   - **Impact**: Ambiguous failure reasons in CI logs
   - **Fix**: Report ALL failure reasons (CRITICAL + below threshold)

### 8.3 What Needs to Be Added

- **Default threshold**: Change from 0 to 70
- **Exit code 2**: Config/parse errors → exit 2
- **Multi-reason reporting**: List all reasons for failure in CI output

### 8.4 What to Keep

- **CLI structure** ✅ — clean `ci` command with `--threshold`
- **Score computation** ✅ — correct and consistent with check
- **CRITICAL auto-fail** ✅ — correctly overrides threshold

### 8.5 Empirical Evidence from Test Projects

- `lint-arwaky-cli ci test-project-rust/ --threshold 80` correctly passes/fails based on score
- `lint-arwaky-cli ci . --threshold 100` fails on own codebase (self-lint finds violations)
- Pending Review: Default threshold fix, exit code 2

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-055 (Check) | Full compliance check is the data source | Check bugs affect CI accuracy | Unit test score computation |
| FR-002 (Config) | Threshold from config YAML | Config not loaded → wrong default | Fix default threshold to 70 |

## 10. Appendices
- `src-rust/cli-commands/surface_dev_command.rs` — CLI ci command
- `src-rust/shared-common/taxonomy_score_vo.rs` — Score computation
- `src-rust/cli-commands/surface_main_handler.rs` — Exit code routing
