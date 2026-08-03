# AES401 violation: using Python primitives (str, int) instead of proper value objects
class UserEntity:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
