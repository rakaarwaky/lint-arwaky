import tempfile
from pathlib import Path

from auto_linter.infrastructure.os_fs_scanner import OSFileSystemAdapter
from auto_linter.taxonomy import FilePath, PatternList


def test_fs_scanner_walk():
    scanner = OSFileSystemAdapter()

    with tempfile.TemporaryDirectory() as tmpdir:
        tmp_path = Path(tmpdir)
        (tmp_path / "src").mkdir()
        (tmp_path / "src" / "main.py").touch()
        (tmp_path / "src" / "utils.py").touch()
        (tmp_path / "tests").mkdir()
        (tmp_path / "tests" / "test_main.py").touch()
        (tmp_path / "ignored_dir").mkdir()
        (tmp_path / "ignored_dir" / "ignored.py").touch()

        # Check walk without ignores
        files = list(scanner.walk(FilePath(value=str(tmp_path))))
        basenames = {Path(f.value).name for f in files}
        assert "main.py" in basenames
        assert "utils.py" in basenames
        assert "test_main.py" in basenames
        assert "ignored.py" in basenames

        # Check walk with ignores
        ignores = PatternList(values=["ignored_dir", "tests"])
        files_with_ignores = list(scanner.walk(FilePath(value=str(tmp_path)), ignores))
        basenames_with_ignores = {Path(f.value).name for f in files_with_ignores}
        assert "main.py" in basenames_with_ignores
        assert "utils.py" in basenames_with_ignores
        assert "test_main.py" not in basenames_with_ignores
        assert "ignored.py" not in basenames_with_ignores
