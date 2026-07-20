# Error Handling Rules

Capabilities error handling must be explicit.

## Rule 1: Do not silently discard errors

Forbidden:

```typescript
const value = result ?? '';
```

Forbidden:

```typescript
const value = result || 0;
```

## Rule 2: Fallible operations should return `Result` or throw

```typescript
function parseInput(content: <RawContent>VO): Result<<DomainVO>, <Name>ParseError> {
    // ...
}
```

## Rule 3: Check/analysis methods may return `<ResultVO>[]`

```typescript
function checkInput(source: <DomainVO>): <ResultVO>[] {
    const violations: <ResultVO>[] = [];
    // analysis logic
    return violations;
}
```

## Rule 4: I/O errors belong to utility implementations (infrastructure layer removed)

Bad in capabilities:

```typescript
function readInput(path: <Path>VO): <ResultVO>[] {
    const content = fs.readFileSync(path.value(), 'utf-8'); // BAD: I/O
    return [];
}
```

Good:

```typescript
// utility_source_reader.ts
export class FileSystem<NameReader> implements I<NameReader>Protocol {
    read(path: <Path>VO): Result<<SourceContent>VO, <Name>ReadError> {
        try {
            const raw = fs.readFileSync(path.value(), 'utf-8');
            return Ok(<SourceContent>VO.new(path, raw));
        } catch (e) {
            return Err(new <Name>ReadError.Io(e));
        }
    }
}

// capabilities_<name>.ts
export class <NameCapability> implements I<NameCapability>Protocol {
    execute(source: <SourceContent>VO): <ResultVO>[] {
        // pure analysis using already-read source
        return [];
    }
}
```
