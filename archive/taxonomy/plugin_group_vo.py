"""plugin_group_vo — Plugin group identifier value object."""

from pydantic import BaseModel, ConfigDict


class PluginGroup(BaseModel):
    """Entry point group identifier."""

    model_config = ConfigDict(frozen=True)

    value: str

    def __str__(self) -> str:
        return self.value

    def __hash__(self) -> int:
        return hash(self.value)

    def __eq__(self, other: object) -> bool:
        if isinstance(other, PluginGroup):
            return self.value == other.value
        if isinstance(other, str):
            return self.value == other
        return NotImplemented
