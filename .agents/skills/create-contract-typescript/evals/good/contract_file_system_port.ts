import { FileContent } from '../file_system/taxonomy_file_content_vo';
import { FilePath } from '../file_system/taxonomy_file_path_vo';
import { FileReadError } from '../file_system/taxonomy_file_read_error';

export interface IFileSystemPort {
    readFile(path: FilePath): Promise<Result<FileContent, FileReadError>>;
}
