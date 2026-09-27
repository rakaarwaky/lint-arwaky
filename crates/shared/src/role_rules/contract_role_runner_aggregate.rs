// PURPOSE: IRoleRunnerAggregate — single entry point over the role-rules domain
// The agent behind the aggregate dispatches each RoleRequest to the rich
// role-protocol operations. Consumers never see the protocols.
use crate::role_rules::taxonomy_role_request::RoleRequest;
use crate::role_rules::taxonomy_role_response::RoleResponse;

/// Single entry point over role-rules; the agent dispatches internally.
pub trait IRoleRunnerAggregate: Send + Sync {
    fn execute(&self, request: RoleRequest) -> RoleResponse;
}
