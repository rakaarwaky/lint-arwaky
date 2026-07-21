# JSDoc Rules

## Purpose

JSDoc comments provide API documentation visible in IDEs and generated docs.

## Rules

1. **Every public module** — one-liner at top of file
2. **Every public class** — one-liner describing purpose
3. **Every public method** — describe purpose, parameters, return values, exceptions
4. **Explain "what" and "why"** — not "how" (code shows how)
5. **Use @param for parameters** — include type and description
6. **Use @returns for return values** — include type and description
7. **Use @throws for exceptions** — include when it's thrown

## Template

```typescript
/**
 * One-liner describing module purpose.
 */

/**
 * One-liner describing class purpose.
 */
class MyClass {
    /**
     * One-liner describing method purpose.
     * @param param1 - Description of param1
     * @param param2 - Description of param2
     * @returns Description of return value
     * @throws ErrorType - When error condition occurs
     */
    myMethod(param1: string, param2: number): boolean {
        // ...
    }
}
```

## Anti-Patterns

- ❌ Missing module docstrings → every file needs one-liner at top
- ❌ Missing parameter documentation → all parameters must be documented
- ❌ Using @ts-ignore without reason → fix root cause instead
- ❌ Over-documenting obvious code → keep concise and meaningful
- ❌ Explaining "how" instead of "what/why" → code shows how
