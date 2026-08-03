use crate::taxonomy::order_entity::OrderEntity;
use crate::contract::fake_protocol::IFakeProtocol;

export function processOrder(order: OrderEntity): boolean {
    return order.isValid();
}

export function validateAmount(amount: number): boolean {
    return amount > 0;
}
