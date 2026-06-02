"""file_system_port — Port interface for low-level file system operations."""

from abc import ABC, abstractmethod

from ..taxonomy import (
    Count,
    FileContentVO,
    FilePath,
    PatternList,
    SuccessStatus,
    Identity,
    FileSystemError,
)

from typing import Iterator


class IFileSystemPort(ABC):
    @abstractmethod
    def walk(
        self, path: FilePath, ignored_patterns: PatternList | None = None
    ) -> Iterator[FilePath]:
        """Recursively yields file paths, respecting ignore patterns."""
        ...

    @abstractmethod
    def is_directory(self, path: FilePath) -> SuccessStatus:
        """Returns True (SuccessStatus) if the path is a directory."""
        ...

    @abstractmethod
    def is_file(self, path: FilePath) -> SuccessStatus:
        """Returns True (SuccessStatus) if the path is a file."""
        ...

    @abstractmethod
    def get_relative_path(self, path: FilePath, start: FilePath) -> FilePath:
        """Returns a relative path as FilePath."""
        ...

    @abstractmethod
    def read_text(self, path: FilePath) -> FileContentVO | FileSystemError:
        """Reads the content of a file as text."""
        ...

    @abstractmethod
    def get_line_count(self, path: FilePath) -> Count:
        """Returns the number of lines in a file."""
        ...

    @abstractmethod
    def exists(self, path: FilePath) -> SuccessStatus:
        """Returns True (SuccessStatus) if the path exists."""
        ...

    @abstractmethod
    def get_parent(self, path: FilePath) -> FilePath:
        """Returns the parent directory of the path."""
        ...

    @abstractmethod
    def write_text(
        self, path: FilePath, content: FileContentVO, mode: Identity | None = None
    ) -> SuccessStatus | FileSystemError:
        """Writes text content to a file (mode: 'w' or 'a')."""
        ...

    @abstractmethod
    def glob(self, pattern: Identity) -> Iterator[FilePath]:
        """Finds files matching a pattern."""
        ...

    @abstractmethod
    def get_cwd(self) -> FilePath:
        """Returns the current working directory."""
        ...

    @abstractmethod
    def get_basename(self, path: FilePath) -> Identity:
        """Returns the base name of the path."""
        ...

    @abstractmethod
    def path_join(self, *parts: Identity) -> FilePath:
        """Joins path components."""
        ...

    @abstractmethod
    def read_file(self, path: FilePath) -> FileContentVO | FileSystemError:
        """Reads the content of a file (alias for read_text)."""
        ...
