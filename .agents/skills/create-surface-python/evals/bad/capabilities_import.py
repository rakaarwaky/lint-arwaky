# BAD: Smart surface imports capabilities directly
from capabilities_my_checker import MyChecker


class CheckCommand:
    def __init__(self):
        self._checker = MyChecker()
