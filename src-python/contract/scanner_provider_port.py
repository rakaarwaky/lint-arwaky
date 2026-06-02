"""scanner_provider_port — Interface for file system scanning and change detection."""

from abc import ABC, abstractmethod

from ..taxonomy import DirectoryPath, FilePathList, FileSystemError


class IScannerProviderPort(ABC):
    """Port for scanning directories and detecting changed files."""

    @abstractmethod
    def scan_directory(self, path: DirectoryPath) -> FilePathList | FileSystemError:
        """Return list of changed files within the directory according to the scanner's policy."""
        ...

    @abstractmethod
    def get_ignored_files(self) -> FilePathList:
        """Return list of files ignored by the scanner."""
        ...
