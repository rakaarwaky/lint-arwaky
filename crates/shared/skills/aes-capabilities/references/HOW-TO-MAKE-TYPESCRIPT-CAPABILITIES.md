# HOW TO MAKE CAPABILITIES TYPESCRIPT

> **Purpose**: Implement concrete protocol behaviour: domain rules and external adaptation as protocol implementations.
>
> **Audience**: Agents and engineers scaffolding AES capability files.
>
> **Scope**: Python, Rust, and TypeScript `capabilities_<domain>_<role>` files — 3-block structure, ≥1 protocol implementor, ≤3 types.
>
> **Location**: Feature domain package; depends only on taxonomy, `_protocol` contracts, and utility.
>
> **Length**: At most 3 types per file; Block 1 → 2 → 3 order.

---

## Rules

### Capability vs Utility Boundary

A capability file **implements a protocol** — it has state (via DI), business rules, and concrete behaviour.
A utility file **performs a pure operation** — no state, no business rules, just a narrow task.

| Decision | → Capability | → Utility |
|---|---|---|
| Uses `this` / instance state / injected dependencies | Yes | No |
| Contains business rules or protocol logic | Yes | No |
| Stateless, domain-agnostic, ≥2 consumers | — | Yes |
| Implements `_protocol` interface | Yes | No |
| Pure exported function with no business logic | No | Yes |

**Rule**: If a method in Block 3 is a module-level function (`export function` without class), stateless, domain-agnostic, and reusable across modules — extract it to a `utility_*` file. It does not belong in the capability layer.

### Import rules

**Allowed imports:** Taxonomy, Contract (`_protocol` only), Utility.
**Forbidden:** `agent_*`, other `capabilities_*`, `surface_*`, local domain models, magic constants.

### Structure rules (TypeScript)

- Rule 1: Internal helper classes without `implements` → ALLOWED.
- Rule 2: ≥1 class implements a protocol interface.
- Rule 3: Total class + interface + enum ≤ 3 (not counting `type` aliases).

### 3-Block Structure

```text
// Block 1: Class Definition & Constructor
// Block 2: Protocol Method Implementation
// Block 3: Utility Methods, Factories, Helpers
```

Method placement: protocol interface methods → Block 2. `toString`/static factory/private → Block 3. Module-level function without class dep → extract to `*utility_.ts`.

### Helper vs Utility

Keep in Block 3 if ANY: uses `this`, domain-specific, single consumer, static factory.
Extract to utility only if ALL: no `this`, pure, no side effects, domain-agnostic, ≥2 consumers.

### Workflow

1. Confirm implements protocol behavior (not orchestration/data/mechanics).
2. File imports from `_protocol` module — if missing → flag `CapabilityNoProtocol`.
3. Create `contract_<name>_protocol.ts` if missing.
4. Enforce 3-Block with explicit `// ─── Block 1:`, `Block 2:`, `Block 3:` comments — AES403 `CapabilityBlockMarkers` reports a file whose banners are missing, out of order, or above 3.
5. AES403: ≥1 interface implementor, ≤3 types, DI via protocols, shared VOs.
6. No forbidden imports, no inter-capability deps, no local domain models.
7. `npx tsc --noEmit`.

---

## Template

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

    // ─── Block 2: Protocol Method Implementation ──────────────
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

---

## Section Contract

The `Enforced` column says which rule reports a violation, so you know what the
linter will catch and what stays a review responsibility. `Manual` means no rule
checks it — the shape is still the contract, but a violation is caught by review
or by `tsc`, not by `scan`.

| Check                                                             | Enforced                                                          |
| ----------------------------------------------------------------- | ----------------------------------------------------------------- |
| All three `// ─── Block 1:` / `Block 2:` / `Block 3:` banners present, in order, none above 3. | **AES403** `CapabilityBlockMarkers` (MEDIUM) |
| Block 1 (class) precedes Block 2 (protocol methods).              | **AES403** `CapabilityBlockOrder` (HIGH)                          |
| ≥1 class implements a protocol interface.                         | **AES403** `CapabilityNoImplementor` (MEDIUM)                     |
| Exactly 1 protocol interface per file.                            | **AES403** `CapabilityMultiProtocol` (MEDIUM)                     |
| ≤3 total classes / interfaces / type aliases.                     | **AES403** `CapabilityTooManyTypes` (HIGH)                        |
| Imports from `_protocol` module only.                             | **AES201**–**AES205** (import rules)                              |
| No agent / surface / root imports.                                | **AES201** `FORBIDDEN_IMPORT`                                     |
| Block 2: ONLY protocol interface method implementations.          | Manual — no rule reads Block 2's contents                        |
| DI via protocol interfaces; shared VOs for fields and signatures. | Manual                                                            |
| Constants → `taxonomy_<domain>_constant.ts`.                      | **AES403** `CapabilityLocalConstant` (MEDIUM)                     |
| Block 3 public helpers with no production caller flagged as defect. | **AES403** `CapabilityPublicHelper` (MEDIUM)                      |
| Test blocks (`describe`, `it`, `test`) live in `tests/`, never inline. | **AES403** `CapabilityEmbeddedTest` (LOW)                      |
| Low-level ops → Utility.                                          | Manual — the helper-vs-utility matrix is a judgement call         |
| `npx tsc --noEmit` passes.                                        | `tsc`, not `scan`                                                 |

**On the block banners.** A banner is `Block <digits>:` standing as its own word
inside a comment, so TypeScript uses the `//` sigil. The colon is load-bearing:
prose such as `Block 1 (types) -> Block 2` is not a marker, and neither is
`Sub-Block 4:`. Put the banner above the block it heads, at the same indent as
the code it introduces.

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# AES403 machine-enforced: type budget ≤3; protocol implementor present;
#   protocol method defined before the first private/static method;
#   module-level constants in taxonomy file; no inline describe/it/test block;
#   public methods with no production caller flagged.
# Manual (not machine-checked): helper-vs-utility matrix; role naming lists.
# Fallback compile gate: npx tsc --noEmit
```

