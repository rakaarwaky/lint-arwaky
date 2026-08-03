# AES201: capabilities importing from surface (forbidden layer)
from ..cli_commands.surface_bad_command import SurfaceBadCommand

class ForbiddenImportChecker:
    def check(self):
        cmd = SurfaceBadCommand()
        return True
