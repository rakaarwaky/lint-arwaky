# BAD: Contract contains implementation
class IFileSystemPort(ABC):
    @abstractmethod
    def read_file(self, path: FilePath) -> FileContent: ...


class FileAdapter(IFileSystemPort):
    def read_file(self, path: FilePath) -> FileContent:
        with open(path.value()) as f:  # BAD: implementation in contract
            return FileContent(f.read())
