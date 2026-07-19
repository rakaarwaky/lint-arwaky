from abc import ABC, abstractmethod
from shared.file_system.taxonomy_file_content_vo import FileContent
from shared.file_system.taxonomy_file_path_vo import FilePath
from shared.file_system.taxonomy_file_read_error import FileReadError


class IFileSystemPort(ABC):
    @abstractmethod
    def read_file(self, path: FilePath) -> Result[FileContent, FileReadError]: ...
