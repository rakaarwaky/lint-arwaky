// Fixture: AES205 — circular dependency B side (contract importing capabilities).
use capabilities::handler::CapabilitiesHandler;
use taxonomy::entity::ProductEntity;

pub fn process() -> ProductEntity {
    let _handler = CapabilitiesHandler::new();
    ProductEntity::new()
}
