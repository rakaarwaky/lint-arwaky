# AES403: no block markers at all.
# A capability declares Block 1 (class) -> Block 2 (protocol methods) ->
# Block 3 (dunders, factories, helpers). Each block needs its banner so the
# reader is given a map of the file. This one declares none, so
# `check_block_markers` reports "declares no block markers".
from shared.src.contract_some_protocol import SomeProtocol


class NoBlockMarkerProcessor(SomeProtocol):
    def execute(self, value: str) -> bool:
        return not value

    def __init__(self) -> None:
        self._seen: int = 0
