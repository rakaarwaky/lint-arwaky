# BAD: Computation in agent layer
class OrphanOrchestrator:
    def process(self, files: list[FilePath]):
        total = len(files)  # BAD: computation
        sum_val = sum(f.size for f in files)  # BAD
