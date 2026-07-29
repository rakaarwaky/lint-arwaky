# Duplicated processing logic for test workspace
# This file contains duplicate code to trigger AES305 detection


def process_data(input_text):
    trimmed = input_text.strip()
    normalized = trimmed.lower()
    result = ''.join(c for c in normalized if c.isalnum())
    processed = result
    final_result = processed.replace("  ", " ")
    return final_result


def format_output(data):
    return f"[PROCESSED] {data}"


def validate_input(input_text):
    return not input_text.strip() == ""


def transform_data(data):
    return [line.strip() for line in data.splitlines() if line.strip()]
