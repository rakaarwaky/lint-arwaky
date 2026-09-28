"""AES701 trigger: a capabilities file parked in the shared folder."""


class SharedStrayCapability:
    """Capability logic that belongs in a feature folder, not in shared."""

    def parse(self, raw: str) -> str:
        return raw.strip()
