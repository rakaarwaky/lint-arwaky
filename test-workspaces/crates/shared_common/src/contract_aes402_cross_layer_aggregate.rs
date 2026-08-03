// AES402: contract importing capabilities (cross-layer violation)
use capabilities::handler::CapabilitiesHandler;

pub struct ConfigAggregate;

impl ConfigAggregate {
    pub fn load(&self) {
        let handler = CapabilitiesHandler::new();
        handler.process();
    }
}
