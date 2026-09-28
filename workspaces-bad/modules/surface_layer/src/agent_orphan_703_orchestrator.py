"""AES703 trigger: surface folder with an agent orchestrator."""


class CliOrchestrator:
    """Agent that should live in a feature folder, not in a surface folder."""

    def run(self) -> str:
        return "ok"
