# PURPOSE: AES405 — agent that carries block markers beyond Block 3
#
# Fixture for `check_agent_block_markers`: the structure is Block 1 (types and
# injected deps) -> Block 2 (aggregate impl) -> Block 3 (constructors, std
# traits, helpers). Block 4 and Block 5 mean the file has outgrown that shape
# and the behaviour they hold belongs in a capability or utility.
from typing import Protocol


class IScannerProtocol(Protocol):
    def scan(self, files: list[str]) -> list[str]:
        ...


class IScannerAggregate(Protocol):
    def execute(self, files: list[str]) -> int:
        ...


# ─── Block 1: Struct Definitions ───────────────────
class ExtraBlockAgent:
    def __init__(self, scanner: IScannerProtocol):
        self._scanner = scanner


# ─── Block 2: Aggregate Trait Implementation ───────
class ExtraBlockAgent(IScannerAggregate):
    def execute(self, files: list[str]) -> int:
        return len(self._scanner.scan(files))


# ─── Block 3: Constructors, Std Traits, Helpers ────
class ExtraBlockAgent:
    def collect(self, files: list[str]) -> list[str]:
        return files


# ─── Block 4: Extra Seams ───────────────────
# AES405: no fourth block — fold this into Block 3 or move it out.
class ExtraBlockAgent:
    def normalise(self, files: list[str]) -> list[str]:
        return files


# ─── Block 5: Reporting ───────────────────
# AES405: no fifth block either.
class ExtraBlockAgent:
    def summarise(self) -> str:
        return ""