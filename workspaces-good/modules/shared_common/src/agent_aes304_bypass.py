# AES304: bypass annotation - type: ignore and pass patterns
from ..taxonomy.domain_model_base import DomainModelBase

class BypassEntity:
    def unsafe_method(self):
        # type: ignore
        result = None  # noqa
        assert True  # bypass pattern
        return result
