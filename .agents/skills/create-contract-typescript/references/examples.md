# Examples

## GOOD: Port Contract

```typescript
import { FileContent } from '../file_system/taxonomy_file_content_vo';
import { FilePath } from '../file_system/taxonomy_file_path_vo';
import { FileReadError } from '../file_system/taxonomy_file_read_error';

export interface IFileSystemPort {
    readFile(path: FilePath): Promise<Result<FileContent, FileReadError>>;
}
```

## GOOD: Protocol Contract

```typescript
import { LintResult } from '../code_analysis/taxonomy_lint_result_vo';
import { SourceContentVO } from '../code_analysis/taxonomy_source_vo';

export interface IImportForbiddenProtocol {
    check(source: SourceContentVO): LintResult[];
}
```

## GOOD: Aggregate Contract

```typescript
import { LintResult } from '../code_analysis/taxonomy_lint_result_vo';
import { ImportScanRequest } from '../import_rules/taxonomy_import_scan_request_vo';

export interface IImportRunnerAggregate {
    run(request: ImportScanRequest): LintResult[];
}
```

## BAD: Contract Contains Implementation

```typescript
export interface IFileSystemPort {
    readFile(path: FilePath): Promise<FileContent>;
}

class FileAdapter implements IFileSystemPort {
    async readFile(path: FilePath): Promise<FileContent> {
        return fs.readFileSync(path.value()); // BAD: implementation in contract
    }
}
```

## BAD: Raw Primitives for Domain Values

```typescript
export interface IFileReaderPort {
    read(path: string): Promise<string>;
}
```
