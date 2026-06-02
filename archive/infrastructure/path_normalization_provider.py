import os


from ..contract import IPathNormalizationPort
from ..taxonomy import FilePath


class PathNormalizationProvider(IPathNormalizationPort):
    """Implementation of path normalization services for infrastructure."""

    def normalize_path(self, path: FilePath) -> FilePath:
        """Normalize path: fix slashes, resolve phantom roots.

        Reads PHANTOM_ROOT and PROJECT_ROOT from environment.
        Defaults to current working directory for PROJECT_ROOT.
        """
        path_str = str(path)
        if not path_str:
            return path

        # 1. Normalize slashes and collapse separators
        path_str = path_str.replace("\\\\", "/")
        path_str = os.path.normpath(path_str).replace("\\\\", "/")

        # 2. Handle phantom roots - only apply when path does NOT already exist
        if not os.path.exists(path_str):
            home = os.path.expanduser("~")
            phantom_root = os.environ.get("PHANTOM_ROOT", home).replace("\\\\", "/")
            actual_root = os.environ.get("PROJECT_ROOT", os.getcwd()).replace(
                "\\\\", "/"
            )

            if phantom_root and path_str.startswith(phantom_root):
                suffix = path_str[len(phantom_root) :]
                if suffix.startswith("/"):
                    path_str = actual_root + suffix
                else:
                    path_str = actual_root + "/" + suffix

        # 3. Handle src/ and src-* pathing only if it's NOT explicitly relative or absolute
        if path_str.startswith("src/") or path_str.startswith("src-"):
            if not os.path.exists(path_str):
                project_root = os.environ.get("PROJECT_ROOT")
                if project_root:
                    candidate = os.path.join(project_root, path_str)
                    if os.path.exists(candidate):
                        return FilePath(
                            value=os.path.abspath(candidate).replace("\\\\", "/")
                        )

        return FilePath(value=path_str)

    def resolve_infrastructure_path(
        self, path: FilePath, context_path: FilePath | None = None
    ) -> FilePath:
        """Unified path resolution for infrastructure adapters.

        1. Normalizes the path using normalize_path.
        2. If relative, tries to resolve against context_path.
        3. Falls back to absolute path.
        """
        path_str = str(path)
        norm_path_vo = self.normalize_path(path)
        norm_path = str(norm_path_vo)

        if norm_path and os.path.isabs(norm_path) and os.path.exists(norm_path):
            return norm_path_vo

        if context_path:
            ctx_str = str(context_path)
            base_dir = (
                os.path.dirname(os.path.abspath(ctx_str))
                if os.path.isfile(ctx_str)
                else os.path.abspath(ctx_str)
            )
            possible = os.path.join(base_dir, path_str)
            if os.path.exists(possible):
                return FilePath(value=os.path.abspath(possible).replace("\\\\", "/"))

        abs_path = os.path.abspath(path_str).replace("\\\\", "/")
        if os.path.exists(abs_path):
            return FilePath(value=abs_path)

        return FilePath(value=abs_path)
