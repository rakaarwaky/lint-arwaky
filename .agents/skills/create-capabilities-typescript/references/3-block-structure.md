# The 3-Block Structure

Every implementation file MUST follow this order **within the class body**:

1. **Block 1 — Class Definition & Constructor**
2. **Block 2 — Domain Protocol Method Implementation**
3. **Block 3 — Utility Methods, Factories, and Private Helpers**

## Block 1 — Class Definition & Constructor

```typescript
export class ArchLineChecker implements ILineCheckerProtocol {
    constructor() {}
}
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain protocol methods ONLY.

```typescript
export class ArchLineChecker implements ILineCheckerProtocol {
    checkLineCounts(
        file: FilePath,
        definition: LayerDefinition | null,
        source: SourceContentVO,
        violations: LintResult[],
    ): void {
        // domain behavior
    }
}
```

Do NOT put these in Block 2: `toString()`, `toJSON()`, `valueOf()`, `equals()`, `[Symbol.iterator]()`, `static create()`, `private helper()`.

## Block 3 — Utility Methods, Factories, and Helpers

```typescript
export class ArchLineChecker implements ILineCheckerProtocol {
    toString(): string {
        return 'ArchLineChecker()';
    }

    equals(other: unknown): boolean {
        return other instanceof ArchLineChecker;
    }

    static create(): ArchLineChecker {
        return new ArchLineChecker();
    }

    private resolveThreshold(layer: string): number {
        // private helper
    }
}

```

Block 3 MUST NOT:

- define domain models (Entities, Value Objects) — that is **No Domain Definition** (ARCHITECTURE §8); consume them from Taxonomy instead.
- perform orchestration — no flow control across capabilities, no error-escalation policy (**No Orchestration**, ARCHITECTURE §8).
- duplicate technical mechanics that belong in a Utility standalone function (**DRY**, ARCHITECTURE §8).
```

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
