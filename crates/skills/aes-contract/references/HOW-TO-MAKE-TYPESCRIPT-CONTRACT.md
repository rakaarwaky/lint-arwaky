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
> **Length**: One file per feature. A `_protocol` file may declare **one interface per**  
> **capability seam**; each interface is **rich** — one method per operation, each with a  
> concrete return type. An `_aggregate` file declares **exactly one method**.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Suffix is strictly** `_protocol` **or** `_aggregate`**.** Type names: `I<Name>Protocol`,  
 `I<Name>Aggregate`. File: `contract_<concept>_<suffix>.ts`.
2. `export interface` **only — no class implementations.** Never private-helper  
 signatures, never convenience API (`AES101`/`AES102`).
3. **One file per feature, one interface per capability seam, each interface rich.** A  
 `_protocol` file for a feature with several capabilities declares one `export  interface` per seam — `I<Injector>Protocol`, `I<Sender>Protocol`, `I<Stream>Protocol`  
 — side by side in the same file. Each interface carries **every** method its  
 capability implements, each with its own named signature and **one concrete return**  
 **type**. Never `execute(op, …)` dispatch, never an untyped argument bag, never a union  
 return spanning several value shapes.
4. **A capability implements exactly one interface, and all of it.** Because each  
 interface is scoped to one capability's own operations, an interface is never partially  
 implemented: it declares only the methods that capability owns, and that capability  
 defines every one of them. This is why a single file per feature is enough — no  
 per-method splitting, and no stubs.
5. **Aggregate = exactly one method.** The aggregate is the single entry point the  
 surface/root/CLI/MCP calls. All consumer verbs live in the agent, which dispatches to  
 the rich protocol interfaces. A dump-all `execute(op, …)` aggregate is the violation  
 here.
6. **Signatures use shared VOs.** Domain values must not be `string`, `number`,
   `string[]`, or `Record<string,T>` — wrap them in a taxonomy-defined VO before they
   appear in a signature. `boolean` is permitted only for semantic toggles or
   predicates (e.g. `enabled: boolean`), never for domain quantities. Enums are not
   banned; the ban targets primitive leakage. A multi-variant response type on the
   aggregate is correct and expected — each variant must hold VO-wrapped values, not
   raw primitives. Protocol methods keep one concrete VO return type; the aggregate
   response type is the sanctioned way to carry heterogeneous results across one
   `execute()` entry point. All methods fully type-annotated.
7. **Register in shared** `index.ts` so the pair is importable.

---

## Workflow

1. **Determine suffix** — `_protocol` (inward, rich interfaces) or `_aggregate` (outward,  
 one method).
2. **List the feature's capability seams** — group the feature's operations by the  
 capability that owns them. One group becomes one interface.
3. **Create file** → `contract_<concept>_<suffix>.ts`, one interface per seam.
4. **Draft each interface** — every method its capability implements, one concrete return  
 type each.
5. **Register** in shared `index.ts`.
6. **Verify** → `lint-arwaky-cli scan <contract-dir>`.

## Template

Copy, fill, delete nothing.

### Protocol file — one interface per capability seam, each rich

```typescript
/** <Feature>-domain capability contracts (AES102 `_protocol`).

One file for the <feature> feature. Each interface below is one capability
seam: an interface carries every method that capability implements, with one
concrete return type each, so a capability implements its interface outright
and never carries stubs.
*/

import { VO, ResultVO } from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Seam1>Protocol {
    /** <Seam 1> capability: <what it owns>. */

    /** <What this operation does.> */
    <operation1>(param: VO): ResultVO;

    /** <What this operation does.> */
    <operation2>(page: PageVO, timeout: TimeoutVO): TextVO;
}

export interface I<Seam2>Protocol {
    /** <Seam 2> capability: <what it owns>. */

    /** <What this operation does.> */
    <operation1>(path: PathVO, content: TextVO): PathVO;
}

export interface I<SeamN>Protocol {
    /** <Seam N> capability: <what it owns>. */

    /** <What this operation does.> */
    <operation1>(/* … */): ResultVO;
}
```

**One file → one feature → one interface per capability seam → many named methods per**  
**interface.**

Violations: an `execute(op, …)` that dispatches several features behind one name, an  
untyped argument bag, a union return that papers over differing result shapes, an  
interface that declares methods its implementors do not own, or an implementor that  
leaves a declared method unimplemented.

### Aggregate file — one method

```typescript
/** <Feature>-domain aggregate contract (AES101 `_aggregate`).

The single entry point over the <feature> feature. Consumers pass a request; the agent
behind the aggregate dispatches to the rich protocol interfaces in
`contract_<feature>_protocol.ts`.
*/

import { RequestVO, ResponseVO } from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Feature>Aggregate {
    /** Single entry point over the <feature> feature. */
    execute(request: RequestVO): ResponseVO;
}
```

**The aggregate is the one door consumers knock on.** Consumers never see the protocol  
interfaces' method lists; the agent behind the aggregate routes the request to the right  
capability. Adding a consumer verb = a new property on `RequestVO` + the agent's dispatch  
branch, not a new aggregate method.

---

## Section Contract

Every contract file is required to carry the rows that apply. Each exists for one reason.


| Section                                  | Why it belongs here                                                                     |
| ---------------------------------------- | --------------------------------------------------------------------------------------- |
| Module docstring (required)              | Names the feature and the seams in the file.                                            |
| Suffix in file + type names              | AES101/AES102 resolve `_protocol` vs `_aggregate` from the name.                        |
| One file per feature                     | All seams of a feature live together; consumers import from one module.                 |
| One interface per capability seam        | An interface lists only what its capability owns, so implementation is always complete. |
| Rich named methods, one return type each | Protocol interfaces: each method has one VO
   return; no dispatch bag. Aggregate: one `execute()` method whose response is a
   taxonomy-defined enum of VOs.          |
| Signature-only interface                 | Outer layers depend on promises, not behaviour.                                         |
| Aggregate: exactly one method            | Consumers depend on one stable entry point, not a shifting method list.                 |
| Shared VOs in signatures                 | Domain values stay opaque across layers; no primitive leakage.                          |
| No impl-layer imports                    | Keeps the dependency arrow (capabilities → contract ← agent).                           |
| Register in shared `index.ts`            | Importable without reaching into private modules.                                       |


---

## Verify

```bash
lint-arwaky-cli scan <contract-dir>
# Checks: AES101/AES102 (filename contract_<concept>_{protocol,aggregate}),
# AES201–AES205 (layer imports: no impl-layer imports; protocol ≠ aggregate import),
# AES402 (no primitives in signatures), role rules (contract ↔ capabilities/agent/surface).
# Manual (not machine-checked):
#   - every method in every protocol interface is named and individually typed
#     (no `execute(op, …)`, no untyped argument bag, no union return spanning differing
#     result shapes);
#   - each capability implements one interface from this file, and implements all of it
#     (a partial implementation produces a type error — compile each capability to
#     prove it);
#   - the aggregate declares exactly one `execute()` method; its response type is a
#     taxonomy-defined VO enum (each variant holds VO-wrapped values, not primitives).
#   - protocol interfaces are signature-only (no class implementation).
# Fallback compile gate: npx tsc --noEmit.
```

