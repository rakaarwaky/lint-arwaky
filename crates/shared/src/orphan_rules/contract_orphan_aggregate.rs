// PURPOSE: IOrphanAggregate — aggregate trait for orphan detection (AES308)
use crate::orphan_rules::taxonomy_orphan_request::OrphanRequest;
use crate::orphan_rules::taxonomy_orphan_response::OrphanResponse;

pub trait IOrphanAggregate: Send + Sync {
    fn execute(&self, request: OrphanRequest) -> OrphanResponse;
}
