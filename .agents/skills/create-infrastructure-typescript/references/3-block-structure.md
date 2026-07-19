# The 3-Block Structure

1. **Block 1 — Class Definition & Constructor**
2. **Block 2 — Port Interface Method Implementation**
3. **Block 3 — Utility Methods, Factories, and Private Helpers**

## Block 1 — Class Definition & Constructor

```typescript
export class FileSystemSourceReader implements IFileReaderPort {
    constructor() {}
}
```

## Block 2 — Public Contract

Block 2 is RESERVED for the domain port interface methods ONLY.

```typescript
export class FileSystemSourceReader implements IFileReaderPort {
    read(path: FilePath): Result<FileContent, FileReadError> {
        // port implementation
    }
}
```

Do NOT put `toString()`, `toJSON()`, `valueOf()`, `equals()`, `static create()`, `private helper()` in Block 2.

## Block 3 — Utility Methods, Factories, and Helpers

```typescript
export class FileSystemSourceReader implements IFileReaderPort {
    toString(): string {
        return 'FileSystemSourceReader()';
    }

    static create(): FileSystemSourceReader {
        return new FileSystemSourceReader();
    }

    private ensureParentDir(path: FilePath): Result<void, FileWriteError> {
        // private helper
    }
}
```

## Method Placement Decision Rule

```text
Method / function found in an infrastructure file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in infrastructure)
  │
  ├─ Is it defined in the I<Name>Port interface?
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
