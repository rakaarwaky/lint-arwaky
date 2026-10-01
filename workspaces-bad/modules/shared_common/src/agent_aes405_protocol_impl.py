# PURPOSE: AES405 P14 — agent that implements contract protocols itself
#
# Fixture for `check_agent_single_aggregate`: an agent is the feature's
# composition root — it implements the feature aggregate and injects protocol
# seams. Implementing a protocol here makes the orchestration layer duplicate a
# capability's work, so both implementations below are violations. Each belongs
# in a `capabilities_*` file that the agent then delegates to.
from typing import Protocol


class IScannerProtocol(Protocol):
    def scan(self, files: list[str]) -> list[str]:
        ...


class IReporterProtocol(Protocol):
    def report(self) -> str:
        ...


class IScannerAggregate(Protocol):
    def execute(self, files: list[str]) -> int:
        ...


class ProtocolImplAgent(IScannerAggregate):
    def __init__(self, scanner: IScannerProtocol, reporter: IReporterProtocol):
        self._scanner = scanner
        self._reporter = reporter

    def execute(self, files: list[str]) -> int:
        return len(self._scanner.scan(files))


# AES405: belongs in `capabilities_file_scanner`, not in the agent.
class ProtocolImplAgent(IScannerProtocol):
    def scan(self, files: list[str]) -> list[str]:
        return files


# AES405: belongs in `capabilities_file_reporter`, not in the agent.
class ProtocolImplAgent(IReporterProtocol):
    def report(self) -> str:
        return ""