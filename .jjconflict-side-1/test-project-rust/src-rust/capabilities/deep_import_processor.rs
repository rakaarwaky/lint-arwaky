// AES007: import with 5+ segments
use crate::contract::sub::module::WrongNamePort;

pub struct DeepImportProcessor;

impl WrongNamePort for DeepImportProcessor {
    fn execute(&self, _input: String) -> bool {
        true
    }
}
