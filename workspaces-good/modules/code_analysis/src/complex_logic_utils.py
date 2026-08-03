# AES011: _utils not in capability suffix list
# AES005: too short
# AES009: no class
def flatten(items):
    result = []
    for i in items:
        if isinstance(i, list):
            result.extend(flatten(i))
        else:
            result.append(i)
    return result
