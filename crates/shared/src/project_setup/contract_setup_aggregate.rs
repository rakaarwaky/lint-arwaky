// PURPOSE: SetupAggregate — aggregate trait for project setup orchestration
use crate::contract_setup_protocol::IAdapterInstallationProtocol;
use crate::taxonomy_project_setup_request::SetupRequest;
use crate::taxonomy_project_setup_response::SetupResponse;

pub type SetupMgmtProtocol = Box<dyn IAdapterInstallationProtocol>;

pub trait ISetupAggregate: Send + Sync {
    fn execute(&self, request: SetupRequest) -> SetupResponse;
}
