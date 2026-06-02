use crate::taxonomy::{FilePath, FileFormat};

pub trait OutputClientAggregate: Send + Sync {
    fn get_output_dir(&self) -> Option<&FilePath>;
    fn write_output(&self, output: &str, command: &str, output_format: Option<&FileFormat>) -> Option<FilePath>;
}
