// AES205 side B — contract importing capabilities (creates cycle).
// This triggers AES201 (forbidden) + AES205 (cycle).
use capabilities::handler::CapabilitiesHandler;
use taxonomy::entity::ProductEntity;

pub fn process() -> ProductEntity {
    let handler = CapabilitiesHandler::new();
    handler.process();
    ProductEntity::new()
}
