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
> **Length**: `_protocol` = one method per feature; `_aggregate` = every method the
> surface/root exports. Only methods outer layers actually call.

---

## Rules

Seven rules. Each one prevents a specific failure mode.

1. **Suffix is strictly `_protocol` or `_aggregate`.** Type names: `I<Name>Protocol`,
   `I<Name>Aggregate`. File: `contract_<concept>_<suffix>.ts`.
2. **`export interface` only — no class implementations.** Never private-helper
   signatures, never convenience API (`AES101`/`AES102`).
3. **Protocol file = exactly ONE interface + ONE method. Never more.** Each
   `contract_<concept>_protocol.ts` declares a single `export interface I<Name>Protocol`
   with a single method for one feature. No second method, no helper methods, no second
   interface/type in the same file — not even a small leaf. A second feature is a second
   protocol *file* (`contract_<other>_protocol.ts`), never a second interface or method
   inside this one. Same shape for every capability in the domain. Only the `_aggregate`
   may hold many methods.
4. **Aggregate = many methods, one per exported consumer operation.** The aggregate is
   the **export surface**: every action the CLI/surface/root may call appears as its
   own method. Rich, typed, one row per export — not a single dump-all `execute()`.
5. **Allowed imports: taxonomy types and other contract types only.** Capabilities,
   agents, surface, root invert the dependency arrow (`AES201`/`AES205`).
6. **Signatures use shared VOs** — no `string`/`number`/`string[]`/`Record<string,T>`
   for domain values. `boolean` allowed for semantic toggles only. All methods fully
   type-annotated.
7. **Register in shared `index.ts`** so the pair is importable.

---

## Workflow

1. **Determine suffix** — `_protocol` (inward, one method) or `_aggregate` (outward, many methods).
2. **Create file** → `contract_<concept>_<suffix>.py`.
3. **Draft protocol** — One class, one `@abstractmethod`, body is `...`.
4. **Draft aggregate** — One class, one method per export, body is `...`.
5. **Register** in shared `__init__.py` with `__all__` + `_layer_symbols`.
6. **Verify** → `lint-arwaky-cli scan <contract-dir>`.

## Template

Copy, fill, delete nothing.

### Protocol interface — one method for one feature (uniform across capabilities)

```typescript
import { <VO>, <ResultVO> } from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Name>Protocol {
    /** Capability contract for one feature: <feature name> — <one sentence>. */
    <featureMethod>(param: <VO>): <ResultVO>;
}
```

**One file → one interface → one method.** Violations: a second method on the
interface, a second `export interface`/type in the same file (including adapter/leaf
interfaces), or an `execute(op, …)` that dispatches multiple features behind one name.
Each of those is a *new protocol file* (or a second capability implementing the same
one-method shape).

### Aggregate interface — many methods, one per export

```typescript
import {
    ExitCode,
    <ArgsVO>,
    <QueryVO>,
    <ResultVO>,
} from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Name>Aggregate {
    /** Export surface over <domain>: one method per consumer operation. */

    /** Export 1: <what the surface runs>. */
    <export1>(args: <ArgsVO>): ExitCode;

    /** Export 2: <what the surface runs>. */
    <export2>(query: <QueryVO>): <ResultVO>;

    /** Export n: add one method per new surface verb. */
    <exportN>(/* … */): ExitCode;
}
```

**The aggregate is what gets exported to consumers.** Every public operation the
surface/root needs is its own method; the agent implements them by dispatching to
one-method protocol capabilities.

---

## Section Contract

Every contract file is required to carry the rows that apply. Each exists for one reason.

| Section                        | Why it belongs here                                                       |
| ------------------------------ | ------------------------------------------------------------------------ |
| Module docstring (required)    | Names the contract's role: capability interface or export/aggregate.     |
| Suffix in file + type name     | AES101/AES102 resolve `_protocol` vs `_aggregate` from the name.         |
| Protocol: 1 file = 1 interface = 1 method | Fan-out stays uniform; extra interface/method → new protocol file, never this one. |
| Aggregate: one method / export | Surface/root exports stay typed and discoverable; no dump-all entry.      |
| Signature-only interface       | Outer layers depend on promises, not behaviour.                          |
| Shared VOs in signatures       | Domain values stay opaque across layers; no primitive leakage.           |
| No impl-layer imports          | Keeps the dependency arrow (capabilities → contract ← agent).            |
| Register in shared `index.ts`  | Importable without reaching into private modules.                        |

---

## Verify

```bash
lint-arwaky-cli scan <contract-dir>
# Checks: AES101/AES102 (filename contract_<concept>_{protocol,aggregate}),
# AES201–AES205 (layer imports: no impl-layer imports; protocol ≠ aggregate import),
# AES402 (no primitives in signatures), role rules (contract ↔ capabilities/agent/surface).
# Manual (not machine-checked): protocol file = exactly 1 interface + 1 method
# (no second interface, no second method, no multi-feature execute dispatch);
# aggregate = one method per consumer export (many exports, not a single execute());
# interface signature-only (no class implementation).
# Fallback compile gate: npx tsc --noEmit.
```
