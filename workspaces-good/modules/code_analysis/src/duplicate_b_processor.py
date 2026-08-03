class DuplicateBProcessor:
    def run(self, items):
        result = []
        for item in items:
            if item is None:
                continue
            value = item.strip()
            if len(value) == 0:
                continue
            normalized = value.lower()
            if normalized in self.seen:
                continue
            self.seen.add(normalized)
            result.append(normalized)
        return result
