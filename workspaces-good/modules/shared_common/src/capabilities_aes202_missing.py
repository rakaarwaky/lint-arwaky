# AES202: capabilities missing mandatory taxonomy import - only imports from agent
from ..di_containers.agent_logic import AgentLogic

class MandatoryMissingChecker:
    def check(self):
        logic = AgentLogic()
        return True
