// Fixture: AES205 — circular dependency B side (contract importing capabilities).
use capabilities::handler::CapabilitiesHandler;

pub fn process() {
    let _handler = CapabilitiesHandler::new();
}
