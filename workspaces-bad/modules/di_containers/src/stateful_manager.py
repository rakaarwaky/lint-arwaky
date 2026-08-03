# AES021: agent stateful manager with instance state

class StatefulManager:
    def __init__(self):
        self.state = {}

    def run(self, task):
        self.state[task] = "done"
        return self.state[task]
