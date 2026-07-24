# Stateless Rules (Utility — TypeScript)

## Rule 1: No Classes

Utility files must NOT contain:

- `class` definitions
- `this` keyword usage
- Class properties or instance state

Bad:

```typescript
class UtilityHelper {
  private cache: Map<string, string> = new Map();

  process(input: string): string {
    // BAD — has state
    return "";
  }
}
```

Good:

```typescript
export function process(input: string): string {
  // pure function, no state
  return input.toUpperCase();
}
```

## Rule 2: Pure Functions Only

A utility function must satisfy:

- **Deterministic:** same input → same output every time
- **No side effects:** no I/O (unless domain-agnostic + reusable), no network, no database
- **No randomness:** no `Math.random()`, no `crypto.getRandomValues()`
- **No global state:** no module-level mutable variables

Bad:

```typescript
// BAD — not deterministic
export function getTimestamp(): number {
  return Date.now();
}
```

Good:

```typescript
// GOOD — deterministic
export function normalizePath(path: string): string {
  return path.replace(/\\/g, "/").replace(/^\.\/?/, "");
}
```

## Rule 3: Domain Agnostic

Utility functions must NOT know about:

- Architecture layer names (agent, capabilities, contract, etc.)
- Business domain rules (naming conventions, import policies)
- Specific capability logic (how a checker validates)

Bad:

```typescript
// BAD — knows about architecture layers
export function isInAgentLayer(filename: string): boolean {
  return filename.startsWith("agent_");
}
```

Good:

```typescript
// GOOD — generic string operation
export function startsWithPrefix(prefix: string, filename: string): boolean {
  return filename.startsWith(prefix);
}
```

## Rule 4: Reusable Across Modules

Utility functions must be useful for multiple modules.

If a function is only used by one capability or one agent → keep as private helper in that module.

Decision Tree:

```text
Found reusable code?
  │
  ├─ Used by ≥2 modules?
  │   ├─ YES → check stateless + pure + domain-agnostic
  │   │         └─ All YES → extract to utility
  │   └─ NO → keep as private helper in the layer file
```

## I/O Exception

Utility CAN perform I/O if ALL conditions are met:

1. Stateless (no `this`, no class properties)
2. Domain-agnostic (no business knowledge)
3. Reusable across multiple modules

Good examples:

```typescript
// OK — stateless, domain-agnostic, reusable
import * as fs from "fs";
import * as path from "path";

export function walkSourceFiles(dir: string, extensions: string[]): string[] {
  // I/O is allowed here
  const files: string[] = [];
  // ... implementation
  return files;
}

export function readFileContent(filePath: string): string {
  // I/O is allowed here
  return fs.readFileSync(filePath, "utf-8");
}
```
