from pydantic import BaseModel, ConfigDict, Field, model_validator
from .job_action_vo import JobId
from .lint_status_vo import ResponseData
from .error_value_vo import ErrorMessage
from .layer_content_vo import LineContentVO


class JobIdList(BaseModel):
    """Collection of job identifiers."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[JobId] = []

    def __iter__(self):
        return iter(self.values)


class ResponseDataList(BaseModel):
    """Collection of response data items."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[ResponseData] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)


class LineContentList(BaseModel):
    """Collection of source lines."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[LineContentVO] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)


class DataFlowList(BaseModel):
    """List of data flow event strings."""

    model_config = ConfigDict(frozen=True)

    @model_validator(mode="before")
    @classmethod
    def coerce_from_list(cls, data: object) -> object:
        if isinstance(data, list):
            return {"values": data}
        return data

    values: list[ErrorMessage] = Field(default_factory=list)

    def __iter__(self):
        return iter(self.values)

    def __len__(self) -> int:
        return len(self.values)
