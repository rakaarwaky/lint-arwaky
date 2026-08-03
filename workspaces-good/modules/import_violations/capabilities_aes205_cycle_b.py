# Fixture: AES205 — circular dependency B side (contract importing capabilities).
from capabilities.processor import CapabilitiesProcessor


def process():
    proc = CapabilitiesProcessor()
    return proc
