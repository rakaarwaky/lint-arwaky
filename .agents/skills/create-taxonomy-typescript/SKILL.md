---
name: create-taxonomy-typescript
description: "Create and validate TypeScript taxonomy layer files in shared taxonomy: VOs, entities, errors, events, and constants. Taxonomy is the domain foundation layer — stable language of the domain, free from technical or behavioral concerns."
metadata:
  tags: [typescript, aes, taxonomy, shared, vo, entity, error, event, constant, primitive-to-vo]
  triggers:
    - "create taxonomy typescript"
    - "add taxonomy typescript"
    - "move dataclass to taxonomy typescript"
    - "create vo typescript"
    - "create error taxonomy typescript"
    - "create constant taxonomy typescript"
    - "check taxonomy typescript"
    - "audit taxonomy typescript"
  dependencies: []
  related:
    - create-capabilities-typescript
    - create-agent-typescript
    - create-contract-typescript
---
# create-taxonomy-typescript

Taxonomy = stable domain language. Single source of truth for VOs, entities, errors, events, constants. Location: `packages/shared/src/<domain>/`.

**Allowed imports:** other taxonomy types, stdlib (`node:path`, etc.).
**Forbidden:** capabilities, agents, surface, root, contracts, `fs.`/`fetch`/database (in VOs/entities/errors/events/constants).

## File Types


| Suffix         | Content                | Key constraint                                     |
| ---------------- | ------------------------ | ---------------------------------------------------- |
| `_vo.ts`       | Value Objects          | `readonly` fields, validate in constructor, no I/O |
| `_entity.ts`   | Entities with identity | Identity VO field required                         |
| `_error.ts`    | Domain errors          | `extends Error`, set `this.name`                   |
| `_event.ts`    | Domain events          | Immutable, VO payload fields                       |
| `_constant.ts` | Compile-time constants | `export const` only — no functions                |
| `_utility.ts`  | Stateless helpers      | No class, no`this`, domain-agnostic                |

## VO Rules (AES401/AES402)

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
export class <Name>Error extends Error {
    constructor(message: string) {
        super(message);
        this.name = '<Name>Error';
    }
}
```

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
4. Errors: `extends Error`, set `this.name`.
5. Constants: `export const NAME = value` only.
6. Register in `index.ts`.
7. `npx tsc --noEmit`.

## Checklist

- [ ]  Correct suffix.
- [ ]  VOs: `readonly` fields, validate on construction; composite VOs use other VOs.
- [ ]  Errors extend `Error`, set `this.name`.
- [ ]  Constants are `export const` pure literal values.
- [ ]  No import from capabilities, agents, surface, root, contracts.
- [ ]  No I/O, network, or database in taxonomy files.
- [ ]  Registered in shared `index.ts`.
- [ ]  `npx tsc --noEmit` passes.
