# PURPOSE: AES405 P21 — agent that declares abstract methods
#
# Fixture for `check_agent_abstract_method`: an agent delegates to contracts
# it consumes, so it must not declare new abstract methods of its own.
from abc import ABC, abstractmethod


class AbstractAgent:
    def __init__(self, scanner, reporter):
        self._scanner = scanner
        self._reporter = reporter

    # AES405: abstract methods belong on the feature's protocol ABC,
    # not on the agent that implements it.
    @abstractmethod
    def audit(self, request):
        ...

    def execute(self, request):
        return self._scanner.scan(request)
