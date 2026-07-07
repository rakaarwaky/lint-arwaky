class DuplicateAProcessor:
    def process(self, data):
        result = []
        for item in data:
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
