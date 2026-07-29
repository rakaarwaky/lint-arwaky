# AES002 — taxonomy error without mandatory VO import
# Error must import VOs from taxonomy

class AuthError:
    def __init__(self, code: int, detail: str):
        self.code = code
        self.detail = detail
