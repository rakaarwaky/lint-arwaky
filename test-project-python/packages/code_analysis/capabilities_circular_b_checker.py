# AES012 — circular import test (B imports A)
from .capabilities_circular_a_analyzer import CircularA

class CircularB:
    def check(self):
        return CircularA().analyze()
