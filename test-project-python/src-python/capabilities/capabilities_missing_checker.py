# AES037: Capability Method - referenced method doesn't exist
# A dispatch catalog referencing non-existent capability method


class CapabilitiesMissingMethodChecker:
    def dispatch(self, action: str):
        # This method is referenced in routing but doesn't exist
        pass
