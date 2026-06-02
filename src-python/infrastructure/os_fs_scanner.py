"""Implementation of IFileSystemScanner using the standard os module."""

from __future__ import annotations

from ..taxonomy import (
    Count,
    FileContentVO,
    FilePath,
    PatternList,
    SuccessStatus,
    BooleanVO,
    Identity,
    Timestamp,
    FileSystemError,
    ErrorMessage,
)

import logging
import os
from typing import Iterator

from ..contract import IFileSystemPort

logger = logging.getLogger(__name__)

try:
    from auto_linter import auto_linter_rust
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False


class OSFileSystemAdapter(IFileSystemPort):
    def __init__(self) -> None:
        self._exists_cache: dict[FilePath, BooleanVO] = {}
        self._is_file_cache: dict[FilePath, BooleanVO] = {}
        self._is_dir_cache: dict[FilePath, BooleanVO] = {}
        self._line_count_cache: dict[tuple[FilePath, Timestamp], Count] = {}

    def walk(
        self, path: FilePath, ignored_patterns: PatternList | None = None
    ) -> Iterator[FilePath]:
        path_str = str(path)
        ignored = ignored_patterns.values if ignored_patterns else []

        if os.path.isfile(path_str):
            yield path
            return

        if RUST_AVAILABLE:
            try:
                paths = auto_linter_rust.walk_files(path_str, ignored)
                for p in paths:
                    yield FilePath(value=p)
                return
            except Exception as e:
                logger.warning(f"Rust fs scanner failed: {e}. Falling back to Python os.walk.")

        for root, dirs, files in os.walk(path_str):
            # Prune ignored directories
            dirs[:] = [d for d in dirs if d not in ignored]

            for f in files:
                full_path = os.path.join(root, f)
                # Check if any part of the relative path is ignored
                rel_path = os.path.relpath(full_path, path_str)
                if any(part in ignored for part in rel_path.split(os.sep)):
                    continue
                yield FilePath(value=full_path)

    def is_directory(self, path: FilePath) -> SuccessStatus:
        if path not in self._is_dir_cache:
            self._is_dir_cache[path] = BooleanVO(value=os.path.isdir(str(path)))
        return SuccessStatus(value=self._is_dir_cache[path])

    def is_file(self, path: FilePath) -> SuccessStatus:
        if path not in self._is_file_cache:
            self._is_file_cache[path] = BooleanVO(value=os.path.isfile(str(path)))
        return SuccessStatus(value=self._is_file_cache[path])

    def get_relative_path(self, path: FilePath, start: FilePath) -> FilePath:
        return FilePath(value=os.path.relpath(str(path), str(start)).replace("\\", "/"))

    def read_text(self, path: FilePath) -> FileContentVO | FileSystemError:
        try:
            with open(str(path), "r", encoding="utf-8") as f:
                return FileContentVO(value=f.read())
        except Exception as e:
            logger.warning("Failed to read text from %s: %s", path, e, exc_info=True)
            return FileSystemError(
                path=path,
                message=ErrorMessage(value=f"Failed to read file: {e}")
            )

    def get_line_count(self, path: FilePath) -> Count:
        try:
            mtime = os.path.getmtime(str(path))
            timestamp = Timestamp(value=str(mtime))
            cache_key = (path, timestamp)
            if cache_key in self._line_count_cache:
                return self._line_count_cache[cache_key]

            with open(str(path), "r", encoding="utf-8") as f:
                count_val = sum(1 for _ in f)
                count = Count(value=count_val)
                self._line_count_cache[cache_key] = count
                return count
        except Exception:
            logger.warning("Failed to count lines in %s", path, exc_info=True)
            return Count(value=0)

    def exists(self, path: FilePath) -> SuccessStatus:
        if path not in self._exists_cache:
            self._exists_cache[path] = BooleanVO(value=os.path.exists(str(path)))
        return SuccessStatus(value=self._exists_cache[path])

    def get_parent(self, path: FilePath) -> FilePath:
        parent = os.path.dirname(str(path)) or "."
        return FilePath(value=parent)

    def invalidate_cache(self, path: FilePath | None = None) -> None:
        """Clear cache for a specific path, or all paths if path is None."""
        if path is None:
            self._exists_cache.clear()
            self._is_file_cache.clear()
            self._is_dir_cache.clear()
            self._line_count_cache.clear()
        else:
            self._exists_cache.pop(path, None)
            self._is_file_cache.pop(path, None)
            self._is_dir_cache.pop(path, None)
            keys_to_remove = [k for k in self._line_count_cache if k[0] == path]
            for k in keys_to_remove:
                self._line_count_cache.pop(k, None)

    def write_text(
        self, path: FilePath, content: FileContentVO, mode: Identity | None = None
    ) -> SuccessStatus | FileSystemError:
        try:
            mode_str = str(mode) if mode else "w"
            py_mode = "a" if mode_str in ("a", "append") else "w"
            with open(str(path), py_mode, encoding="utf-8") as f:
                f.write(str(content))
            self.invalidate_cache(path)
            return SuccessStatus(value=BooleanVO(value=True))
        except Exception as e:
            logger.warning("Failed to write text to %s: %s", path, e, exc_info=True)
            return FileSystemError(
                path=path,
                message=ErrorMessage(value=f"Failed to write file: {e}")
            )

    def glob(self, pattern: Identity) -> Iterator[FilePath]:
        import glob

        for p in glob.glob(str(pattern), recursive=True):
            yield FilePath(value=p)

    def get_cwd(self) -> FilePath:
        return FilePath(value=os.getcwd().replace("\\", "/"))

    def get_basename(self, path: FilePath) -> Identity:
        return Identity(value=os.path.basename(str(path)))

    def path_join(self, *parts: Identity) -> FilePath:
        return FilePath(value=os.path.join(*[str(p) for p in parts]).replace("\\", "/"))

    def read_file(self, path: FilePath) -> FileContentVO:
        return self.read_text(path)
