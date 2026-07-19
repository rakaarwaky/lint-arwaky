from shared.file_system.taxonomy_file_content_vo import FileContent
from shared.file_system.taxonomy_file_path_vo import FilePath
from shared.file_system.taxonomy_file_read_error import FileReadError
from shared.file_system.contract_file_reader_port import IFileReaderPort


# ─── Block 1: Class Definition & Constructor ──────────────
class FileSystemSourceReader(IFileReaderPort):
    def __init__(self) -> None:
        pass

    # ─── Block 2: Public Contract (domain port ONLY) ──────
    def read(self, path: FilePath) -> Result[FileContent, FileReadError]:
        try:
            raw = path.value().read_text()
        except Exception as err:
            return Err(FileReadError.io(path, err))
        return FileContent.new(raw).map_err(FileReadError.validation)

    # ─── Block 3: Dunder Methods, Factories & Helpers ─────
    def __repr__(self) -> str:
        return "FileSystemSourceReader()"
