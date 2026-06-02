# AES021 — agent role violation: orchestrator with state
# AES002 — missing mandatory imports (no taxonomy, no contract IO)
# AES003 — wrong name (only 1 word)


class StatefulOrchestrator:
    def __init__(self):
        self.state = {}
        self.executed = []
        self.cache = {}

    def execute(self, task):
        self.executed.append(task)
        return self.state.get(task, None)

    def cached_result(self, key):
        if key in self.state:
            return self.state[key]
        return None
