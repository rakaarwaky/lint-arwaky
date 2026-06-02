# AES001 + AES002 + AES015 — layer violations
# Surface should import agent+taxonomy+contract(io), NOT infrastructure
from infrastructure.utils import do_thing  # AES001: wrong import

# AES015: imported but unused

class BadSurface:
    @staticmethod
    def handle():
        return do_thing()
