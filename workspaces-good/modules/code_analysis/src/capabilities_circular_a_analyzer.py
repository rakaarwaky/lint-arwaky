# AES012 — circular import test (A imports B)
from .capabilities_circular_b_checker import CircularB

class CircularA:
    def analyze(self):
        return CircularB().check()
