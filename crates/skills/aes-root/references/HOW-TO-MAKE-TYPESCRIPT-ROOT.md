# HOW TO MAKE ROOT TYPESCRIPT

> **Purpose**: Compose the system: containers wire capabilities to contracts; entries bootstrap and start the process.
>
> **Audience**: Agents and engineers scaffolding AES composition roots.
>
> **Scope**: Python, Rust, and TypeScript `root_<concept>_<container|entry>` files (plus documented barrel/entry exceptions).
>
> **Location**: Feature or app package top; the only layer that may construct implementations.
>
> **Length**: One container per feature; one (or few) entry files; no business logic, no orchestration policy.

---

## Rules

1. **Suffix is strictly `_container` or `_entry`.** File: `root_<concept>_<suffix>.ts`.
   Documented exceptions: `index.ts`, `index.js`, barrel/entry files (AES101/AES102).
2. **Container = wire one feature only.** Instantiate capabilities; expose the aggregate
   (contract) type — never a concrete capability. Never business logic.
3. **Entry = bootstrap the app.** Read `process.argv` (if needed), compose feature
   container factories, start the surface/CLI loop. Never construct capabilities directly.
4. **Allowed imports: everything below root** — agent, capabilities, surface, contract,
   taxonomy, utility. Nothing below root may import root (AES201–AES205).
5. **No business logic, no orchestration policy, no technical parsing, no UI behaviour** —
   those live in capabilities / agent / utility / surface.
6. **Signatures use shared VOs** where domain values appear (AES402). `boolean` for
   semantic toggles only.
7. **Register** in package `index.ts`.

### Workflow

1. **Determine role** — Container (wire one feature) or Entry (bootstrap all)?
2. **Create file** → `root_<concept>_<suffix>.ts`.
3. **Wire deps** → instantiate capabilities; bind to contract interfaces/aggregates.
4. **Register** → update `index.ts`.
5. **Verify** → `lint-arwaky-cli scan <layer-path>` then `npx tsc --noEmit`.

---

## Template

Copy, fill, delete nothing. File: `root_<concept>_<suffix>.ts` where
`<suffix>` is `_container` or `_entry`.

### Container — wire one feature

```typescript
// PURPOSE: <Concept>Container — wiring for <feature> feature (root layer, wiring only)
import { <Concept>Orchestrator } from "./agent_<concept>_orchestrator";
import { <Capability> } from "./capabilities_<concept>_<role>";
import type { <Concept>Aggregate } from "../shared/src/contract_<concept>_aggregate";
import type { <Concept>Protocol } from "../shared/src/contract_<concept>_protocol";

// ─── Block 1: Class Definition ────────────────────────────

export class <Concept>Container {
  private readonly orchestrator: <Concept>Orchestrator;

  constructor(/* shared deps: Deps */) {
    const runners: <Concept>Protocol[] = [new <Capability>(/* deps */)];
    this.orchestrator = new <Concept>Orchestrator(runners);
  }

  // ─── Block 2: Wiring & Factory ──────────────────────────

  get aggregate(): <Concept>Aggregate {
    return this.orchestrator;
  }
}

export function create<Concept>Feature(/* deps */): <Concept>Aggregate {
  return new <Concept>Container(/* deps */).aggregate;
}
```

**Container rules:** import capabilities + agent + contract types only;
expose the aggregate (contract), never a concrete capability; construction and
binding only — no business logic.

### Entry — bootstrap the application

```typescript
// PURPOSE: <concept> CLI entry — compose feature containers and start the surface
import { create<Concept>Feature } from "./root_<concept>_container";
import { run } from "./surface_<concept>_command";

const argv = process.argv.slice(2);
const aggregate = create<Concept>Feature();
process.exit(run(aggregate, argv));
```

**Entry rules:** read `process.argv` (bootstrap only) and compose container
factories; start the surface/CLI loop; never construct capabilities directly;
no business logic.

---

## Section Contract

| Section | Why it belongs here |
| ------- | ------------------- |
| Module comment / PURPOSE | Names the role: composition container or app entry. |
| Correct filename + suffix | AES101/AES102 resolve the layer and role from the name. |
| Layer-legal imports | Keeps the dependency arrow pointed down (AES201–AES205). |
| Contract types in signatures | Expose aggregate/protocol, not concretions (AES402). |
| Container: construct + bind only | Wiring is the whole job; logic belongs below. |
| Entry: compose containers + start | Bootstrap only; never construct capabilities. |
| Registered in `index.ts` | Dead code otherwise; composition needs the export. |

---

## Anti-Patterns

- **Business / orchestration / parsing / UI logic in root** — move down a layer.
- **Entry importing `capabilities_*` directly** — go through a container factory.
- **Container exposing a concrete capability type** — return the aggregate contract.
- **Wrong suffix or undocumented name** — only `_container` / `_entry` (or listed exceptions).

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): correct role suffix; no business/orchestration/parsing/UI logic; nothing below root imports root.
# Fallback compile gate: npx tsc --noEmit
```
