// PURPOSE: The ten capability seams of the doc-rules feature — one protocol
// trait per AES document type (PRD, FRD, AGENTS, DATA, ROADMAP, README,
// DESIGN, ARCHITECTURE, CONTRIBUTING, BACKLOG). Each trait answers ALL
// invariants scoped to its document type (AES601–AES607), so one capability
// file owns one document end to end.
//
// The agent behind the aggregate injects all ten, hands each the same audit
// context, and merges what they report; no consumer ever sees a protocol.
// Cross-cutting invariants that are not scoped to a single document
// (required root-document presence, member-document presence) live in shared
// utilities and are invoked by the orchestrator directly.
use crate::taxonomy_doc_audit_context_vo::DocAuditContext;
use crate::taxonomy_doc_rules_request::DocFinding;

/// PRD.md — AES601 PR-ID parity with ROADMAP, AES603 spec purity, AES604
/// crosslinks (ROADMAP, ARCHITECTURE), AES605 heading, AES606 entry tables
/// under `Project Requirements` plus template body, AES607 PR folder
/// ownership.
pub trait IPrdDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to PRD.md in *context*.
    fn audit_prd(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// FRD.md — AES601 FR-ID format, FR protocol parity, promised API methods;
/// AES602 section order; AES603 spec purity; AES604 crosslinks
/// (PRD, BACKLOG, ROADMAP, ARCHITECTURE); AES605 heading; AES606 entry
/// tables under `Functional Requirements` plus template body.
pub trait IFrdDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to FRD.md in *context*.
    fn audit_frd(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// AGENTS.md — AES604 crosslinks (README, CONTRIBUTING, ARCHITECTURE,
/// ROADMAP, PRD); AES605 heading plus verbatim line drift; AES606 entry
/// tables (`Runtime`) plus template body.
pub trait IAgentsDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to AGENTS.md in *context*.
    fn audit_agents(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// DATA.md — AES601 bare-ID format; AES602 section order; AES603 spec
/// purity; AES604 crosslinks (PRD, BACKLOG, ROADMAP, ARCHITECTURE); AES605
/// heading (DT-prefixed H2s allowed); AES606 entry tables under `Data
/// Requirements` plus template body.
pub trait IDataDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to DATA.md in *context*.
    fn audit_data(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// ROADMAP.md — AES601 PR-ID parity with PRD; AES603 spec purity; AES604
/// crosslinks (PRD, ARCHITECTURE); AES605 heading; AES606 entry tables
/// under `Roadmap Status` plus template body.
pub trait IRoadmapDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to ROADMAP.md in *context*.
    fn audit_roadmap(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// README.md — AES604 crosslinks (none; placeholder table); AES605 heading;
/// AES606 entry tables (`Quick Start`, `Reference`) plus template body.
pub trait IReadmeDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to README.md in *context*.
    fn audit_readme(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// DESIGN.md — AES601 DS-ID alignment with BACKLOG; AES604 crosslinks
/// (PRD, BACKLOG, ROADMAP, ARCHITECTURE); AES605 heading; AES606 entry
/// tables under `Design Requirements` plus `no_extra_table` / `H3 unexpected`
/// sweeps plus template body.
pub trait IDesignDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to DESIGN.md in *context*.
    fn audit_design(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// ARCHITECTURE.md — AES605 heading; AES606 template body.
pub trait IArchitectureDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to ARCHITECTURE.md in
    /// *context*.
    fn audit_architecture(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// CONTRIBUTING.md — AES604 crosslinks (PRD, ARCHITECTURE); AES605 heading;
/// AES606 entry tables (`Workflow`, `Commit Convention`, `Reference`) plus
/// template body.
pub trait IContributingDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to CONTRIBUTING.md in
    /// *context*.
    fn audit_contributing(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// BACKLOG.md — AES601 BACKLOG ID-level alignment against its paired spec
/// union (FRD/DATA/DESIGN per role, or root PRD/ROADMAP); AES604 role-aware
/// crosslinks; AES605 heading; AES606 entry tables under `Backlog Status`
/// plus template body.
pub trait IBacklogDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to BACKLOG.md in *context*.
    fn audit_backlog(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}
