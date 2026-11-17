// AES201: surface importing capabilities (forbidden layer)
use crate::capabilities::analyzer::SomeAnalyzer;

pub struct ForbiddenImportCapabilitiesCommand;
impl ForbiddenImportCapabilitiesCommand {
    pub fn execute(&self) -> &'static str { "forbidden" }
}
