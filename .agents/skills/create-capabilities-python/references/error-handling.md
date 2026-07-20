# Error Handling Rules

Capabilities error handling must be explicit.

## Rule 1: Do not silently discard errors

Forbidden:

```python
value = result or ""
```

Forbidden:

```python
value = result or 0
```

## Rule 2: Fallible operations should return `Result` or raise

```python
def parse_manifest(content: ManifestContent) -> Result[Manifest, ManifestParseError]:
    # ...
    ...
```

## Rule 3: Check/analysis methods may return `list[LintResult]`

```python
def check_imports(source: SourceContentVO) -> list[LintResult]:
    violations: list[LintResult] = []
    # analysis logic
    return violations
```

## Rule 4: I/O errors belong to utility implementations (infrastructure layer removed)

Bad in capabilities:

```python
def check_file(path: FilePath) -> list[LintResult]:
    content = open(path.value()).read()  # BAD: I/O in capabilities
    return []
```

Good:

```python
# utility_source_reader.py
class FileSystemSourceReader(ISourceReaderProtocol):
    def read(self, path: FilePath) -> Result[SourceContentVO, SourceReadError]:
        try:
            raw = path.value().read_text()
        except Exception as e:
            return Err(SourceReadError.Io(e))
        return SourceContentVO.new(path, raw).map_err(SourceReadError.Validation)

# capabilities_import_checker.py
class ImportChecker(IImportCheckerProtocol):
    def check(self, source: SourceContentVO) -> list[LintResult]:
        # pure analysis using already-read source
        return []
```
