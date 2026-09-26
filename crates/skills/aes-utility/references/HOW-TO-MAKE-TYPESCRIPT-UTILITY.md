# HOW TO MAKE UTILITY TYPESCRIPT

> **Purpose**: Hold stateless, domain-agnostic helper functions shared across the system.
>
> **Audience**: Agents and engineers scaffolding AES utility files in the shared domain.
>
> **Scope**: Python, Rust, and TypeScript `utility_<domain>_<role>` files — free functions only, no class/struct/impl.
>
> **Location**: Shared domain source root next to taxonomy, registered in the shared barrel.
>
> **Length**: Free functions only; used by ≥2 modules; taxonomy-only imports.

---

## Rules

### Import rules

**Allowed imports:** Taxonomy only (`shared/taxonomy_*`).
**Forbidden:** import from Capabilities, Agent, Surface, Contract.

1. Only exported functions — no `class`.
2. Pure + deterministic — no `Math.random()`, no `Date.now()`, no global mutable state.
3. Domain-agnostic — no business rules, no layer-name knowledge.
4. Reusable — used by ≥2 modules; if single consumer → keep as private helper.
5. I/O allowed only if all above hold.

**Keep as private helper** if ANY: uses `this`, domain-specific, single consumer.
**Extract here** only if ALL: no `this`, pure/I/O-safe, domain-agnostic, ≥2 consumers.

### Workflow

1. Confirm ≥2 consumers, stateless, domain-agnostic.
2. Create `utility_<domain>_<role>.ts`.
3. Register in `index.ts`.
4. `npx tsc --noEmit`.

---

## Template

```typescript
/** <Domain> utility functions — stateless, pure, domain-agnostic.

Exported functions only — no classes, no state.
*/

// import type { UserVO } from "./taxonomy_user_vo";  // uncomment if using VOs

/** <Description of what this function does> */
export function <functionName>(<paramName>: string): string {
  // pure function logic here
  return "";
}

/** <Description of what this function does> */
export function <functionName>(<paramName>: string): string {
  // pure function logic here
  return "";
}
```

---

## Section Contract

| Check | Why it belongs here |
| ----- | ------------------- |
| Only exported functions — no class. | Required by AES layer rules and the linter; missing it is a defect. |
| No `this`, no instance state. | Required by AES layer rules and the linter; missing it is a defect. |
| Pure/deterministic (or I/O justified: domain-agnostic + reusable). | Required by AES layer rules and the linter; missing it is a defect. |
| No business rules or layer-name knowledge. | Required by AES layer rules and the linter; missing it is a defect. |
| Used by ≥2 modules. | Required by AES layer rules and the linter; missing it is a defect. |
| No import from Capabilities, Agent, Surface, Contract. | Required by AES layer rules and the linter; missing it is a defect. |
| No magic constants (→ `taxonomy_*_constant.ts`). | Required by AES layer rules and the linter; missing it is a defect. |
| `npx tsc --noEmit` passes. | Required by AES layer rules and the linter; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
# Checks: AES101/AES102 (filename + suffix), AES201–AES205 (layer imports),
# AES401–AES406 (role/primitive/structure rules for this layer).
# Manual (not machine-checked): stateless, domain-agnostic, ≥2 consumers; no class/`self`/`this`/struct/impl.
# Fallback compile gate: npx tsc --noEmit
```
