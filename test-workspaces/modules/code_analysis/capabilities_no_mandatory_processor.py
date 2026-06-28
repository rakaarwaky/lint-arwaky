# AES202 — missing mandatory import test
# Capabilities files MUST import taxonomy and contract(protocol)
# This file imports NEITHER — triggering AES202 violations


class CapabilitiesNoMandatoryProcessor:
    def process(self) -> str:
        return "no mandatory imports"
