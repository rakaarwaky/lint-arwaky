class <Name>Orchestrator:
    def process(self, files: list[FilePath]):
        total = len(files)  # BAD: computation
        sum_val = sum(f.size for f in files)  # BAD

total = len(files)
average = total_score / total
