// PURPOSE: File system checker utility — stateless path and file type verification helpers
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::utility_file;

pub fn path_exists(path: &FilePath) -> bool {
    utility_file::path_exists(path.value())
}

pub fn is_file(path: &FilePath) -> bool {
    utility_file::is_file_generic(path.value())
}

pub fn is_dir(path: &FilePath) -> bool {
    utility_file::is_dir(path.value())
}
