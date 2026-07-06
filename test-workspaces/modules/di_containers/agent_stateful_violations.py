# PURPOSE: Test AES0305 — agent non-stateless, any type, infra imports, single goal
# --- Imports violating AES0305 ---
from typing import Any
import os  # infrastructure import

class AgentStatefulViolations:
    def __init__(self):
        self.state = None

    def run(self, data: Any) -> Any:  # AES0305: any type
        self.state = data  # AES0305: state assignment outside __init__
        return self._process(data)

    def _process(self, data: Any) -> Any:
        return os.path.basename(data)  # AES0305: infra import usage


class AgentSingleGoal:
    def execute(self):
        pass  # AES0305: single execution goal