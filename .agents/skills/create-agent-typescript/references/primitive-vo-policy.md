# Primitive and VO Rules

Aggregate contracts should use shared VOs for domain data.

Bad:

```typescript
export interface IOrphanOrchestratorAggregate {
    execute(files: string[]): string[];
}
```

Good:

```typescript
export interface IOrphanOrchestratorAggregate {
    execute(request: ScanRequest): LintResult[];
}
```

## Primitive Policy

| Primitive  | Rule |
| ---------- | ---- |
| `string`   | Forbidden for domain fields and public contract values. Use VO. |
| `number`   | Forbidden for domain values. Use VO. |
| `boolean`  | Allowed for semantic toggles when no richer VO is needed. |

Prefer VOs for: requests, reports, file paths, identifiers, execution results, violations, policies, thresholds.
