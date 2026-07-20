# The 3-Block Structure

Every implementation file MUST follow this order **within the class body** with mandatory block markers:

1. **Block 1 — Class Definition & Constructor**
2. **Block 2 — Domain Protocol Method Implementation**
3. **Block 3 — Utility Methods, Factories, and Private Helpers**

Each block MUST be preceded by a block marker comment:

```typescript
// ─── Block 1: Class Definition & Constructor ───────────────
```

```typescript
// ─── Block 2: Protocol Method Implementation ───────────────
```

```typescript
// ─── Block 3: Utility Methods, Factories, Helpers ──────────
```

## Block 1 — Class Definition & Constructor

```typescript
// ─── Block 1: Class Definition & Constructor ───────────────
export class Capabilities<NameCapability> implements I<NameCapability>Protocol {
    constructor() {}
}
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain protocol methods ONLY.

```typescript
// ─── Block 2: Protocol Method Implementation ───────────────
export class Capabilities<NameCapability> implements I<NameCapability>Protocol {
    execute(
        input: <DomainVO>,
        output: <ResultVO>[],
    ): void {
        // domain behavior
    }
}
```

Do NOT put these in Block 2: `toString()`, `toJSON()`, `valueOf()`, `equals()`, `[Symbol.iterator]()`, `static create()`, `private helper()`.

## Block 3 — Utility Methods, Factories, and Helpers

```typescript
// ─── Block 3: Utility Methods, Factories, Helpers ──────────
export class Capabilities<NameCapability> implements I<NameCapability>Protocol {
    toString(): string {
        return 'Capabilities<NameCapability>()';
    }

    equals(other: unknown): boolean {
        return other instanceof Capabilities<NameCapability>;
    }

    static create(): Capabilities<NameCapability> {
        return new Capabilities<NameCapability>();
    }

    private resolveThreshold(input: <DomainVO>): number {
        // private helper
    }
}
```

Block 3 MUST NOT:

- define domain models (Entities, Value Objects) — that is **No Domain Definition** (ARCHITECTURE §8); consume them from Taxonomy instead.
- perform orchestration — no flow control across capabilities, no error-escalation policy (**No Orchestration**, ARCHITECTURE §8).
- duplicate technical mechanics that belong in a Utility standalone function (**DRY**, ARCHITECTURE §8).

## Method Placement Decision Rule

```text
Method / function found in a capabilities file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in capabilities)
  │
  ├─ Is it defined in the I<Name>Protocol interface?
  │   └─ YES → Block 2
  │
  ├─ Is it a utility/serialization method? (toString, toJSON, valueOf, equals)
  │   └─ YES → Block 3
  │
  ├─ Is it a Symbol method? ([Symbol.iterator], [Symbol.toPrimitive])
  │   └─ YES → Block 3
  │
  ├─ Is it a static factory method? (static create, static from, static of)
  │   └─ YES → Block 3
  │
  ├─ Is it a static method?
  │   ├─ Uses class-level state (static fields)?
  │   │   └─ YES → Block 3 (keep as private static)
  │   ├─ Tightly coupled to class semantics?
  │   │   └─ YES → Block 3 (keep as static)
  │   └─ Pure logic, no class dependency?
  │       └─ YES → EXTRACT to *_utility.ts
  │
  └─ Is it a private instance method using this?
      └─ YES → Block 3
```
