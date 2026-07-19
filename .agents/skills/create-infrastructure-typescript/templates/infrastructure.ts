import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';
import { I<Name>Port } from '../shared/<domain>/contract_<name>_port';

// ─── Block 1: Class Definition & Constructor ──────────────
export class Infrastructure<Name> implements I<Name>Port {
    constructor(/* DI params */) {
        // DI fields use port interfaces
        // Value fields use shared VOs
    }

    // ─── Block 2: Public Contract (domain port ONLY) ──────
    methodName(param: <VO>): Result<<VO>, Error> {
        // port implementation
    }

    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'Infrastructure<Name>()';
    }

    static create(): Infrastructure<Name> {
        return new Infrastructure<Name>();
    }
}
