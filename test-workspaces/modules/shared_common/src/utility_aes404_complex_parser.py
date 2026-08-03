# AES404: utility contains forbidden type definition (class)
class ParserConfig:
    def __init__(self):
        self.max_depth = 10
        self.strict = True

def parse(input_str):
    return input_str.upper()
