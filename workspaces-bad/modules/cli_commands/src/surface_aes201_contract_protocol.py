# AES201: surface importing contract(protocol) (forbidden layer)
from contract.calculator_protocol import ICalculatorProtocol

class SurfaceContractCommand:
    def execute(self) -> str:
        return "forbidden"
