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
function parseManifest(content: ManifestContent): Result<Manifest, ManifestParseError> {
    // ...
}
```

## Rule 3: Check/analysis methods may return `LintResult[]`

```typescript
function checkImports(source: SourceContentVO): LintResult[] {
    const violations: LintResult[] = [];
    // analysis logic
    return violations;
}
```

## Rule 4: I/O errors belong to infrastructure/port implementations

Bad in capabilities:

```typescript
function checkFile(path: FilePath): LintResult[] {
    const content = fs.readFileSync(path.value(), 'utf-8'); // BAD: I/O
    return [];
}
```

Good:

```typescript
// infrastructure_source_reader.ts
export class FileSystemSourceReader implements ISourceReaderPort {
    read(path: FilePath): Result<SourceContentVO, SourceReadError> {
        try {
            const raw = fs.readFileSync(path.value(), 'utf-8');
            return Ok(SourceContentVO.new(path, raw));
        } catch (e) {
            return Err(new SourceReadError.Io(e));
        }
    }
}

// capabilities_import_checker.ts
export class ImportChecker implements IImportCheckerProtocol {
    check(source: SourceContentVO): LintResult[] {
        // pure analysis using already-read source
        return [];
    }
}
```
