# Fixture: AES204 — dummy function to suppress unused import warning.
from taxonomy.vo import UserVO


def process():
    x = 42
    print(f"value: {x}")


def _use_user_vo(u: UserVO):
    pass
