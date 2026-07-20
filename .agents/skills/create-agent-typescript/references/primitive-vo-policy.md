# Primitive and VO Rules

Aggregate contracts should use shared VOs for domain data.

Bad:

```typescript
export interface I<NameOrchestrator>Aggregate {
    execute(files: string[]): string[];
}
```

Good:

```typescript
export interface I<NameOrchestrator>Aggregate {
    execute(request: <ScanRequest>VO): <ResultVO>[];
}
```

## Primitive Policy

| Primitive  | Rule |
| ---------- | ---- |
| `string`   | Forbidden for domain fields and public contract values. Use VO. |
| `number`   | Forbidden for domain values. Use VO. |
| `boolean`  | Allowed for semantic toggles when no richer VO is needed. |

Prefer VOs for: requests, reports, file paths, identifiers, execution results, results, policies, thresholds.
