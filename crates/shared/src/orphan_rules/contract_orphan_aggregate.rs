// PURPOSE: IOrphanAggregate — aggregate trait for orphan detection (AES308)
use crate::orphan_rules::taxonomy_orphan_request_vo::{OrphanRequest, OrphanResponse};

pub trait IOrphanAggregate: Send + Sync {
    fn execute(&self, request: OrphanRequest) -> OrphanResponse;
}
