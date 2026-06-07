use crate::taxonomy::{ActionArgs, ActionName, FilePath};

pub trait PipelineInputAggregate: Send + Sync {
    fn action(&self) -> ActionName;
    fn args(&self) -> Option<&ActionArgs>;
    fn path(&self) -> Option<&FilePath>;
}
