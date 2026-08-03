use crate::taxonomy::order_entity::OrderEntity;
use crate::contract::service_container_aggregate::ServiceContainerAggregate;

export class StatefulTaskOrchestrator {
    private container: ServiceContainerAggregate;
    private state: Map<string, any>;
    private counter: number;

    constructor(container: ServiceContainerAggregate) {
        this.container = container;
        this.state = new Map();
        this.counter = 0;
    }

    execute(task: string): void {
        this.counter++;
        this.state.set(task, { status: "running" });
        this.processTask(task);
    }

    private processTask(task: string): void {
        this.state.set(task, { status: "completed" });
    }
}
