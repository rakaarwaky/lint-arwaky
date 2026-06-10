# PURPOSE: Test AES012 — circular import A
from code_analysis.capabilities_circular_b_processor import ClassB

class ClassA:
    def __init__(self):
        self.b = ClassB()
