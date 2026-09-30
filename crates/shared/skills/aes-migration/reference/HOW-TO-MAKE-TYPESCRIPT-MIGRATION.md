# HOW TO MAKE MIGRATION TYPESCRIPT

> **Purpose**: Guide phased migration of legacy TypeScript/JavaScript projects into AES layered architecture — taxonomy → contract → utility → capabilities → agent → surface → root.
>
> **Audience**: Agents and engineers executing a migration to AES.
>
> **Scope**: Phase-based migration workflow for TypeScript projects; references layer skills for execution.
>
> **Location**: Project root; each phase operates on a layer directory.
>
> **Length**: 9 phases (0–8); total duration depends on violation count.

---

## Rules

Nine rules. Each one governs one migration phase.

1. **Phase 0 first — audit before touching anything.** Run `lint-arwaky-cli scan .`; record baseline violations to choose strategy.
2. **Taxonomy before contract before capabilities.** VOs must exist before protocols can reference them.
3. **Protocol = one file per feature, one trait per capability seam, each trait rich.** One trait declares every method its capability owns, each with a concrete return type. Never a mega `execute(op, …)` that dispatches multiple features behind one name.
4. **Aggregate = exactly one method.** The single `execute(request)` entry point; the response is a taxonomy-defined enum of VOs. New consumer verbs are new variants on the request enum, not new aggregate methods.
5. **Utility is stateless free functions only.** No class, no interface, no upward imports (AES404).
6. **Capabilities implement protocol; agent implements aggregate.** No cross-layer imports (AES201).
7. **Surface calls aggregate; never imports agent or capabilities.** Dependency arrow points down (AES201 purpose).
8. **Root wires everything; never contains business logic.** Container constructs; entry bootstraps.
9. **Verify at every phase.** `lint-arwaky-cli scan <layer-dir>` → 0 before moving to next phase.

## Workflow

1. **Phase 0 — Audit** — run `lint-arwaky-cli scan .`; record baseline.
2. **Phase 1 — Taxonomy** — extract VOs, errors, constants.
3. **Phase 2 — Contract** — create protocol (one file per feature, one trait per capability seam) + aggregate (exactly one `execute()` method).
4. **Phase 3 — Utility** — extract stateless helpers to shared.
5. **Phase 4 — Capabilities** — implement protocols with business logic.
6. **Phase 5 — Agent** — implement aggregate, delegate to capabilities.
7. **Phase 6 — Surface** — map I/O, call aggregate.
8. **Phase 7 — Root** — wire containers, bootstrap entry.
9. **Phase 8 — Verify** — full scan → 0 violations; compile clean.

---

## Template

Copy, fill, delete nothing. Migration proceeds phase-by-phase; each phase outputs files matching the layer HOW-TO.

### Phase 1 — Taxonomy

```typescript
// packages/shared/src/<domain>/taxonomy_<domain>_vo.ts
export class <VOName> {
  constructor(readonly value: string) {
    if (!value) throw new Error("<VOName> cannot be empty");
  }
}

export interface <EntityName> {
  readonly id: <VOName>;
  readonly name: string;
}
```

### Phase 2 — Contract

```typescript
// packages/shared/src/<domain>/contract_<domain>_protocol.ts
import { RequestVO, ResponseVO } from "./taxonomy_<domain>_vo";
import { <VOName>, <ResultVO> } from "./taxonomy_<domain>_vo";

// One file per feature, one interface per capability seam, each interface rich.
export interface I<Seam1>Protocol {
  /** <What this seam owns.> */
  <operation_1>(arg: <VOName>): Promise<<ResultVO> | null>;
  /** <What this seam owns.> */
  <operation_2>(arg: <VOName>): Promise<<ResultVO> | null>;
}

export interface I<Seam2>Protocol {
  /** <What this seam owns.> */
  <operation_1>(arg: <VOName>): Promise<<ResultVO> | null>;
}

// packages/shared/src/<domain>/contract_<domain>_aggregate.ts
// Exactly one method — the single entry point consumers call.
export interface I<Domain>Aggregate {
  execute(request: RequestVO): Promise<ResponseVO>;
}
```

### Phase 3 — Utility

```typescript
// packages/shared/src/<domain>/utility_<domain>_<role>.ts
import { <VOName> } from "./taxonomy_<domain>_vo";

export function <helper>(arg: <VOName>): <VOName> {
  // Stateless — no class, no interface, no upward imports
  return new <VOName>(arg.value.toLowerCase());
}
```

### Phase 4 — Capabilities

```typescript
// packages/<domain>/src/capabilities_<domain>_<role>.ts
import { I<Domain>Protocol } from "@shared/<domain>/contract_<domain>_protocol";
import { <VOName>, <ResultVO> } from "@shared/<domain>/taxonomy_<domain>_vo";

export class <Capability> implements I<Seam1>Protocol {
  // Dependencies are typed against a contract protocol, never a raw type.
  constructor(private readonly dep: I<SeamDependency>Protocol) {}

  // Every declared method is implemented — a partial implementation is a compile error.
  async <operation_1>(arg: <VOName>): Promise<<ResultVO> | null> {
    // Business logic here
    ...
  }

  async <operation_2>(arg: <VOName>): Promise<<ResultVO> | null> {
    ...
  }
}
```

### Phase 5 — Agent

```typescript
// packages/<domain>/src/agent_<domain>_orchestrator.ts
import { I<Domain>Aggregate } from "@shared/<domain>/contract_<domain>_aggregate";
import { I<Domain>Protocol } from "@shared/<domain>/contract_<domain>_protocol";

export class <Domain>Orchestrator implements I<Domain>Aggregate {
  // The agent holds protocol traits (not the aggregate), dispatched via execute().
  constructor(private readonly proto: I<Seam1>Protocol) {}

  // One method: match the request, dispatch to the owning protocol interface.
  async execute(request: RequestVO): Promise<ResponseVO> {
    if (request instanceof RequestVO.<Variant1>) {
      return new ResponseVO.<Result1>(await this.proto.<operation_1>(..));
    }
    return new ResponseVO.<Result2>(await this.proto.<operation_2>(..));
  }
}
```

### Phase 6 — Surface

```typescript
// packages/<domain>/src/surface_<domain>_command.ts
import { I<Domain>Aggregate } from "@shared/<domain>/contract_<domain>_aggregate";

export function main(aggregate: I<Domain>Aggregate, argv: string[]): number {
  // Parse input, call aggregate, render output — no business logic
  ...
}
```

### Phase 7 — Root

```typescript
// packages/<domain>/src/root_<domain>_container.ts
import { I<Domain>Aggregate } from "@shared/<domain>/contract_<domain>_aggregate";
import { I<Domain>Protocol } from "@shared/<domain>/contract_<domain>_protocol";
import { <Capability> } from "./capabilities_<domain>_<role>";
import { <Domain>Orchestrator } from "./agent_<domain>_orchestrator";

export class <Domain>Container {
  readonly aggregate: I<Domain>Aggregate;

  constructor(dep: I<SeamDependency>Protocol) {
    const proto: I<Seam1>Protocol = new <Capability>(dep);
    this.aggregate = new <Domain>Orchestrator(proto);
  }
}
```

---

## Section Contract

| Section | Why it belongs here |
| ------- | ------------------- |
| Phase number in heading | Makes the migration plan explicit; you know which layer you are on. |
| Rules as numbered list | Each rule prevents one failure mode; agents scan for the first violated rule. |
| Template per phase | Copy-paste-ready skeleton so the agent never guesses the structure. |
| Section Contract table | Justifies each required section; keeps the HOW-TO self-documenting. |
| Verify block | Machine check (lint) + manual check (reader) with exact commands. |

---

## Verify

```bash
# Phase 0 — baseline
lint-arwaky-cli scan .

# Per phase (replace <dir> with the layer directory being migrated)
lint-arwaky-cli scan packages/shared/src/<domain>  # taxonomy + contract + utility
lint-arwaky-cli scan packages/<domain>/src  # capabilities → agent → surface → root

# Final gate
lint-arwaky-cli scan .   # must be 0
npx tsc --noEmit
```

A pass means naming, layer imports, primitives, and roles are clean. Migration strategy
(phase order, skip conditions) is **manual** — see Rules §1–2.
