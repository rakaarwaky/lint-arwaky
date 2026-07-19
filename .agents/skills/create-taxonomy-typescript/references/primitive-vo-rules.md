# Primitive-to-VO Rules

## General Rule

Domain data MUST use VOs, not raw primitives.

Bad:

```typescript
export interface LintResult {
    filePath: string;
    line: number;
    severity: string;
}
```

Good:

```typescript
export interface LintResult {
    filePath: FilePath;
    line: LineNumber;
    severity: Severity;
}
```

## Primitive Policy

| Primitive  | Rule                                                                                |
| ---------- | ----------------------------------------------------------------------------------- |
| `string`   | Forbidden for domain fields and public contract return values. Use VO.              |
| `number`   | Forbidden. Use domain VO.                                                           |
| `boolean`  | Allowed for semantic toggles when no richer VO is needed.                           |

Prefer VOs for: file paths, symbol names, messages, line numbers, column numbers, severity, durations, counts, thresholds, identifiers.

## VO Construction Rules

VOs MUST validate on construction when the domain has invariants.

```typescript
export class LineNumber {
    private readonly _value: number;

    constructor(value: number) {
        if (value === 0) {
            throw new Error('LineNumber must be positive');
        }
        this._value = value;
    }

    get value(): number { return this._value; }
}
```

## Optional and Collection Primitives

Bad:

```typescript
export interface RuleSet {
    patterns: string[];
    description: string | null;
}
```

Good:

```typescript
export interface RuleSet {
    patterns: PatternList;
    description: RuleDescription | null;
}
```
