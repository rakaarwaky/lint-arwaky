use crate::taxonomy::{ActionArgs, FilePath};

pub trait PipelineInputAggregate: Send + Sync {
    fn action(&self) -> &str;
    fn args(&self) -> Option<&ActionArgs>;
    fn path(&self) -> Option<&FilePath>;
}
