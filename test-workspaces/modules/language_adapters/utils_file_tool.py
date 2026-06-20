# AES011: _tool not in infrastructure suffix list
# AES005: too short
# AES009: no class
# AES014: hardcoded path
def read_file(path):
    with open(path) as f:  # noqa: SIM115
        return f.read()
