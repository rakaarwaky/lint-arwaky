// AES402: contract importing capabilities (cross-layer violation)
import { CapabilitiesHandler } from '../capabilities/handler';

export class ConfigAggregate {
    load(): void {
        const handler = new CapabilitiesHandler();
        handler.process();
    }
}
