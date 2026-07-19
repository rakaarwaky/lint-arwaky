import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';
import { I<Name>Protocol } from '../shared/<domain>/contract_<name>_protocol';

// ─── Block 1: Class Definition & Constructor ──────────────
export class Capabilities<Name> implements I<Name>Protocol {
    constructor(/* DI params */) {
        // DI fields use protocol interfaces
        // Value fields use shared VOs
    }

    // ─── Block 2: Public Contract (domain protocol ONLY) ──
    methodName(param: <VO>): void {
        // domain behavior
    }

    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'Capabilities<Name>()';
    }

    static create(): Capabilities<Name> {
        return new Capabilities<Name>();
    }
}
