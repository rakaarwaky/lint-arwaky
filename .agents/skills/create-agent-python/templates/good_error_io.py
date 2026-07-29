try:
    content = self.reader.read(file)
except Exception as e:
    results.append(<ResultVO>.from_read_error(file, e))
