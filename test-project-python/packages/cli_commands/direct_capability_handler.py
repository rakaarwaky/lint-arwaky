# AES022: surface bypasses agent by importing capabilities directly

use crate::capabilities::some_processor::SomeProcessor


class DirectCapabilityHandler:
    def handle(self) -> str:
        return "bypass"
