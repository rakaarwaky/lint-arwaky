# AES032: Agent Role - container/registry/mixin with domain logic
from taxonomy.auth_error import AuthError


class AgentBadContainer:
    def process_payment(self, amount: float) -> bool:
        if amount > 1000:
            raise AuthError("Exceeds limit")
        return True
