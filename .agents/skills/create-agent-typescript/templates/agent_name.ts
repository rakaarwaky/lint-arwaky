import { <RequestVO> } from '../shared/<domain>/taxonomy_<request>_vo';
import { <ResultVO> } from '../shared/<domain>/taxonomy_<result>_vo';
import { I<Name>Aggregate } from '../shared/<domain>/contract_<name>_aggregate';
import { I<Protocol>Protocol } from '../shared/<domain>/contract_<protocol>_protocol';

// ─── Block 1: Class Definition & Constructor ──────────────
export class <Name>Orchestrator implements I<Name>Aggregate {
    constructor(private readonly deps: <Name>Deps) {
        // DI fields use protocol interfaces
        // Value fields use shared VOs
    }

    // ─── Block 2: Public Contract (domain aggregate ONLY) ──
    execute(request: <RequestVO>): <ResultVO> {
        // orchestration only - delegate to protocol
        const formatter: I<Protocol>Protocol = this.getFormatter(request);
        return formatter.process(request);
    }

    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return '<Name>Orchestrator()';
    }

    private getFormatter(request: <RequestVO>): I<Protocol>Protocol {
        switch (request.type) {
            case RequestType.A: return this.deps.a;
            case RequestType.B: return this.deps.b;
        }
    }

    static create(): <Name>Orchestrator {
        return new <Name>Orchestrator({ a: new Default<Protocol>(), b: new Default<Protocol>() });
    }
}
