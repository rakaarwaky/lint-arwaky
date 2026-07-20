# The 3-Block Structure

Every implementation file MUST follow this order with mandatory block markers:

1. **Block 1 — Class Definition & Constructor**
2. **Block 2 — Aggregate Interface Method Implementation**
3. **Block 3 — Utility Methods, Factories, and Private Helpers**

Each block MUST be preceded by a block marker comment:

```typescript
// ─── Block 1: Class Definition & Constructor ───────────────
```

```typescript
// ─── Block 2: Aggregate Method Implementation ──────────────
```

```typescript
// ─── Block 3: Utility Methods, Factories, Helpers ──────────
```

## Block 1 — Class Definition & Constructor

```typescript
// ─── Block 1: Class Definition & Constructor ───────────────
export class <NameOrchestrator> implements I<NameOrchestrator>Aggregate {
    constructor(private readonly analyzer: I<NameAnalyzer>Protocol) {}
}
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain aggregate interface methods ONLY.

```typescript
// ─── Block 2: Aggregate Method Implementation ──────────────
export class <NameOrchestrator> implements I<NameOrchestrator>Aggregate {
    execute(request: <ScanRequest>VO): <ResultVO>[] {
        // orchestration only
    }
}
```

Do NOT put `toString()`, `toJSON()`, `valueOf()`, `equals()`, `static create()`, `private helper()` in Block 2.

## Block 3 — Utility Methods, Factories, and Helpers

```typescript
// ─── Block 3: Utility Methods, Factories, Helpers ──────────
export class <NameOrchestrator> implements I<NameOrchestrator>Aggregate {
    toString(): string {
        return '<NameOrchestrator>()';
    }

    static create(): <NameOrchestrator> {
        return new <NameOrchestrator>(new Capabilities<NameCapability>());
    }

    private shouldSkipFile(file: FilePath): boolean {
        // private helper
    }
}
```

## Method Placement Decision Rule

```text
Method / function found in an agent file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in agent)
  │
  ├─ Is it defined in the I<Name>Aggregate interface?
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
