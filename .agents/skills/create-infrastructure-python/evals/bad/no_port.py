# BAD: No port ABC inheritance (AES404)
class FileCache:
    def read(self):
        # public behavior without port ABC
        pass
