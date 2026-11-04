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

| Section | Why it belongs here|
| ------- | -------------------|
| Phase number in heading | Makes the migration plan explicit; you know which layer you are on.|
| Rules as numbered list | Each rule prevents one failure mode; agents scan for the first violated rule.|
| Template per phase | Copy-paste-ready skeleton so the agent never guesses the structure.|
| Section Contract table | Justifies each required section; keeps the HOW-TO self-documenting.|
| Verify block | Machine check (lint) + manual check (reader) with exact commands.|

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

---

## TypeScript Lint (Verify Pipeline)

> Shared commands, MCP tools, exit codes, and AES101 naming live in `aes-migration/SKILL.md` (Lint Rules — Global).
> This file only holds TypeScript deltas: adapters, scan session, TypeScript-specific fix routing, and role boundaries.

### Adapters & external tooling

`lint-arwaky-cli install` installs external adapters; `lint-arwaky-cli adapters` lists the active ones (ESLint, TSC, …); `lint-arwaky-cli external packages/` runs only those linters. `get_config` accepts `"language": "typescript"`.

```bash
npx tsc --noEmit
npx eslint src/ --ext .ts,.tsx,.js,.jsx
npx vitest run    # or npm test
```

### Scan session

```bash
# Pre-flight — package must typecheck
node --version && npm --version
npm install
npx tsc --noEmit

# Scan (text / json / by member / by rule)
lint-arwaky-cli scan workspaces-bad/packages
lint-arwaky-cli scan workspaces-bad/packages --format json
lint-arwaky-cli scan workspaces-bad/packages --member animator
lint-arwaky-cli scan workspaces-bad/packages --filter AES201
lint-arwaky-cli fix packages/ --dry-run --filter AES101
lint-arwaky-cli ci packages/ --threshold 80
lint-arwaky-cli scan packages/ --format junit -o reports/
lint-arwaky-cli orphan packages/ --format json
lint-arwaky-cli role packages/ --filter AES403
lint-arwaky-cli security packages/
lint-arwaky-cli dependencies packages/

# Reports → XDG data dir
lint-arwaky-cli scan packages/ --format json \
  > ~/.local/share/aes-lint-arwaky/reports/scan_typescript.json
lint-arwaky-cli scan packages/ --format sarif \
  > ~/.local/share/aes-lint-arwaky/reports/scan_typescript.sarif
```

### TypeScript-specific fix routing

- **AES101 rename** → after `git mv`, update `index.ts` barrel exports (missing from barrel = orphan AES501–506).
- **AES201 forbidden import** → remove the cross-layer `import { … } from "…"`; depend on the contract interface injected through DI (`aes-contract`).
- **AES303** → add the missing `class` / `interface` / `enum` (skipped for `utility`/`surface` layers).
- **AES304** → remove `@ts-ignore`, `@ts-expect-error`, `@ts-nocheck`, `eslint-disable` (`lint-arwaky-cli fix <path> --filter AES304`).
- **AES403** → every capability class MUST implement its protocol interface.
- **AES405** → every agent class MUST implement its aggregate interface; use concrete types instead of `any`.
- **AES404** → `utility_*` modules export pure functions only; import `taxonomy` and nothing else.

### Role boundaries

| Layer | Can contain | Cannot contain |
| :----- | :----- | :----- |
| capabilities | Pure computation, validation | I/O, network, database |
| utility | Stateless exported functions | State, classes, non-taxonomy imports |
| agent | Orchestration flow | Computation, I/O, business |

### Quick fix recipes

```bash
# AES101/102 rename
git mv src/capabilities_scanner.ts src/capabilities_file_scanner.ts
# then update index.ts barrel exports

# AES203 unused import
lint-arwaky-cli fix src/file.ts --filter AES203

# AES304 bypass
grep -rn '@ts-ignore\|@ts-expect-error\|@ts-nocheck\|eslint-disable\|FIXME\|TODO' src/
# fix root cause, delete suppression

# AES501–506 orphan
lint-arwaky-cli orphan packages/ --format json
# truly dead → delete; should be wired → container / barrel chain

# AES401–406 role
lint-arwaky-cli role packages/ --filter AES403
# I/O out of capabilities → utility; business out of surface → capabilities;
# orchestration out of capabilities → agent; surface uses aggregate interfaces only
```

---

## TypeScript Section Contract

| Section | Why it belongs here |
| :----- | :----- |
| Pre-flight typecheck | A scan on a non-compiling package is noise, not signal. |
| Scan with path + member + format | Findings must be machine-triageable and package-scoped. |
| Triage order | CRITICAL (AES201/205/304) blocks merge; fix before HIGH/MEDIUM cosmetics. |
| Fix routing by AES code | Wrong fix layer recreates the violation; route via lint-routing in `aes-migration/SKILL.md`. |
| Barrel update after rename | AES101 without `index.ts` re-export becomes AES501–506. |
| Role-boundary check | Capabilities/utility/agent rows above must hold after fixes. |
| Re-scan to 0 | Done means zero AES findings, not "mostly clean". |
| Language verify | `tsc` / eslint / tests still pass — AES clean ≠ package healthy. |

---

## TypeScript Verify

```bash
# 1. Re-scan — must be 0 AES violations
lint-arwaky-cli scan <target-path>

# 2. Typecheck
npx tsc --noEmit

# 3. ESLint (if configured)
npx eslint src/ --ext .ts,.tsx,.js,.jsx

# 4. Tests (if configured)
npx vitest run  # or npm test
```

Exit codes (shared, `aes-migration/SKILL.md`): `0` pass · `1` violations · `2` config/parse error.

Manual:

- [ ] Layer imports follow AES201.
- [ ] Capability classes implement their protocol interface (AES403).
- [ ] Utility modules are stateless exported functions importing `taxonomy` only (AES404).
- [ ] Agent classes implement their aggregate interface; no `any` (AES405).
- [ ] Surface files follow role-based imports (AES406).
- [ ] `npx tsc --noEmit` and `npx eslint src/ --ext .ts,.tsx,.js,.jsx` clean.
- [ ] `lint-arwaky-cli scan packages/` reports 0 violations.
