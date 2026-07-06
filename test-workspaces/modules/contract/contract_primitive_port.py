class ContractPrimitivePort:
    """Contract port using primitive types — violates AES402."""

    def process_data(self, data: str) -> int:
        return len(data)

    def validate_input(self, name: str) -> bool:
        return len(name) > 0

    def transform_result(self, value: int) -> str:
        return str(value)

    def fetch_records(self, query: str) -> list:
        return []

    def build_cache(self, key: str) -> dict:
        return {}
