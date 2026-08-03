use crate::capabilities::auth_checker::AuthChecker;
use crate::taxonomy::order_entity::OrderEntity;

export class DirectInfraHandler {
    private checker: AuthChecker;

    constructor(checker: AuthChecker) {
        this.checker = checker;
    }

    handle(request: any): boolean {
        return this.checker.authenticate(request.token);
    }
}
