# AES006 — primitive usage in taxonomy error
# Error should use VOs, not raw primitives
class PaymentError:
    def __init__(self, code: int, message: str):
        self.code = code
        self.message = message
