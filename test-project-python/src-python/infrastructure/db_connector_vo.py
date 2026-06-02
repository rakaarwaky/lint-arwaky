# AES010 — forbidden suffix: _vo in infrastructure
# VOs belong in taxonomy, not infrastructure
class ConnectorVO:
    def __init__(self, endpoint: str):
        self.endpoint = endpoint
