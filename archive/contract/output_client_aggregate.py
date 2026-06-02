from abc import ABC, abstractmethod
from typing import Iterator
import io
from ..taxonomy import FilePath, LogOutput, Identity, FileFormat

class OutputClientAggregate(ABC):
    """AGGREGATE: Orchestrator for managing output files and stdout teeing."""
    @abstractmethod
    def get_output_dir(self) -> FilePath | None:
        ...

    @abstractmethod
    def write_output(
        self,
        output: LogOutput,
        command: Identity,
        output_format: FileFormat | None = None,
    ) -> FilePath | None:
        ...

    @abstractmethod
    def tee_stdout(self) -> Iterator[io.StringIO]:
        ...
