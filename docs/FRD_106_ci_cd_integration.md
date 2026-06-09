# 📄 Feature Requirements Document (FRD)
**Feature Name:** CI/CD Integration — OIDC & SLSA Provenance
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
Defines CI/CD integration with OpenID Connect (OIDC) authentication for cloud deployments and SLSA Provenance generation for supply chain security.

### 2.2 Scope
**In-Scope:** OIDC token exchange (GitHub Actions → cloud), SLSA provenance attestation generation, GitHub Actions workflow templates.
**Out-of-Scope:** Deployment orchestration, secret management, self-hosted runners.

### 2.3 Glossary
| Term | Definition |
|------|------------|
| **OIDC** | OpenID Connect — federated identity protocol for CI/CD |
| **SLSA** | Supply-chain Levels for Software Artifacts |
| **Provenance** | Verifiable metadata about build process |

## 3. Feature Overview
### 3.1 Background & Problem
Build artifacts lacked attestation, and cloud deployments required long-lived secrets. OIDC eliminates static secrets; SLSA provides verifiable build integrity.

### 3.2 Business Goals
- Eliminate long-lived cloud credentials via OIDC
- Achieve SLSA Level 2 provenance for releases
- Generate verifiable build attestations

### 3.3 Target Users
- DevOps engineers configuring CI/CD pipelines
- Security auditors verifying supply chain integrity

## 4. Functional Requirements
### 4.1 User Stories
- **US-001:** As a DevOps engineer, I want GitHub Actions to authenticate to cloud via OIDC, so I don't manage long-lived secrets.
- **US-002:** As a security auditor, I want SLSA provenance for each release, so I can verify build integrity.

### 4.2 Use Cases & Workflow
```
Release workflow:
  1. GitHub Actions requests OIDC token from GitHub's OIDC provider
  2. Token exchanged for cloud provider access token
  3. Build completes, provenance attestation generated
  4. Attestation uploaded to artifact registry
```

### 4.3 Business Rules
- OIDC tokens obtained via `actions/oidc-subs` in GitHub Actions
- SLSA provenance generated using `slsa-framework/slsa-github-generator`
- Provenance attached to GitHub release artifacts

## 5. Non-Functional Requirements
| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | OIDC token acquisition | < 5s |
| NFR-002 | Provenance generation | < 30s |
| NFR-003 | Provenance verification | < 10s |

## 6. UI/UX Requirements
No UI. CI/CD is automated in GitHub Actions workflows.

## 7. Acceptance Criteria
| ID | Given | When | Then | Status |
|----|-------|------|------|--------|
| AC-001 | Release workflow triggers | GitHub Actions runs on tag push | OIDC token acquired, build succeeds | Pending Review |
| AC-002 | Build completes | After successful compilation | SLSA provenance `.intoto.jsonl` generated | Pending Review |
| AC-003 | Provenance uploaded | Release published | Provenance attached as release asset | Pending Review |

## 8. Empirical Findings
### 8.1 Current Implementation
| Component | Location | Status |
|-----------|----------|--------|
| GitHub Actions workflow | `.github/workflows/release.yml` | Pending Review |
| OIDC configuration | `.github/workflows/oidc-setup.yml` | Pending Review |
| SLSA generator | `.github/workflows/slsa.yml` | Pending Review |

## 9. Dependencies & Risks
| Dependency | Description | Risk | Mitigation |
|------------|-------------|------|------------|
| FR-100 (MCP Server) | CI/CD may invoke MCP for testing | MCP health check ensures connectivity | |
| GitHub OIDC provider | Requires GitHub Actions | OIDC tied to GitHub ecosystem | |

## 10. Appendices
- `.github/workflows/release.yml`
- `docs/SLSA.md` — Provenance details
