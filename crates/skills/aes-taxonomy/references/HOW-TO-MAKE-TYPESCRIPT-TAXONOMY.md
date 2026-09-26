# HOW TO MAKE TAXONOMY TYPESCRIPT

> **Purpose**: Define the stable language of the domain: value objects, entities, errors, events, and constants.
>
> **Audience**: Agents and engineers scaffolding AES taxonomy files in the shared domain.
>
> **Scope**: TypeScript taxonomy files named `taxonomy_<domain>_<concept>_<suffix>.ts` using suffixes `_vo`, `_entity`, `_error`, `_event`, `_constant` only.
>
> **Location**: TypeScript shared domain source root next to contracts:
>
> ```text
> packages/shared/src/<domain>/
> ```
>
> *(Note: If your monorepo uses `modules/` instead of `packages/`, substitute accordingly, but keep the `src/<domain>/` structure.)*
>
> TypeScript taxonomy files must be registered in the domain `index.ts`, which is the TypeScript shared barrel.
>
> **Length**: One taxonomy type per file. Constants files must be at least 5 lines per AES302. No I/O, no upward imports, no primitives for public/domain fields.

---

## Rules

### Import rules

**Allowed imports:**

```text
Other taxonomy types
Pure node stdlib modules (e.g., node:util) when strictly necessary for pure logic
```

**Forbidden imports:**

```text
capabilities
agents
surface
root
contracts
fs / path (when used for I/O)
fetch / http / https
database drivers
any module with side effects
```

Taxonomy files must not perform I/O, network access, database access, filesystem access, environment access, randomness, or time retrieval.

---

### File-name pattern

Use:

```text
taxonomy_<domain>_<concept>_<suffix>.ts
```

Where:

```text
<domain>  = snake_case domain name (underscores only; hyphens fail AES101)
<concept> = snake_case concept name; may contain multiple underscores
<suffix>  = one of the allowed taxonomy suffixes
```

The pattern has three placeholders after `taxonomy`, but because `<concept>` may itself be multi-word, the final filename may contain more than three underscore-separated segments.

Examples:

```text
taxonomy_order_order_id_vo.ts
taxonomy_order_money_vo.ts
taxonomy_order_order_entity.ts
taxonomy_order_order_not_found_error.ts
taxonomy_order_order_created_event.ts
taxonomy_order_order_constant.ts
```

The filename must match the taxonomy type it contains.

Example mapping:

```text
taxonomy_order_order_id_vo.ts           -> OrderId
taxonomy_order_order_entity.ts          -> Order
taxonomy_order_order_not_found_error.ts -> OrderNotFoundError
taxonomy_order_order_created_event.ts   -> OrderCreated
taxonomy_order_order_constant.ts        -> ORDER_* constants
```

Allowed suffixes are exactly:

```text
vo
entity
error
event
constant
```

Anything else fails AES102. The filename itself must use underscores only — kebab-case (`taxonomy-order-money-vo.ts`) fails AES101.

---

### File-name suffix table

| Suffix | Content | Key constraint |
|---|---|---|
| `_vo.ts` | Value Objects | `readonly` fields, validate in constructor, no I/O |
| `_entity.ts` | Entities with identity | Identity VO field required; domain fields must be VOs |
| `_error.ts` | Domain errors | Extend `Error`, set `this.name`, store VO fields only |
| `_event.ts` | Domain events | Immutable (`readonly`), VO payload fields only |
| `_constant.ts` | Compile-time constants | `export const` only — no functions, no I/O |

---

### VO primitive rules (AES401)

AES401 forbids raw primitives as public/domain fields.

There is no `boolean` carve-out. `boolean` is treated as a primitive.

The implementation primitive set is broader than the most common examples. It includes at least:

```text
string
number
boolean
bigint
symbol
Array
Record
Map
Set
any
unknown
object
null
undefined
```

Public/domain fields in entities, events, errors, and composite VOs must be taxonomy VOs, not primitives.

A leaf VO may privately encapsulate a primitive value (like `string` or `number`) for validation and representation. That internal encapsulation is a modeling convention.

**Important limitation:** AES401 does **not** scan `_vo` files for primitives. The check is gated on the `_entity`/`_error`/`_event` suffixes. A primitive inside a `_vo` file will not trigger a violation. Trust discipline, not enforcement, for VOs.

A leaf VO may expose a read-only accessor (e.g., `get value(): string`) for its wrapped primitive, but domain boundaries must pass the VO itself, not the primitive.

Constants are the explicit exception for literal values because constant files contain compile-time literals, not domain fields.

---

### File length (AES302)

Every taxonomy file must be at least **5 lines** or the linter reports:

```text
[AES302] FILE_TOO_SHORT: File contains fewer than the required minimum lines.
FIX: Expand the component or merge this logic into a related module. (min: 5).
```

JSDoc comments and `export const` lines count. For constants, each constant should have a descriptive JSDoc line above it — this keeps the file above the minimum while also documenting intent.

---

## Workflow

1. Determine type:
   ```text
   VO
   Entity
   Error
   Event
   Constant
   ```

2. Create the file:
   ```text
   taxonomy_<domain>_<concept>_<suffix>.ts
   ```
   inside:
   ```text
   packages/shared/src/<domain>/
   ```

3. For VOs:
   ```text
   Validate in the constructor.
   Throw an Error on invalid input.
   Ensure fields are readonly.
   No I/O.
   ```

4. For Entities:
   ```text
   Include an identity VO field.
   Use VO fields only for domain state.
   Do not use raw primitives for public/domain fields.
   ```

5. For Errors:
   ```text
   Extend the built-in Error class.
   Set this.name to the class name.
   Store VO fields only — no raw string, number, or Record payloads.
   Expose an error_id getter (stable numeric id) for fast machine branching.
   Expose an error_code getter (stable string identifier) for human-readable
     error classification.
   Provide a message getter that returns a human-readable description
     derived from the stored VOs, error id, and error code.
   Do not store raw string messages as domain payloads.
   ```

6. For Events:
   ```text
   Use readonly fields.
   Use VO payload fields only.
   Name events as past-tense domain facts.
   ```

7. For Constants:
   ```text
   Use export const only.
   No functions.
   No computed values at runtime.
   No I/O.
   ```

8. Register the public type in the domain `index.ts`.

9. Verify the project compiles:
   ```bash
   npx tsc --noEmit
   ```

---

## Template

### Value Object

```typescript
export class <Name> {
    private readonly _value: string;

    constructor(value: string) {
        if (!value.trim()) {
            throw new Error('<Name> cannot be empty');
        }
        this._value = value;
    }

    get value(): string {
        return this._value;
    }

    toString(): string {
        return this._value;
    }
}
```

Concrete example:

```typescript
export class OrderId {
    private readonly _value: string;

    constructor(value: string) {
        if (!value.trim()) {
            throw new Error('OrderId cannot be empty');
        }
        this._value = value;
    }

    get value(): string {
        return this._value;
    }

    toString(): string {
        return this._value;
    }
}
```

---

### Entity

Use parameter properties to enforce immutability and conciseness.

```typescript
import { <Name>Id } from './taxonomy_<domain>_<concept>_id_vo';
// import other VOs

export class <Name> {
    constructor(
        public readonly id: <Name>Id,
        // other VO fields
    ) {}
}
```

Concrete example:

```typescript
import { OrderId } from './taxonomy_order_order_id_vo';
import { Money } from './taxonomy_order_money_vo';

export class Order {
    constructor(
        public readonly id: OrderId,
        public readonly total: Money,
    ) {}
}
```

Do not do this:

```typescript
export class Order {
    constructor(
        public readonly id: string,
        public readonly total: number,
    ) {}
}
```

Entity fields must be taxonomy VOs.

---

### Error

When extending built-in classes like `Error` in TypeScript, you must restore the prototype chain to ensure `instanceof` works correctly.

```typescript
import { <VO> } from './taxonomy_<domain>_<concept>_vo';

export class <Name>Error extends Error {
    constructor(
        public readonly <field>: <VO>,
    ) {
        super();
        this.name = '<Name>Error';
        Object.setPrototypeOf(this, <Name>Error.prototype);
    }

    /** Stable numeric id. Callers branch on this, never on the message. */
    get error_id(): number {
        return <NNNN>;
    }

    /** Stable machine-readable name. Callers branch on this, never on the message. */
    get error_code(): string {
        return '<DOMAIN>_<REASON>';
    }

    /** Typed message derived from the error id, code, and stored VOs — required by AES best practice. */
    get message(): string {
        return `${this.error_id} ${this.error_code}: <Description>: ${this.<field>.toString()}`;
    }
}
```

Concrete example:

```typescript
import { OrderId } from './taxonomy_order_order_id_vo';

export class OrderNotFoundError extends Error {
    constructor(
        public readonly orderId: OrderId,
    ) {
        super();
        this.name = 'OrderNotFoundError';
        Object.setPrototypeOf(this, OrderNotFoundError.prototype);
    }

    /** Stable numeric id. Callers branch on this, never on the message. */
    get error_id(): number {
        return 1001;
    }

    /** Stable machine-readable name. Callers branch on this, never on the message. */
    get error_code(): string {
        return 'ORDER_NOT_FOUND';
    }

    get message(): string {
        return `${this.error_id} ${this.error_code}: order not found: ${this.orderId.toString()}`;
    }
}
```

Every error must expose all four:
- **Field VO** — programmatic access to the domain data that caused the error
- **Error id** — a stable numeric id such as `1001`, so APIs, database rows,
  and monitoring queries can correlate on a compact value
- **Error code** — a stable string name such as `ORDER_NOT_FOUND`, readable in
  logs without a lookup table
- **Message** — a human-readable description derived from the error id, the
  error code, and the stored VOs

Both the error id and the error code stay stable across releases. Renaming or
reformatting the message is a compatible change; changing the id or the code
is a breaking one. Allocate ids in blocks per domain (order errors from 1000,
billing errors from 2000) so two domains cannot collide on the same number.

Good:

```typescript
public readonly orderId: OrderId
public readonly amount: Money
```

Bad:

```typescript
public readonly orderId: string
public readonly message: string              // raw string payload
public readonly payload: Record<string, unknown>
```

---

### Event

```typescript
import { <VO> } from './taxonomy_<domain>_<concept>_vo';

export class <Name> {
    constructor(
        public readonly <field>: <VO>,
    ) {}
}
```

Concrete example:

```typescript
import { OrderId } from './taxonomy_order_order_id_vo';

export class OrderCreated {
    constructor(
        public readonly orderId: OrderId,
    ) {}
}
```

Events must be immutable and use VO payload fields only.

---

### Constants

Use the singular `_constant` suffix:

```text
taxonomy_<domain>_<concept>_constant.ts
```

Template:

```typescript
/** Default value description. */
export const <NAME>_DEFAULT: number = 24.0;

/** Minimum value description. */
export const <NAME>_MIN: number = 0.5;

/** Filename constant. */
export const <NAME>_FILENAME: string = 'file.json';
```

Concrete example:

```typescript
/** Default order refresh interval in hours. */
export const ORDER_REFRESH_INTERVAL_DEFAULT: number = 24.0;

/** Minimum order refresh interval in hours. */
export const ORDER_REFRESH_INTERVAL_MIN: number = 0.5;

/** Order snapshot filename. */
export const ORDER_SNAPSHOT_FILENAME: string = 'order_snapshot.json';
```

Constants must be pure literals.

Forbidden:

```typescript
export const MAX_ITEMS = process.env.MAX_ITEMS ? parseInt(process.env.MAX_ITEMS) : 100;
export const getFilename = () => 'file.json';
```

> The JSDoc comments above each constant are not decoration — they keep the file
> above the AES302 five-line minimum. A bare two-constant file fails.

---

### Registration

Register each public taxonomy type in the domain `index.ts` using explicit named exports to prevent accidental leaks and improve tree-shaking.

Example:

```typescript
// packages/shared/src/order/index.ts

export { OrderId } from './taxonomy_order_order_id_vo';
export { Order } from './taxonomy_order_order_entity';
export { OrderNotFoundError } from './taxonomy_order_order_not_found_error';
export { OrderCreated } from './taxonomy_order_order_created_event';
```

---

## Section Contract

| Check | Enforcement | Why it belongs here |
|---|---|---|
| Correct file pattern: `taxonomy_<domain>_<concept>_<suffix>.ts` (underscores only). | Machine-checked: AES101. | Required by AES layer rules and the linter; missing it is a defect. |
| Correct suffix: `_vo`, `_entity`, `_error`, `_event`, `_constant`. | Machine-checked: AES102. | Required by AES layer rules and the linter; missing it is a defect. |
| File is at least 5 lines. | Machine-checked: AES302. | Required by AES layer rules and the linter; missing it is a defect. |
| No forbidden layer imports from capabilities, agents, surface, root, contracts. | Partially machine-checked: AES201–AES205. See Verify. | Required by AES layer rules; missing it is a defect. |
| Registered in shared `index.ts`. | Machine-checked: AES501 orphan check. | Required by AES layer rules and the linter; missing it is a defect. |
| Constant file contains only constant material, not classes/functions. | Machine-checked: AES401 on constant files. | Required by AES layer rules and the linter; missing it is a defect. |
| One taxonomy type per file. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| VOs validate on construction. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| VOs are immutable (`readonly`). | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Leaf VOs encapsulate only the primitive needed for validation. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Composite VOs use other VOs, not raw primitives. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Public/domain fields avoid primitives. | Convention — not machine-checked. The linter does not inspect TypeScript class properties. | Required by AES taxonomy convention; missing it is a defect. |
| Entities contain an identity VO field. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Entities use VO fields for domain state. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Events are immutable (`readonly`). | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Events use VO payload fields only. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Errors extend `Error` and set `this.name`. | Convention for semantic correctness; structural inheritance is checked by compiler. | Required by AES taxonomy convention; missing it is a defect. |
| Errors store VO fields only. | Convention — not machine-checked. The reader verifies this; the linter does not. | Required by AES taxonomy convention; missing it is a defect. |
| Errors expose an `error_id` getter (stable numeric id), an `error_code` getter (stable string name), and a `message` getter derived from their VOs. | Best practice — not machine-checked. Enables callers to branch on `error_id`/`error_code` and to read the description without string matching. | Required by AES best practice; missing it is a defect. |
| Constants are `export const` pure literal values. | Convention for literal purity; structural violations may be machine-checked. | Required by AES taxonomy convention; missing it is a defect. |
| No I/O, network, database, filesystem, environment, randomness, or time retrieval. | Convention — not machine-checked fully. The reader verifies this; the linter does not guarantee it. | Required by AES taxonomy convention; missing it is a defect. |
| `npx tsc --noEmit` passes. | Manual fallback gate. | Required by the TypeScript compilation check; missing it is a defect. |

---

## Verify

```bash
lint-arwaky-cli scan <layer-path>
```

Machine-checked areas include:

```text
AES101: filename pattern (underscores only)
AES102: suffix correctness
AES203: unused imports
AES204: dummy functions
AES205: circular dependencies
AES301/AES302: file length bounds
AES401: non-constant declarations inside _constant files
AES501–AES506: orphan files and reachability
```

### What is convention, not enforcement

```text
Import boundary rules (AES201/AES202) — the import matrix above is the target
  architecture. Do not treat a clean scan as proof the boundaries hold.

Primitive rules (AES401 on _vo and _entity files) — VOs are not scanned for
  primitives, and TypeScript class properties are not either. A `public readonly
  name: string` in an entity will not trigger AES401. Trust discipline: keep
  domain fields as VOs by convention, not by expectation of a violation.

Class/inheritance rules (AES403/AES405) — verify these by reading.
```

A pass means the mechanically-checkable rules are satisfied. The boundary and
primitive rules above are the reader's responsibility.

### Orphan reachability — read this before Phase 2

A taxonomy file that is only referenced by barrels and other taxonomy files
is reported as an orphan:

```
[AES501] 'taxonomy_order_order_id_vo' is not reachable and not imported by higher layers.
WHY? only imported by lower-layer files (index.ts, taxonomy_order_entity.ts)
FIX: Import 'taxonomy_order_order_id_vo' from a contract_* or higher-layer file.
```

Barrel registration alone does **not** clear this. The shared barrel satisfies
the "has importers" half; you also need a `contract_*` file to import the
taxonomy type. However, the contract itself becomes an orphan (`AES502`)
until it is referenced by a capability, agent, or root file. The fix is
structural, not per-file: wire the full chain through root, then rescan.
Individual layer phases are expected to show orphan violations — they clear
once the complete dependency graph is in place. Do not treat an isolated
taxonomy scan as the final gate.

Fallback compile gate:

```bash
npx tsc --noEmit
```

Related: [HOW-TO-MAKE-PYTHON-TAXONOMY.md](HOW-TO-MAKE-PYTHON-TAXONOMY.md), [HOW-TO-MAKE-RUST-TAXONOMY.md](HOW-TO-MAKE-RUST-TAXONOMY.md)
