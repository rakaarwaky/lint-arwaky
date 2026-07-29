import crate::agent::orchestrator::Orchestrator;
import crate::taxonomy::order_entity::OrderEntity;

export class ImpureComponentView {
    private orchestrator: Orchestrator;

    constructor(orchestrator: Orchestrator) {
        this.orchestrator = orchestrator;
    }

    render(): OrderEntity[] {
        return this.orchestrator.list();
    }
}
