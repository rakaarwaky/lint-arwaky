# Utility Functions (`_utility.ts`)

## The Ultimate Boundary

A function belongs in `*_utility.ts` ONLY if ALL of these are true:

1. Stateless: no `this`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no randomness, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Multi-consumer reusable: useful for multiple modules/layers.

## Good Utility Example

```typescript
export function matchWholeToken(haystack: string, needle: string): boolean {
    if (!needle) return false;
    const pattern = new RegExp(`(?<!\\w)${escapeRegex(needle)}(?!\\w)`);
    return pattern.test(haystack);
}

function escapeRegex(str: string): string {
    return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
```

## Bad Utility: Domain Knowledge

```typescript
export function getTargetLayerFromSuffix(suffix: string): string {
    switch (suffix) {
        case 'port': return 'infrastructure';
        case 'protocol': return 'capabilities';
        default: return 'unknown';
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
