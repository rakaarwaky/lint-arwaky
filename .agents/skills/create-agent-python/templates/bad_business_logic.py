class <Name>Orchestrator:
    def evaluate(self, content: FileContent) -> bool:
        return "forbidden-marker" in content.value  # BAD: business rule
