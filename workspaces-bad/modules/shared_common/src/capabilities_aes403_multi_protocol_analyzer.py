# AES403 multi-protocol: a single capability inherits two protocol ABCs.
class MultiProtocolAnalyzer(IAlphaProtocol, IBetaProtocol):
    def execute(self):
        return 42
