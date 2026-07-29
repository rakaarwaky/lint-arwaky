# BAD: I/O in capabilities layer (AES404)
class <NameCapability>:
    def process(self) -> None:
        content = open("file.txt").read()  # FORBIDDEN
