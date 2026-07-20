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
def parse_input(content: <RawContent>VO) -> Result[<DomainVO>, <Name>ParseError]:
    # ...
    ...
```

## Rule 3: Check/analysis methods may return `list[<ResultVO>]`

```python
def check_input(source: <DomainVO>) -> list[<ResultVO>]:
    violations: list[<ResultVO>] = []
    # analysis logic
    return violations
```

## Rule 4: I/O errors belong to utility implementations (infrastructure layer removed)

Bad in capabilities:

```python
def read_input(path: <Path>VO) -> list[<ResultVO>]:
    content = open(path.value()).read()  # BAD: I/O in capabilities
    return []
```

Good:

```python
# utility_source_reader.py
class FileSystem<NameReader>(I<NameReader>Protocol):
    def read(self, path: <Path>VO) -> Result[<SourceContent>VO, <Name>ReadError]:
        try:
            raw = path.value().read_text()
        except Exception as e:
            return Err(<Name>ReadError.Io(e))
        return <SourceContent>VO.new(path, raw).map_err(<Name>ReadError.Validation)

# capabilities_<name>.py
class <NameCapability>(I<NameCapability>Protocol):
    def execute(self, source: <SourceContent>VO) -> list[<ResultVO>]:
        # pure analysis using already-read source
        return []
```
