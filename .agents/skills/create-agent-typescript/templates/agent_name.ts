import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';
import { I<Name>Aggregate } from '../shared/<domain>/contract_<name>_aggregate';

// ─── Block 1: Class Definition & Constructor ──────────────
export class Agent<Name> implements I<Name>Aggregate {
    constructor(/* DI params */) {
        // DI fields use port interfaces
        // Value fields use shared VOs
    }

    // ─── Block 2: Public Contract (domain aggregate ONLY) ──
    execute(request: ScanRequest): LintResult[] {
        // orchestration only
        return [];
    }

    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'Agent<Name>()';
    }

    static create(): Agent<Name> {
        return new Agent<Name>();
    }
}
