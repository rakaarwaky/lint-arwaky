// AES013 — forbidden inheritance test
// This contract aggregate file inherits from contract port (forbidden)
import { IDeadProtocol } from "./dead_protocol_aggregate";

export class ForbiddenSourceAggregate implements IDeadProtocol {
    aggregate(): string {
        return 'forbidden';
    }
}
