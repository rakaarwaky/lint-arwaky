# AES006 — primitive usage in taxonomy entity
# Taxonomy entity should use VOs, not raw primitives

class RawPersonEntity:
    def __init__(self, name: str, age: int, email: str):
        self.name = name
        self.age = age
        self.email = email
