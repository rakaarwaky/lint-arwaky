# AES305 violation: duplicate function block (same body as _b)
def compute_value(x: int) -> int:
    result = x + 1
    return result

def another_util(y: str) -> str:
    result = y.upper()
    return result
