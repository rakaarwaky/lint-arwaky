# AES201 — forbidden import test
# Surface command should NOT import from infrastructure
import infrastructure

class ForbiddenImportCommand:
    def execute(self) -> str:
        return "forbidden"
