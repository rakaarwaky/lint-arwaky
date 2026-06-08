use crate::pipeline_jobs::taxonomy_action_vo::ActionArgs;
use crate::pipeline_jobs::taxonomy_action_vo::ActionName;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait PipelineInputAggregate: Send + Sync {
    fn action(&self) -> ActionName;
    fn args(&self) -> Option<&ActionArgs>;
    fn path(&self) -> Option<&FilePath>;
}
