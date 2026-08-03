# AES505: orphan agent file (implements aggregate but not called by any surface)
class MyOrphanAggregate:
    pass

class OrphanOrchestrator(MyOrphanAggregate):
    def execute(self) -> str:
        return "orphan"
