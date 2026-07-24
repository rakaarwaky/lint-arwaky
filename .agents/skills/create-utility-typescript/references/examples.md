# Utility Examples (BAD / GOOD — TypeScript)

## Bad: Class in Utility

```typescript
// BAD — utility must not contain class definitions
class PathNormalizer {
  private separator: string;

  constructor(separator: string) {
    this.separator = separator;
  }

  normalize(path: string): string {
    return path.replace(/\//g, this.separator);
  }
}
```

## Good: Stateless Exported Function

```typescript
// GOOD — utility contains only exported functions
export function normalizePathSeparator(path: string, separator: string): string {
  return path.replace(/\//g, separator);
}
```

## Bad: Business Logic in Utility

```typescript
// BAD — utility must not know about business rules
export function isValidImport(importPath: string): boolean {
  // This knows about AES architecture — NOT domain-agnostic
  return importPath.startsWith("shared.") || importPath.startsWith("crate.common.");
}
```

## Good: Generic String Operation

```typescript
// GOOD — generic, domain-agnostic operation
export function startsWithModulePrefix(path: string, prefix: string): boolean {
  return path.startsWith(prefix);
}
```

## Bad: Mutable State

```typescript
// BAD — utility must not have side effects or global state
const cache: Map<string, string> = new Map();

export function cachedProcess(input: string): string {
  // BAD — global state mutation
  const cached = cache.get(input);
  if (cached) return cached;
  const result = input.toUpperCase();
  cache.set(input, result);
  return result;
}
```

## Good: Pure Function

```typescript
// GOOD — pure function, deterministic output
export function toUppercase(input: string): string {
  return input.toUpperCase();
}
```

## Bad: Single-Module Dependency

```typescript
// BAD — if only one capability uses this, keep as private helper
export function checkImportForAes204(importPath: string): boolean {
  // This knows about AES204 rule — domain-specific
  return importPath.includes("AES204");
}
```

## Good: Reusable Generic Function

```typescript
// GOOD — useful for any module that needs keyword checking
export function containsKeyword(text: string, keyword: string): boolean {
  return text.includes(keyword);
}
```

## I/O Example (Allowed)

```typescript
// OK — stateless, domain-agnostic, reusable across modules
import * as fs from "fs";
import * as path from "path";

export function readFileContent(filePath: string): string {
  /** Read file content — stateless, domain-agnostic, reusable. */
  return fs.readFileSync(filePath, "utf-8");
}

export function walkDirectory(dir: string, extensions: string[]): string[] {
  /** Walk directory — stateless, domain-agnostic, reusable. */
  const files: string[] = [];
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isFile()) {
      const ext = path.extname(entry.name);
      if (extensions.includes(ext)) {
        files.push(fullPath);
      }
    }
  }
  return files;
}
```
