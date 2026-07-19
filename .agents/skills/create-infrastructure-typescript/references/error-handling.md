# Error Handling Rules

## Rule 1: Do not silently discard errors

Forbidden:

```typescript
const content = fs.readFileSync(path.value(), 'utf-8') ?? '';
```

## Rule 2: Fallible port methods should return `Result` or throw

```typescript
read(path: FilePath): Result<FileContent, FileReadError>;
```

## Rule 3: Use descriptive error types

```typescript
export class FileReadError extends Error {
    constructor(
        public readonly path: FilePath,
        public readonly cause: Error,
    ) {
        super(`Failed to read ${path.value()}: ${cause.message}`);
    }
}
```

## Rule 4: Infrastructure should not produce lint results directly

Infrastructure should return data, errors, or VOs. Lint violations belong to capabilities.

Bad:

```typescript
read(path: FilePath): LintResult[] { // BAD }
```

Good:

```typescript
read(path: FilePath): Result<FileContent, FileReadError> { // OK }
```
