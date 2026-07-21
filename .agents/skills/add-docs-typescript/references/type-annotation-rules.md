# Type Annotation Rules

## Purpose

Type annotations provide type safety and IDE support.

## Rules

1. **All function parameters** — must have type annotations
2. **All function return types** — must have type annotations
3. **Use interfaces** — for object shapes
4. **Use type aliases** — for unions and intersections
5. **Use generics** — for reusable types
6. **Avoid `any`** — use `unknown` if type is truly unknown
7. **Use `Record<string, unknown>`** — for dynamic object shapes

## Template

```typescript
// Interface for object shapes
interface ImportRule {
    pattern: string;
    message: string;
}

// Type alias for unions
type ValidationResult = [boolean, string];

// Function with type annotations
function validate(data: Record<string, unknown>): ValidationResult {
    // ...
}

// Generic function
function first<T>(items: T[]): T | undefined {
    return items[0];
}
```

## Anti-Patterns

- ❌ Missing type annotations → add types to all parameters and returns
- ❌ Using `any` → use `unknown` or specific type
- ❌ Using `@ts-ignore` → fix root cause instead
- ❌ Missing interface for object shapes → define interface
- ❌ Missing type alias for unions → define type alias
