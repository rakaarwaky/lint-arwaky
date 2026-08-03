# AES402: contract importing capabilities (cross-layer violation)
from capabilities.handler import CapabilitiesHandler

class ConfigAggregate:
    def load(self):
        handler = CapabilitiesHandler()
        handler.process()
