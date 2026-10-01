// PURPOSE: IFixAggregate — single entry point over the auto-fix domain
// FRD API Contract alignment:
//   - `FixRequest::Execute { path, dry_run }` — per-request dry_run (FR-004 §9)
use crate::taxonomy_auto_fix_request::FixRequest;
use crate::taxonomy_auto_fix_response::FixResponse;

/// Single entry point over auto-fix; the agent dispatches internally.
pub trait IFixAggregate: Send + Sync {
    fn execute(&self, request: FixRequest) -> FixResponse;
}
