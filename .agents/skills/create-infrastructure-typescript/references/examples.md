# Examples

## BAD: Infrastructure Without Port (AES404)

```typescript
export class FileCache {
    read(): string {
        // public behavior without port interface
    }
}
```

Fix:

```typescript
export class FileCache implements IFileCachePort {
    read(): string {
        // contract implementation
    }
}
```

## BAD: Business Logic in Infrastructure

```typescript
export class OrphanFileCache {
    analyze(content: FileContent): boolean {
        // BAD: domain logic
        return content.value.includes('orphan');
    }
}
```

Fix: Move analysis to capabilities.

## BAD: Utility Methods in Block 2

```typescript
export class FileCacheAdapter implements IFileReaderPort {
    constructor(private readonly _cacheDir: FilePath) {}

    toString(): string {                    // ← Block 2 position, NOT a port method
        return 'FileCacheAdapter()';
    }

    read(path: FilePath): string { ... }    // ← pushed down
}
```

Fix: Move `toString()` to Block 3.

## GOOD: Correct 3-Block with Utility Methods

```typescript
export class FileCacheAdapter implements IFileReaderPort {

    constructor(private readonly _cacheDir: FilePath) {}  // Block 1: constructor

    read(path: FilePath): string { ... }                  // Block 2: port method ONLY

    toString(): string {                                  // Block 3: utility method
        return `FileCacheAdapter(cacheDir=${this._cacheDir.value})`;
    }

    static create(): FileCacheAdapter {                   // Block 3: factory
        return new FileCacheAdapter(new FilePath('.cache'));
    }

    private resolvePath(filePath: string): string {       // Block 3: private helper
        return `${this._cacheDir.value}/${filePath}`;
    }
}
```
