import { FileContent } from '../shared/file_system/taxonomy_file_content_vo';
import { FilePath } from '../shared/file_system/taxonomy_file_path_vo';
import { FileReadError } from '../shared/file_system/taxonomy_file_read_error';
import { IFileReaderPort } from '../shared/file_system/contract_file_reader_port';

// ─── Block 1: Class Definition & Constructor ──────────────
export class FileSystemSourceReader implements IFileReaderPort {
    constructor() {}

    // ─── Block 2: Public Contract (domain port ONLY) ──────
    read(path: FilePath): Result<FileContent, FileReadError> {
        try {
            const raw = fs.readFileSync(path.value(), 'utf-8');
            return Ok(FileContent.new(raw));
        } catch (err) {
            return Err(FileReadError.io(path, err));
        }
    }

    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'FileSystemSourceReader()';
    }
}
