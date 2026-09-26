// PURPOSE: SetupAggregate — aggregate trait for project setup orchestration
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::project_setup::taxonomy_setup_request::SetupRequest;
use crate::project_setup::taxonomy_setup_response::SetupResponse;

pub type SetupMgmtProtocol = Box<dyn ISetupManagementProtocol>;

pub trait ISetupAggregate: Send + Sync {
    fn execute(&self, request: SetupRequest) -> SetupResponse;
}
