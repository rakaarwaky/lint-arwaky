// BAD: Contract contains implementation
export interface IFileSystemPort {
    readFile(path: FilePath): Promise<FileContent>;
}

class FileAdapter implements IFileSystemPort {
    async readFile(path: FilePath): Promise<FileContent> {
        return fs.readFileSync(path.value()); // BAD: implementation in contract
    }
}
