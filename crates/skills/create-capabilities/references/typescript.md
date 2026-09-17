# Capabilities — TypeScript

File: `capabilities_<domain>_<role>.ts`. Capabilities = concrete protocol interface implementation.

## Import rules

**Allowed imports:** Taxonomy, Contract (`_protocol` only), Utility.
**Forbidden:** `agent_*`, other `capabilities_*`, `surface_*`, local domain models, magic constants.

## Structure rules (TypeScript)

- Rule 1: Internal helper classes without `implements` → ALLOWED.
- Rule 2: ≥1 class implements a protocol interface.
- Rule 3: Total class + interface + enum ≤ 3 (not counting `type` aliases).

## 3-Block Structure

```text
// Block 1: Class Definition & Constructor
// Block 2: Protocol Method Implementation
// Block 3: Utility Methods, Factories, Helpers
```

Method placement: protocol interface methods → Block 2. `toString`/static factory/private → Block 3. Module-level function without class dep → extract to `*utility_.ts`.

## Helper vs Utility

Keep in Block 3 if ANY: uses `this`, domain-specific, single consumer, static factory.
Extract to utility only if ALL: no `this`, pure, no side effects, domain-agnostic, ≥2 consumers.

## Templates

### 3-block implementation

```typescript
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
```

### Protocol interface

```typescript
import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Name>Protocol {
    methodName(param: <VO>): void;
}
```

## Workflow

1. Confirm implements protocol behavior (not orchestration/data/mechanics).
2. File imports from `_protocol` module — if missing → flag `CapabilityNoProtocol`.
3. Create `contract_<name>_protocol.ts` if missing.
4. Enforce 3-Block.
5. AES403: ≥1 interface implementor, ≤3 types, DI via protocols, shared VOs.
6. No forbidden imports, no inter-capability deps, no local domain models.
7. `npx tsc --noEmit`.

## Checklist

- [ ]  Block 1 → 2 → 3 order followed.
- [ ]  Block 2: ONLY protocol interface method implementations.
- [ ]  ≥1 class implements protocol interface; ≤3 total types.
- [ ]  Imports from `_protocol` module only.
- [ ]  No local domain models, no agent/capability imports.
- [ ]  DI via protocol interfaces; shared VOs for fields and signatures.
- [ ]  Constants → `taxonomy_<domain>_constant.ts`.
- [ ]  Low-level ops → Utility.
- [ ]  `npx tsc --noEmit` passes.
