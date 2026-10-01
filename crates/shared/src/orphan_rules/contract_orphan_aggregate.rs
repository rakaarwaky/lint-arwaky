// PURPOSE: IOrphanAggregate — aggregate trait for orphan detection (AES308)
use crate::taxonomy_orphan_rules_request::OrphanRequest;
use crate::taxonomy_orphan_rules_response::OrphanResponse;

pub trait IOrphanAggregate: Send + Sync {
    fn execute(&self, request: OrphanRequest) -> OrphanResponse;
}
