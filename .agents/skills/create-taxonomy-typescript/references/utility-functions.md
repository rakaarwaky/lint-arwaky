# Utility Functions (`_utility.ts`)

## The Ultimate Boundary

A function belongs in `*_utility.ts` ONLY if ALL of these are true:

1. Stateless: no `this`, no class field access.
2. Domain-agnostic: does not know business rules.
3. Multi-consumer reusable: useful for multiple modules/layers.

I/O is allowed in utility functions (e.g., `walk_source_files`, `default_ignored_paths`).

## Good Utility Example

```typescript
export function matchWholeToken(haystack: string, needle: string): boolean {
  if (!needle) return false;
  const pattern = new RegExp(`(?<!\\w)${escapeRegex(needle)}(?!\\w)`);
  return pattern.test(haystack);
}

function escapeRegex(str: string): string {
  return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
```

## Bad Utility: Domain Knowledge

```typescript
export function getTargetLayerFromSuffix(suffix: string): string {
  switch (suffix) {
    case "protocol":
      return "capabilities";
    default:
      return "unknown";
  }
}
```

This belongs in capabilities as a private helper.

## Bad Utility: Single Consumer Only

```typescript
export function formatImportViolation(rule: ImportRuleVO): string {
  return `Import rule violation: ${rule.pattern.value}`;
}
```

If only one capability uses it, keep it as a private helper.
