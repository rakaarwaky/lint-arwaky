# Taxonomy — TypeScript

Source layout: `packages/shared/src/<domain>/`. File: `taxonomy_<domain>_<type>.ts`.

## Import rules

**Allowed imports:** other taxonomy types, stdlib (`node:path`, etc.).
**Forbidden:** capabilities, agents, surface, root, contracts, `fs.`/`fetch`/database (in VOs/entities/errors/events/constants).

## File-name suffix table

| Suffix         | Content                | Key constraint                                     |
| ---------------- | ------------------------ | ---------------------------------------------------- |
| `_vo.ts`       | Value Objects          | `readonly` fields, validate in constructor, no I/O |
| `_entity.ts`   | Entities with identity | Identity VO field required                         |
| `_error.ts`    | Domain errors          | `extends Error`, set `this.name`                   |
| `_event.ts`    | Domain events          | Immutable, VO payload fields                       |
| `_constant.ts` | Compile-time constants | `export const` only — no functions                |
| `_utility.ts`  | Stateless helpers      | No class, no`this`, domain-agnostic                |

## VO primitive rules (AES401)

Forbidden for domain fields: `string`, `number`, `string[]`, `Record<string,T>`.
`boolean` allowed for semantic toggles only.

## Templates

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

### Entity

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

### Error

```typescript
import { <VO> } from './taxonomy_<name>_vo';

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

    /** Typed message derived from the error id, code, and VOs. */
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
- **`error_id`** — a stable numeric id such as `1001`, for API/DB/monitoring correlation
- **`error_code`** — a stable string name such as `ORDER_NOT_FOUND`, readable in logs
- **`message`** — a human-readable description derived from the id, code, and VOs

Both the `error_id` and `error_code` stay stable across releases. Renaming or
reformatting `message` is a compatible change; changing either id or code is
breaking. Allocate ids in per-domain blocks (order from 1000, billing from 2000)
to avoid cross-domain collisions.

Never store a raw `string` or `Record<string, unknown>` as a domain payload — the
VO, `error_id`, and `error_code` carry the structure; `message` carries the prose.

### Constants

```typescript
/** Default value description. */
export const <NAME>_DEFAULT: number = 24.0;

/** Minimum value description. */
export const <NAME>_MIN: number = 0.5;

/** Filename constant. */
export const <NAME>_FILENAME: string = 'file.json';
```

## Workflow

1. Determine type (VO/Entity/Error/Event/Constant/Utility).
2. Create `taxonomy_<domain>_<type>.ts` in `shared/src/<domain>/`.
3. VOs: `readonly` fields, validate in constructor, throw on invalid.
4. Errors: `extends Error`, set `this.name`; expose `error_id`, `error_code`, and
   `message` derived from the stored VOs.
5. Constants: `export const NAME = value` only.
6. Register in `index.ts`.
7. `npx tsc --noEmit`.

## Checklist

- [ ]  Correct suffix.
- [ ]  VOs: `readonly` fields, validate on construction; composite VOs use other VOs.
- [ ]  Errors extend `Error`, set `this.name`, expose `error_id`, `error_code`, and `message`.
- [ ]  Constants are `export const` pure literal values.
- [ ]  No import from capabilities, agents, surface, root, contracts.
- [ ]  No I/O, network, or database in taxonomy files.
- [ ]  Registered in shared `index.ts`.
- [ ]  `npx tsc --noEmit` passes.
