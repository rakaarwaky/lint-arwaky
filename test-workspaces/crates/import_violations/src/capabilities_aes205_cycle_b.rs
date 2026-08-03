// Fixture: AES205 — circular dependency B side (contract importing capabilities).
use capabilities::processor::CapabilitiesProcessor;

pub fn process() {
    let _proc = CapabilitiesProcessor::new();
}
