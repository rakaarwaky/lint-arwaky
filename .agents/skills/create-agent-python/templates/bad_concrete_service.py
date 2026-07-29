class <Name>Orchestrator:
    def __init__(self, analyzer: TextFormatter):  # BAD: concrete type instead of protocol
        self._analyzer = analyzer
