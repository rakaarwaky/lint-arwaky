// PURPOSE: The ten capability seams of the doc-rules feature — one protocol
// trait per AES document type (PRD, FRD, AGENTS, DATA, ROADMAP, README,
// DESIGN, ARCHITECTURE, CONTRIBUTING, BACKLOG). Each trait answers ALL
// invariants scoped to its document type (AESD01–AESD07), so one capability
// file owns one document end to end.
//
// The agent behind the aggregate injects all ten, hands each the same audit
// context, and merges what they report; no consumer ever sees a protocol.
// Cross-cutting invariants that are not scoped to a single document
// (required root-document presence, member-document presence) live in shared
// utilities and are invoked by the orchestrator directly.
use crate::taxonomy_doc_audit_context_vo::DocAuditContext;
use crate::taxonomy_doc_rules_request::DocFinding;

/// PRD.md — AESD01 PR-ID parity with ROADMAP, AESD03 spec purity, AESD04
/// crosslinks (ROADMAP, ARCHITECTURE), AESD05 heading, AESD06 entry tables
/// under `Project Requirements` plus template body, AESD07 PR folder
/// ownership.
pub trait IPrdDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to PRD.md in *context*.
    fn audit_prd(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// FRD.md — AESD01 FR-ID format, FR protocol parity, promised API methods;
/// AESD02 section order; AESD03 spec purity; AESD04 crosslinks
/// (PRD, BACKLOG, ROADMAP, ARCHITECTURE); AESD05 heading; AESD06 entry
/// tables under `Functional Requirements` plus template body.
pub trait IFrdDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to FRD.md in *context*.
    fn audit_frd(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// AGENTS.md — AESD04 crosslinks (README, CONTRIBUTING, ARCHITECTURE,
/// ROADMAP, PRD); AESD05 heading plus verbatim line drift; AESD06 entry
/// tables (`Runtime`) plus template body.
pub trait IAgentsDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to AGENTS.md in *context*.
    fn audit_agents(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// DATA.md — AESD01 bare-ID format; AESD02 section order; AESD03 spec
/// purity; AESD04 crosslinks (PRD, BACKLOG, ROADMAP, ARCHITECTURE); AESD05
/// heading (DT-prefixed H2s allowed); AESD06 entry tables under `Data
/// Requirements` plus template body.
pub trait IDataDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to DATA.md in *context*.
    fn audit_data(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// ROADMAP.md — AESD01 PR-ID parity with PRD; AESD03 spec purity; AESD04
/// crosslinks (PRD, ARCHITECTURE); AESD05 heading; AESD06 entry tables
/// under `Roadmap Status` plus template body.
pub trait IRoadmapDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to ROADMAP.md in *context*.
    fn audit_roadmap(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// README.md — AESD04 crosslinks (none; placeholder table); AESD05 heading;
/// AESD06 entry tables (`Quick Start`, `Reference`) plus template body.
pub trait IReadmeDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to README.md in *context*.
    fn audit_readme(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// DESIGN.md — AESD01 DS-ID alignment with BACKLOG; AESD04 crosslinks
/// (PRD, BACKLOG, ROADMAP, ARCHITECTURE); AESD05 heading; AESD06 entry
/// tables under `Design Requirements` plus `no_extra_table` / `H3 unexpected`
/// sweeps plus template body.
pub trait IDesignDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to DESIGN.md in *context*.
    fn audit_design(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// ARCHITECTURE.md — AESD05 heading; AESD06 template body.
pub trait IArchitectureDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to ARCHITECTURE.md in
    /// *context*.
    fn audit_architecture(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// CONTRIBUTING.md — AESD04 crosslinks (PRD, ARCHITECTURE); AESD05 heading;
/// AESD06 entry tables (`Workflow`, `Commit Convention`, `Reference`) plus
/// template body.
pub trait IContributingDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to CONTRIBUTING.md in
    /// *context*.
    fn audit_contributing(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}

/// BACKLOG.md — AESD01 BACKLOG ID-level alignment against its paired spec
/// union (FRD/DATA/DESIGN per role, or root PRD/ROADMAP); AESD04 role-aware
/// crosslinks; AESD05 heading; AESD06 entry tables under `Backlog Status`
/// plus template body.
pub trait IBacklogDocProtocol: Send + Sync {
    /// Report every invariant violation scoped to BACKLOG.md in *context*.
    fn audit_backlog(&self, context: &DocAuditContext) -> Vec<DocFinding>;
}
