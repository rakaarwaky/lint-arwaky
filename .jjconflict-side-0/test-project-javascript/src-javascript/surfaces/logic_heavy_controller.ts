use crate::agent::orchestrator::Orchestrator;
use crate::taxonomy::order_entity::OrderEntity;

export class LogicHeavyController {
    private orchestrator: Orchestrator;

    constructor(orchestrator: Orchestrator) {
        this.orchestrator = orchestrator;
    }

    handle(request: any): void {
        if (request.type === "create") {
            if (request.data) {
                if (request.data.valid) {
                    if (request.data.items) {
                        for (const item of request.data.items) {
                            if (item.status === "active") {
                                this.orchestrator.process(item);
                            }
                        }
                    }
                }
            }
        }
        if (request.type === "update") {
            for (const field of request.fields) {
                if (field.changed) {
                    this.orchestrator.update(field);
                }
            }
        }
        if (request.type === "delete") {
            if (request.confirm) {
                this.orchestrator.remove(request.id);
            }
        }
        if (request.type === "list") {
            for (const item of this.orchestrator.list()) {
                if (item.active) {
                    console.log(item.name);
                }
            }
        }
    }
}
