# AES007: capabilities directly imports from a contract submodule instead of barrel
# Violation: direct contract sub-module import

use crate::contract::dummy_port::IDummyPort

from contract.dummy_port import IDummyPort


class DirectContractProcessor:
    def process(self, data: IDummyPort) -> str:
        return str(data)
