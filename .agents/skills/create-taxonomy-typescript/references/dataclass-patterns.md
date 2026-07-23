# Data Type Patterns

## Value Objects (`_vo.ts`)

Prefer readonly properties.

Bad:

```typescript
export class FilePath {
  constructor(public value: string) {}
}
```

Good:

```typescript
export class FilePath {
  private readonly _value: string;

  constructor(value: string) {
    if (!value.trim()) {
      throw new Error("FilePath cannot be empty");
    }
    this._value = value;
  }

  get value(): string {
    return this._value;
  }
}
```

## Composite Value Objects

Use other VOs as fields, not raw primitives.

```typescript
export class ImportRuleVO {
  constructor(
    private readonly pattern: RulePattern,
    private readonly message: RuleMessage,
  ) {}
}
```

## Entities (`_entity.ts`)

```typescript
export class SymbolEntity {
  constructor(
    private readonly _id: SymbolId,
    private readonly _name: SymbolName,
  ) {}

  get id(): SymbolId {
    return this._id;
  }
  get name(): SymbolName {
    return this._name;
  }
}
```

## Error Types (`_error.ts`)

Use TypeScript Error classes.

```typescript
export class ConfigError extends Error {
  constructor(
    private readonly _key: ConfigKey,
    private readonly _message: ErrorMessage,
  ) {
    super(`Config error for ${_key.value}: ${_message.value}`);
    this.name = "ConfigError";
  }
}
```

## Event Types (`_event.ts`)

```typescript
export class ScanCompletedEvent {
  constructor(private readonly _scanId: ScanId) {}
  get scanId(): ScanId {
    return this._scanId;
  }
}
```

## Constants (`_constant.ts`)

```typescript
export const FPS_DEFAULT: number = 24.0;
export const MIN_REVEAL_SECONDS: number = 0.5;
export const MANIFEST_FILENAME: string = "manifest.json";
```

Rules: no functions, no I/O, no external layer imports, no mutable state.
