# AES404: utility with complex logic (should be pure function)
def parse_complex(input_str):
    if not input_str:
        raise ValueError("empty input")
    
    result = ""
    for ch in input_str:
        if ch.isalnum():
            result += ch
        elif ch == ' ':
            result += '_'
    
    if len(result) > 100:
        raise ValueError("too long")
    
    return result
