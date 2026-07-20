// PURPOSE: File system checker utility — stateless path and file type verification helpers
use crate::common::taxonomy_path_vo::FilePath;
use std::path::Path;

pub fn path_exists(path: &FilePath) -> bool {
    Path::new(path.value()).exists()
}

pub fn is_file(path: &FilePath) -> bool {
    Path::new(path.value()).is_file()
}

pub fn is_dir(path: &FilePath) -> bool {
    Path::new(path.value()).is_dir()
}
