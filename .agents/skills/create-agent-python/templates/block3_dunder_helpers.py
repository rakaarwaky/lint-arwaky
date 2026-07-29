# ─── Block 3: Dunder Methods, Factories, Helpers ───────────
    def __repr__(self) -> str:
        return "<NameOrchestrator>()"

    @classmethod
    def create_default(cls) -> "<NameOrchestrator>":
        return cls(analyzer=Capabilities<NameCapability>())

    def _should_skip_file(self, file: FilePath) -> bool:
        # private helper
        ...
