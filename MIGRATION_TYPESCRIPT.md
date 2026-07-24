# AES Migration Guide — TypeScript

> Skill-driven migration workflow for TypeScript/JavaScript projects to AES architecture.
> Each phase delegates to a dedicated skill in `.agents/skills/`.

See [ARCHITECTURE.md](ARCHITECTURE.md) for layer rules and [README.md](README.md) for project usage.

## Workspace Structure

```
project-root/
├── package.json             ← workspace root config
├── pnpm-workspace.yaml      ← pnpm workspace definition (if using pnpm)
├── packages/
│   ├── shared/              ← shared taxonomy + contract + utility types
│   │   ├── package.json
│   │   └── src/
│   │       ├── index.ts
│   │       ├── common/          ← truly shared across ALL features
│   │       └── <feature>/       ← shared types per feature domain
│   │
│   ├── <feature>/           ← feature package
│   │   ├── package.json
│   │   └── src/
│   │       ├── index.ts
│   │       ├── capabilities_<concept>_<role>.ts
│   │       ├── agent_<concept>_orchestrator.ts
│   │       ├── surface_<concept>_<role>.ts
│   │       └── root_<concept>_container.ts
│   └── ...
└── src/
    └── root_<name>_entry.ts   ← entry point (at workspace root)
```

**Key rules:**

- All 7 layers coexist in each feature slice.
- Stable domain taxonomy, contracts, and utilities live under `packages/shared/src/<feature>/`.
- Orchestration, capabilities, and surfaces live in the feature package.
- Entry points (`root_*_entry.ts`) live at workspace root or `src/`.

---

## Prerequisites

```bash
npm install -g lint-arwaky-cli
lint-arwaky-cli version
lint-arwaky-cli scan your-project/
```

---

## Phase 0: Audit

> **Skill:** `lint-arwaky-typescript` — load for audit commands and violation analysis.

```bash
lint-arwaky-cli scan your-project/
find your-project/packages -name "*.ts" | wc -l
```

- Violations < 10 → full migration in one session
- Violations 10-50 → phased migration
- Violations > 50 → start with taxonomy only

---

## Phase 1: Taxonomy Layer

> **Skill:** `create-taxonomy-typescript` — load for VOs, errors, constants, entities, events.

Define Value Objects, Errors, Events, and compile-time Constants under `packages/shared/src/<feature>/`.

### Steps

1. Identify domain types with `grep -rn "^export interface\|^export type\|^export enum" packages/*/src/`
2. Load `create-taxonomy-typescript` skill
3. Create taxonomy files following skill templates and workflow
4. Register in domain `index.ts`
5. Verify: `npx tsc --noEmit`

---

## Phase 2: Contract Layer

> **Skill:** `create-contract-typescript` — load for protocol and aggregate interfaces.

Contracts define public interfaces (Protocols and Aggregates) without exposing implementation.

### Steps

1. Load `create-contract-typescript` skill
2. Create protocol interfaces (inbound/outbound) under `packages/shared/src/<feature>/`
3. Create aggregate facades under `packages/shared/src/<feature>/`
4. Register in domain `index.ts`
5. Verify: `npx tsc --noEmit`

---

## Phase 3: Utility Layer

> **Skill:** `create-utility-typescript` — load for stateless standalone functions.

Utility contains low-level technical mechanics — **stateless standalone functions only**.

### Steps

1. Identify reusable stateless functions across modules
2. Load `create-utility-typescript` skill
3. Create utility files under `packages/shared/src/<feature>/`
4. Register in domain `index.ts`
5. Verify: `npx tsc --noEmit`

---

## Phase 4: Capabilities Layer

> **Skill:** `create-capabilities-typescript` — load for business logic and external adaptation.

Capabilities contain concrete behavior implementations (business logic + external adapters).

### Steps

1. Load `create-capabilities-typescript` skill
2. Create business logic capabilities (implement protocol interfaces)
3. Create external adaptation capabilities (repositories, clients)
4. Verify: `npx tsc --noEmit`

---

## Phase 5: Agent Layer

> **Skill:** `create-agent-typescript` — load for orchestration logic.

Orchestrates sequential execution, branching, looping, and error handling.

### Steps

1. Load `create-agent-typescript` skill
2. Create orchestrator class implementing aggregate interface
3. Inject protocol dependencies via constructor
4. Verify: `npx tsc --noEmit`

---

## Phase 6: Surface Layer

> **Skill:** `create-surface-typescript` — load for user-facing input translation.

Translates user-facing inputs into actions, delegating to the Agent orchestrator.

### Steps

1. Load `create-surface-typescript` skill
2. Create surface classes (commands, handlers, endpoints)
3. Inject aggregate interface via constructor
4. Verify: `npx tsc --noEmit`

---

## Phase 7: Root Layer

> **Skill:** `create-root-typescript` — load for DI container and entry point wiring.

Wires concrete implementations to contracts and bootstraps the system.

### Steps

1. Load `create-root-typescript` skill
2. Create DI container wiring all capabilities → orchestrator → surface
3. Create entry point at workspace root or `src/`
4. Verify: `npx tsc --noEmit`

---

## Phase 8: Verify

> **Skill:** `build-verify-all` — load for final build verification.

```bash
lint-arwaky-cli scan your-project/
npx tsc --noEmit
vitest run
npm run lint && npm run format
```

---

## Supplementary Skills (Post-Migration)

| Skill | When to Use |
|-------|-------------|
| `add-docs-typescript` | Add JSDoc, type annotations after migration |
| `fix-bypass-typescript` | Remove `@ts-ignore`, `@ts-expect-error` |
| `cleanup-consolidate-typescript` | Remove dead code, merge duplicates |
| `create-test-typescript` | Generate test suites |

---

## Reference: File Naming & Import Rules

See [ARCHITECTURE.md](ARCHITECTURE.md) §3 (Naming Convention) and §11 (Import Rules).

| Layer | Pattern |
|-------|---------|
| taxonomy | `taxonomy_<concept>_<suffix>.ts` |
| contract | `contract_<concept>_<suffix>.ts` |
| utility | `utility_<concept>_<suffix>.ts` |
| capabilities | `capabilities_<concept>_<suffix>.ts` |
| agent | `agent_<concept>_orchestrator.ts` |
| surface | `surface_<concept>_<suffix>.ts` |
| root | `root_<concept>_<suffix>.ts` |

---

## Troubleshooting

See [ARCHITECTURE.md](ARCHITECTURE.md) §12 (Troubleshooting) for violation codes and fixes.
