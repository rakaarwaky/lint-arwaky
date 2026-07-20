# BAD: Data class defined in capabilities layer (AES201)
class <NameResult>:
    is_valid: bool
    reason: str

class Capabilities<NameCapability>:
    def analyze(self) -> <NameResult>:
        return <NameResult>(is_valid=True, reason="")
