# PURPOSE: Test AES0304 — raw literals in infrastructure
class RawLiteralAdapter:
    def __init__(self):
        self.timeout = 30
        self.host = "localhost"
        self.port = 8080
        self.debug = True

    def connect(self):
        url = "http://example.com"
        retries = 3
        return url
