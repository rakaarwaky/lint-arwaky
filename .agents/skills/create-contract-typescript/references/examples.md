# Examples

## GOOD: Port Contract

```typescript
import { FileContent } from "../shared/<name-feature>/taxonomy_file_content_vo";
import { FilePath } from "../shared/<name-feature>/taxonomy_file_path_vo";
import { FileReadError } from "../shared/<name-feature>/taxonomy_file_read_error";

export interface IFileSystemProtocol {
  readFile(path: FilePath): Promise<Result<FileContent, FileReadError>>;
}
```

## GOOD: Protocol Contract

```typescript
import { <ResultVO> } from '../shared/<name-feature>/taxonomy_result_vo';
import { SourceContentVO } from '../shared/<name-feature>/taxonomy_source_vo';

export interface IImportForbiddenProtocol {
    check(source: SourceContentVO): <ResultVO>[];
}
```

## GOOD: Aggregate Contract

```typescript
import { <ResultVO> } from '../shared/<name-feature>/taxonomy_result_vo';
import { <ScanRequest>VO } from '../shared/<name-feature>/taxonomy_scan_request_vo';

export interface IImportRunnerAggregate {
    run(request: <ScanRequest>VO): <ResultVO>[];
}
```

## BAD: Contract Contains Implementation

```typescript
export interface IFileSystemProtocol {
  readFile(path: FilePath): Promise<FileContent>;
}

class FileAdapter implements IFileSystemProtocol {
  async readFile(path: FilePath): Promise<FileContent> {
    return fs.readFileSync(path.value()); // BAD: implementation in contract
  }
}
```

## BAD: Raw Primitives for Domain Values

```typescript
export interface IFileReaderProtocol {
  read(path: string): Promise<string>;
}
```
