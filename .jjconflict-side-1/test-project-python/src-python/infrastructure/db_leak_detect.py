# AES011: _detect not in infrastructure suffix list
# AES005: too short
# AES009: no class
# AES014: eval usage
def detect_leaks(source):
    return eval(f"len({source})")  # B307
