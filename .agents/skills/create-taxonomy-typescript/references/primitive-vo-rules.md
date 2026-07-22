# Primitive-to-VO Rules

## General Rule

Domain data MUST use VOs, not raw primitives.

Bad:

```typescript
export interface <ResultVO> {
    target: string;
    position: number;
    level: string;
}
```

Good:

```typescript
export interface <ResultVO> {
    target: <Target>VO;
    position: <LineNumber>VO;
    level: <Severity>VO;
}
```

## Primitive Policy

| Primitive | Rule                                                                   |
| --------- | ---------------------------------------------------------------------- |
| `string`  | Forbidden for domain fields and public contract return values. Use VO. |
| `number`  | Forbidden. Use domain VO.                                              |
| `boolean` | Allowed for semantic toggles when no richer VO is needed.              |

Prefer VOs for: file paths, symbol names, messages, line numbers, column numbers, severity, durations, counts, thresholds, identifiers.

## VO Construction Rules

VOs MUST validate on construction when the domain has invariants.

```typescript
export class <LineNumber>VO {
    private readonly _value: number;

    constructor(value: number) {
        if (value === 0) {
            throw new Error('<LineNumber>VO must be positive');
        }
        this._value = value;
    }

    get value(): number { return this._value; }
}
```

## Optional and Collection Primitives

Bad:

```typescript
export interface <RuleSet>VO {
    patterns: string[];
    description: string | null;
}
```

Good:

```typescript
export interface <RuleSet>VO {
    patterns: <PatternList>VO;
    description: <RuleDescription>VO | null;
}
```
