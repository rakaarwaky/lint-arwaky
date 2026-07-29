class <Name>Orchestrator:
    def execute(self, path: FilePath):
        content = open(path.value()).read()  # BAD: direct I/O in agent
