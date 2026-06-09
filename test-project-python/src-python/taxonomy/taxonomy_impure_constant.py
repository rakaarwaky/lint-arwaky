# AES015: Constant Purity - _constant file has non-const declaration
import math

ACTIVE_TIMEOUT = 30
BATCH_SIZE = 100


def helper_function():
    return ACTIVE_TIMEOUT * 2


class ConfigManager:
    pass
