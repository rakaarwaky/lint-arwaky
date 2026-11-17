// AES201: surface importing contract(protocol) (forbidden layer)
use crate::contract::calculator_protocol::ICalculatorProtocol;

pub struct ForbiddenImportContractCommand;
impl ForbiddenImportContractCommand {
    pub fn execute(&self) -> &'static str { "forbidden" }
}
