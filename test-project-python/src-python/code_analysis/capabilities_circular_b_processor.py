# PURPOSE: Test AES012 — circular import B
from code_analysis.capabilities_circular_a_processor import ClassA

class ClassB:
    def __init__(self):
        self.a = ClassA()
