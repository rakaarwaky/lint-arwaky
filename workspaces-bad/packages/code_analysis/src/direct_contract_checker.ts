use crate::contract::dummy_port::IDummyProtocol;
use crate::taxonomy::order_entity::OrderEntity;

export class DirectContractChecker {
    private protocol: IDummyProtocol;

    constructor(protocol: IDummyProtocol) {
        this.protocol = protocol;
    }

    check(order: OrderEntity): boolean {
        return this.protocol.validate(order);
    }
}
