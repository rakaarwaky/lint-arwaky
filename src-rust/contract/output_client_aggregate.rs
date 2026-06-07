use crate::taxonomy::{ContentString, FileFormat, FilePath, Identity};

pub trait OutputClientAggregate: Send + Sync {
    fn get_output_dir(&self) -> Option<&FilePath>;
    fn write_output(
        &self,
        output: &ContentString,
        command: &Identity,
        output_format: Option<&FileFormat>,
    ) -> Option<FilePath>;
}
