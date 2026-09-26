# HOW TO MAKE CONTRACT TYPESCRIPT

> **Purpose**: State a TypeScript contract's public promises (protocol or aggregate) so  
> outer layers can depend on the seam without importing a concretion.
>
> **Audience**: Agents and engineers scaffolding AES contract interfaces in the shared  
> domain.
>
> **Scope**: Pure interface definitions in `contract_<concept>_<suffix>.ts` — `_protocol`  
> or `_aggregate` only. No class implementations.
>
> **Location**: Shared domain package next to taxonomy, registered in the shared  
> `index.ts`.
>
> **Length**: `_protocol` = every method the capabilities expose; `_aggregate` = one entry  
> point. Only methods outer layers actually call.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Suffix is strictly** `_protocol` **or** `_aggregate`**.** Type names: `I<Name>Protocol`,  
 `I<Name>Aggregate`. File: `contract_<concept>_<suffix>.ts`.
2. `export interface` **only — no class implementations.** Never private-helper  
 signatures, never convenience API (`AES101`/`AES102`).
3. **Protocol = rich, one method per capability operation.** The interface declares every  
 operation its capabilities implement, each with its own named method and its own typed  
 signature. A capability implements the whole interface. No `execute(op, …)` dispatch,  
 no `any` argument bag, no union return spanning differing result shapes — every method  
 has one concrete return type.
4. **Aggregate = ONE method.** The aggregate is the single entry point the  
 surface/root/CLI/MCP calls. All consumer verbs live in the agent, which dispatches to  
 the rich protocol. A dump-all `execute(op, …)` aggregate is the violation here, not a  
 rich one.
5. **Allowed imports: taxonomy types and other contract types only.** Capabilities,  
 agents, surface, root invert the dependency arrow (`AES201`/`AES205`).
6. **Signatures use shared VOs** — no `string`/`number`/`string[]`/`Record<string,T>`  
 for domain values. `boolean` allowed for semantic toggles only. No union return  
 spanning several value shapes; when operations genuinely differ in return type, they  
 are separate methods. All methods fully type-annotated.
7. **Register in shared** `index.ts` so the pair is importable.

---

## Workflow

1. **Determine suffix** — `_protocol` (inward, rich) or `_aggregate` (outward, one method).
2. **Create file** → `contract_<concept>_<suffix>.ts`.
3. **Draft protocol** — one interface, one method per capability operation, each with a  
 concrete return type.
4. **Draft aggregate** — one interface, one method.
5. **Register** in shared `index.ts`.
6. **Verify** → `lint-arwaky-cli scan <contract-dir>`.

## Template

Copy, fill, delete nothing.

### Protocol interface — rich, one named method per operation

```typescript
import { VO, ResultVO } from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Name>Protocol {
    /** Capability contract for <domain>: every operation the capabilities expose. */

    /** <What this operation does.> */
    <operation1>(param: VO): ResultVO;

    /** <What this operation does.> */
    <operation2>(page: PageVO, timeout: TimeoutVO): TextVO;

    /** Add one method per new capability operation. */
    <operationN>(/* … */): ResultVO;
}
```

**One file → one interface → many named methods.** Each method is a real, typed operation.  
Violations: an `execute(op, …)` that dispatches several features behind one name, an  
untyped argument bag, or a union return that papers over differing result shapes. A  
capability implements the whole interface, so it never carries stubs for operations it  
does not own — if it does not own them, it does not implement this interface.

### Aggregate interface — one method

```typescript
import { RequestVO, ResponseVO } from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Name>Aggregate {
    /** Single entry point over <domain>; the agent dispatches internally. */
    execute(request: RequestVO): ResponseVO;
}
```

**The aggregate is the one door consumers knock on.** Consumers never see the protocol's  
method list; the agent behind the aggregate routes the request to the right capability  
operation. Adding a consumer verb = a new property on `RequestVO` + the agent's  
dispatch branch, not a new aggregate method.

---

## Section Contract

Every contract file is required to carry the rows that apply. Each exists for one reason.


| Section                            | Why it belongs here                                                            |
| ---------------------------------- | ------------------------------------------------------------------------------ |
| Module docstring (required)        | Names the contract's role: capability interface or entry-point interface.      |
| Suffix in file + type name         | AES101/AES102 resolve `_protocol` vs `_aggregate` from the name.               |
| Protocol: rich, named methods      | Each operation stays typed and discoverable; no dispatch bag, no union return. |
| Protocol: one concrete return type | Callers know the result shape without narrowing a union.                       |
| Aggregate: exactly one method      | Consumers depend on one stable entry point, not a shifting method list.        |
| Signature-only interface           | Outer layers depend on promises, not behaviour.                                |
| Shared VOs in signatures           | Domain values stay opaque across layers; no primitive leakage.                 |
| No impl-layer imports              | Keeps the dependency arrow (capabilities → contract ← agent).                  |
| Register in shared `index.ts`      | Importable without reaching into private modules.                              |


---

## Verify

```bash
lint-arwaky-cli scan <contract-dir>
# Checks: AES101/AES102 (filename contract_<concept>_{protocol,aggregate}),
# AES201–AES205 (layer imports: no impl-layer imports; protocol ≠ aggregate import),
# AES402 (no primitives in signatures), role rules (contract ↔ capabilities/agent/surface).
# Manual (not machine-checked): protocol methods are all named and individually typed
# (no `execute(op, …)`, no untyped argument bag, no union return spanning differing result
# shapes); aggregate declares exactly one method; interface signature-only
# (no class implementation).
# Fallback compile gate: npx tsc --noEmit.
```

