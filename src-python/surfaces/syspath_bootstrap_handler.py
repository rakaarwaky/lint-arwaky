"""Auto-linter bootstrap handler: ensures the project's src directory is on sys.path.

This class is invoked first by the surface entry points to manipulate
sys.path before any other project imports.
"""

from __future__ import annotations

import os
import sys
from ..taxonomy import FilePath, BooleanVO
from ..contract import ISetupManagementProtocol


class SyspathBootstrapHandler:
    """Handler for system path manipulation during bootstrap."""

    _setup_contract: type[ISetupManagementProtocol]

    @staticmethod
    def execute() -> BooleanVO:
        """Ensure the project's src directory is on sys.path.

        Returns:
            BooleanVO: Success status.
        """
        _src_dir = FilePath(
            value=os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
        )
        return _execute_bootstrap(_src_dir)

    @staticmethod
    def get_src_dir() -> FilePath:
        """Return the resolved src directory path."""
        return FilePath(
            value=os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
        )


def _execute_bootstrap(src_dir: FilePath | None = None) -> BooleanVO:
    """Perform the sys.path bootstrap."""
    if src_dir is None:
        src_dir = SyspathBootstrapHandler.get_src_dir()

    _path_str = str(src_dir)
    if _path_str in sys.path:
        sys.path.remove(_path_str)
    sys.path.insert(0, _path_str)
    return BooleanVO(value=True)


# Auto-bootstrap on import — this module sets up sys.path as a side-effect
# so that callers can simply `import surfaces as surfaces.syspath_bootstrap_handler`
# without needing an explicit .execute() call that triggers E402.
_src_dir_str = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
if _src_dir_str not in sys.path:
    sys.path.insert(0, _src_dir_str)
