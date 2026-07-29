# ─── Block 1: Class Definition & Constructor ───────────────
class <NameOrchestrator>(I<NameOrchestrator>Aggregate):
    def __init__(self, analyzer: I<NameAnalyzer>Protocol):
        self._analyzer = analyzer
