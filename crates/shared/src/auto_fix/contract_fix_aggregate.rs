// PURPOSE: IFixAggregate — single entry point over the auto-fix domain
// FRD API Contract alignment:
//   - `FixRequest::Execute { path, dry_run }` — per-request dry_run (FR-004 §9)
//   - `FixRequest::ManualReport` — FR-005: non-fixable violation reporting
use crate::auto_fix::taxonomy_fix_request::FixRequest;
use crate::auto_fix::taxonomy_fix_response::FixResponse;

/// Single entry point over auto-fix; the agent dispatches internally.
pub trait IFixAggregate: Send + Sync {
    fn execute(&self, request: FixRequest) -> FixResponse;
}
