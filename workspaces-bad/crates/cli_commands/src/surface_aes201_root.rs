// AES201: surface importing root (forbidden layer)
use crate::root::calculator_entry::CalculatorEntry;

pub struct ForbiddenImportRootCommand;
impl ForbiddenImportRootCommand {
    pub fn execute(&self) -> &'static str { "forbidden" }
}
