from dataclasses import dataclass


@dataclass(frozen=True)
class FilePath:
    _value: str

    def __post_init__(self) -> None:
        if not self._value.strip():
            raise ValueError("FilePath cannot be empty")

    @property
    def value(self) -> str:
        return self._value

    def __str__(self) -> str:
        return self._value
