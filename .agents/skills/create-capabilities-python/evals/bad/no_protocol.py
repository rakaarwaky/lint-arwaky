# BAD: No protocol ABC inheritance (AES403)
class <NameComposer>:
    def compose_frame(self):
        # public behavior without protocol ABC
        pass
