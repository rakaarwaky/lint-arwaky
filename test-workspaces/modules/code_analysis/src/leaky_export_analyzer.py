# AES013: non-barrel file with __all__ export list

__all__ = ["LeakyClass"]


class LeakyClass:
    def analyze(self) -> str:
        return "leaked"
