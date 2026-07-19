# Primitive and VO Rules

Infrastructure public contracts should use shared VOs for domain data.

Bad:

```typescript
export interface IFileWriterPort {
    write(path: string, content: string): void;
}
```

Good:

```typescript
export interface IFileWriterPort {
    write(path: FilePath, content: FileContent): Result<void, FileWriteError>;
}
```

## Primitive Policy

| Primitive  | Rule |
| ---------- | ---- |
| `string`   | Forbidden for domain fields and public contract values. Use VO. |
| `number`   | Forbidden for domain values. Use VO. |
| `boolean`  | Allowed for technical toggles when no richer VO is needed. |

Prefer VOs for: file paths, URLs, timeouts, durations, cache keys, cache values, query results, identifiers, messages.
