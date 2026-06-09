# AES036: Capability Bottleneck - all dispatch routes to single capability
class CapabilitiesBadRouter:
    def handle(self, action: str):
        if action == "create":
            self._create()
        elif action == "update":
            self._update()
        elif action == "delete":
            self._delete()
        elif action == "read":
            self._read()

    def _create(self): pass
    def _update(self): pass
    def _delete(self): pass
    def _read(self): pass
