import os
import shutil
import sys
from unittest.mock import patch, MagicMock

from auto_linter.infrastructure.javascript_linter_adapter import _resolve_js_cmd
from auto_linter.surfaces.cli_setup_command import SetupCommandsSurface


def test_resolve_js_cmd_global_path():
    """Verify that _resolve_js_cmd uses global path if available."""
    with patch("shutil.which", return_value="/usr/local/bin/eslint"):
        cmd = _resolve_js_cmd("eslint", ["file.ts"], "/some/project")
        assert cmd == ["/usr/local/bin/eslint", "file.ts"]


def test_resolve_js_cmd_local_node_modules():
    """Verify that _resolve_js_cmd finds local node_modules/.bin."""
    def mock_exists(path):
        if "node_modules/.bin/eslint" in path:
            return True
        return False

    with patch("shutil.which", return_value=None), \
         patch("os.path.exists", side_effect=mock_exists), \
         patch("os.path.isfile", return_value=True):
        cmd = _resolve_js_cmd("eslint", ["file.ts"], "/some/project")
        assert cmd[0].endswith("node_modules/.bin/eslint")
        assert cmd[1] == "file.ts"


def test_resolve_js_cmd_parent_node_modules():
    """Verify that _resolve_js_cmd climbs up the directory hierarchy."""
    existing_paths = ["/workspace/node_modules/.bin/eslint"]

    def mock_exists(path):
        return any(path.endswith(p) for p in existing_paths)

    with patch("shutil.which", return_value=None), \
         patch("os.path.exists", side_effect=mock_exists), \
         patch("os.path.isfile", return_value=True):
        cmd = _resolve_js_cmd("eslint", ["file.ts"], "/workspace/nested/subproject")
        assert cmd[0].endswith("/workspace/node_modules/.bin/eslint")
        assert cmd[1] == "file.ts"


def test_resolve_js_cmd_npx_fallback():
    """Verify that _resolve_js_cmd falls back to npx when npx is available."""
    def mock_which(name, path=None):
        if name == "npx":
            return "/usr/bin/npx"
        return None

    with patch("shutil.which", side_effect=mock_which), \
         patch("os.path.exists", return_value=False):
        cmd = _resolve_js_cmd("eslint", ["file.ts"], "/some/project")
        assert cmd == ["npx", "eslint", "file.ts"]


def test_find_binary_setup_commands():
    """Verify that SetupCommandsSurface._find_binary uses parent directories and npx."""
    surface = SetupCommandsSurface()

    existing_paths = ["/root_project/node_modules/.bin/prettier"]

    def mock_exists(path):
        return any(path.endswith(p) for p in existing_paths)

    def mock_which(name, path=None):
        if name == "npx":
            return "/usr/bin/npx"
        return None

    with patch("shutil.which", side_effect=mock_which), \
         patch("os.path.abspath", return_value="/root_project/nested"), \
         patch("os.path.exists", side_effect=mock_exists), \
         patch("os.path.isfile", return_value=True):
        path = surface._find_binary("prettier")
        assert path.endswith("/root_project/node_modules/.bin/prettier")
