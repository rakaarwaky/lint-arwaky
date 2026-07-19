# BAD: No protocol ABC inheritance (AES403)
class FrameComposer:
    def compose_frame(self):
        # public behavior without protocol ABC
        pass
