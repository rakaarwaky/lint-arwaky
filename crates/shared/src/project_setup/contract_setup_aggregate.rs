// PURPOSE: SetupAggregate — aggregate trait for project setup orchestration
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::project_setup::taxonomy_setup_request_vo::{SetupRequest, SetupResponse};

pub type SetupMgmtProtocol = Box<dyn ISetupManagementProtocol>;

pub trait ISetupAggregate: Send + Sync {
    fn execute(&self, request: SetupRequest) -> SetupResponse;
}
