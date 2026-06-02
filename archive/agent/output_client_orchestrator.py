"""output_client_orchestrator — Implementation of output management logic."""

import io
import sys
from datetime import datetime
from typing import Iterator
from contextlib import contextmanager
from pathlib import Path

from ..taxonomy import FilePath, LogOutput, Identity, FileFormat
from ..contract import OutputClientAggregate, ServiceContainerAggregate


class OutputClientOrchestrator(OutputClientAggregate):
    """
    Orchestrator for managing output files and stdout teeing.
    Grounded in the Agent layer to satisfy the OutputClientAggregate contract.
    """

    def __init__(
        self, container: ServiceContainerAggregate, default_output_dir: FilePath | None = None
    ):
        self.container = container
        self._default_output_dir = default_output_dir

    def get_output_dir(self) -> FilePath | None:
        """Get the output directory, prioritizing project configuration."""
        if self._default_output_dir:
            return self._default_output_dir

        # In a real scenario, we might pull this from a ConfigProviderPort
        return None

    def write_output(
        self,
        output: LogOutput,
        command: Identity,
        output_format: FileFormat | None = None,
    ) -> FilePath | None:
        """Write content to a timestamped file in the output directory."""
        output_dir_vo = self.get_output_dir()
        if not output_dir_vo:
            return None

        output_dir = Path(output_dir_vo.value)
        output_dir.mkdir(parents=True, exist_ok=True)

        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        ext = "txt"
        if output_format:
            if hasattr(output_format, "value"):
                ext = getattr(output_format, "value")
            elif hasattr(output_format, "name"):
                ext = getattr(output_format, "name")
            else:
                ext = str(output_format)

        cmd_str = command.value if hasattr(command, "value") else str(command)
        filename = f"{cmd_str}_{timestamp}.{ext}"
        output_path = output_dir / filename

        output_str = output.value if hasattr(output, "value") else str(output)
        output_path.write_text(output_str)
        return FilePath(value=str(output_path))

    @contextmanager
    def tee_stdout(self) -> Iterator[io.StringIO]:
        """Context manager that tees sys.stdout to both terminal and a buffer."""
        original_stdout = sys.stdout
        buffer = io.StringIO()

        class TeeWriter:
            def __init__(self, original, buffer):
                self.original = original
                self.buffer = buffer

            def write(self, s):
                self.original.write(s)
                self.buffer.write(s)

            def flush(self):
                self.original.flush()
                self.buffer.flush()

            def __getattr__(self, name):
                return getattr(self.original, name)

        tee = TeeWriter(original_stdout, buffer)
        sys.stdout = tee
        try:
            yield buffer
        finally:
            sys.stdout = original_stdout
