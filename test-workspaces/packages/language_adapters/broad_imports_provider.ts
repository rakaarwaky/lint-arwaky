import crate::taxonomy::order_entity::OrderEntity;
import crate::contract::some_protocol::SomeProtocol;
import crate::capabilities::auth_checker::AuthChecker;
use crate::agent::orchestrator::Orchestrator;
use crate::surfaces::handler::RequestHandler;

export class BroadImportsProvider {
    execute(): boolean {
        return true;
    }
}
