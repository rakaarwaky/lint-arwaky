# BAD: I/O in capabilities layer (AES404)
class MyCapability:
    def process(self):
        with open("file.txt") as f:  # FORBIDDEN
            content = f.read()
