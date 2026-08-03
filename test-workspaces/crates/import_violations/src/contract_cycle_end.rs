// AES205 side B — contract importing capabilities (creates cycle with cycle_start).
// contract(aggregate) importing capabilities is forbidden → triggers AES201 + AES205.
use capabilities::handler::CapabilitiesHandler;
use taxonomy::entity::ProductEntity;

pub fn process() -> ProductEntity {
    let handler = CapabilitiesHandler::new();
    handler.process();
    ProductEntity::new()
}
