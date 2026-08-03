# Fixture: AES201 — capabilities importing agent (forbidden layer).
from agent.orchestrator import AgentOrchestrator


def process():
    agent = AgentOrchestrator()
    return agent
