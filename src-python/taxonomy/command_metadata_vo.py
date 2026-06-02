"""command_metadata_vo — Command information value objects."""

from pydantic import BaseModel, ConfigDict, model_validator
from .log_suggestion_vo import DescriptionVO, Suggestion


class CommandMetadataVO(BaseModel):
    """Metadata for a CLI command."""

    model_config = ConfigDict(frozen=True)
    description: DescriptionVO
    example: Suggestion

    @model_validator(mode="before")
    @classmethod
    def coerce_fields(cls, data: object) -> object:
        if isinstance(data, dict):
            if "description" in data and isinstance(data["description"], str):
                data["description"] = DescriptionVO(value=data["description"])
            if "example" in data and isinstance(data["example"], str):
                data["example"] = Suggestion(value=data["example"])
        return data
