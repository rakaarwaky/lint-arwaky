use crate::contract::dummy_port::IDummyPort;
use crate::taxonomy::order_entity::OrderEntity;

export class DirectContractChecker {
    private port: IDummyPort;

    constructor(port: IDummyPort) {
        this.port = port;
    }

    check(order: OrderEntity): boolean {
        return this.port.validate(order);
    }
}
