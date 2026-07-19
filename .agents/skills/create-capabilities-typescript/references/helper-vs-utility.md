# Helper vs Utility Decision

The boundary is not only about `this`.

The real question is:

> Does this function know about specific business/domain rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this class, or by multiple modules?

## Keep as Private Helper in Block 3

Keep the function inside the capabilities file if ANY of these is true:

1. It contains business/domain rules.
2. It accesses `this.field` or instance state.
3. It is tightly coupled to this capability only.
4. It is a factory method such as `static create()` or `static from()`.
5. It is stateless but only used by this one class and is domain-specific.

Example:

```typescript
class ContractRoleChecker {
    private resolveScope(scope: string): [string, string[]] {
        // Domain-specific parsing logic.
        // Even without `this`, this can remain a private helper
        // if only this checker uses it.
    }
}
```

## Extract to Utility (`*_utility.ts`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `this`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Reusable: useful for multiple capabilities/infrastructures/modules.

Example:

```typescript
// shared/code_analysis/taxonomy_string_utility.ts
export function matchWholeToken(haystack: string, needle: string): boolean {
    // generic token matching
}
```

## I/O Blocker

A function may be stateless, but if it performs I/O, it MUST NOT become a taxonomy utility.

It also MUST NOT stay in capabilities.

```typescript
// BAD in capabilities layer
function readConfig(filePath: string): string | null {
    return fs.readFileSync(filePath, 'utf-8'); // I/O
}
```

Rule:

```text
Stateless + I/O = infrastructure/port implementation
NOT taxonomy utility
NOT capabilities layer
```

## Decision Tree

```text
Found reusable code in capabilities?
  │
  ├─ Does it know business/domain rules?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need this or class state?
  │   └─ YES → keep as helper/method in Block 3
  │
  ├─ Does it perform I/O or side effects?
  │   └─ YES → move to infrastructure/port implementation
  │
  └─ Is it stateless, pure, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility
```
