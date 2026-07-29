use crate::taxonomy::order_entity::OrderEntity;
use crate::contract::fake_protocol::IFakeProtocol;

pub use ProcessorClass;

class ProcessorClass {
    process(order: OrderEntity): boolean {
        return true;
    }
}
