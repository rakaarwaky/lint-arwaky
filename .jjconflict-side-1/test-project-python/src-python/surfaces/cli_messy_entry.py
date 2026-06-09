# AES005: too short
# AES009: no class
# AES014: noqa
def messy_start():  # noqa: C901
    import sys
    if len(sys.argv) > 1:
        print(sys.argv[1])
