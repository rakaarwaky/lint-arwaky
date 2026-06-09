# AES038: Missing VO - capability method call missing required VO parameter
class CapabilitiesMissingVoProcessor:
    def process(self, raw_id: int) -> str:
        # Should use Value Object instead of raw int
        return f"processing {raw_id}"
