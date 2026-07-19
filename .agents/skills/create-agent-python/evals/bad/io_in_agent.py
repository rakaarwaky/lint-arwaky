# BAD: I/O in agent layer
class OrphanOrchestrator:
    def execute(self, path: FilePath):
        content = open(path.value()).read()  # BAD
