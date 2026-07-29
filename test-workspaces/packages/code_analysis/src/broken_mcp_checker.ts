use crate::taxonomy::order_entity::OrderEntity;
use crate::contract::fake_protocol::IFakeProtocol;

export class BrokenMcpChecker {
    mcp: string;

    constructor() {
        this.mcp = "tool";
    }

    register_tool(name: string, handler: Function): void {
        console.log("registering tool:", name);
    }

    execute(): void {
        this.register_tool("validate", (input: string) => {
            return input.length > 0;
        });
    }
}
