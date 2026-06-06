use crate::taxonomy::FileFormat;
use crate::taxonomy::FilePath;
use async_trait::async_trait;


#[async_trait]
pub trait ReportCommandsAggregate: Send + Sync {
    fn root_path(&self) -> Option<&FilePath>;
    async fn report(&self, path: &FilePath, output_format: &FileFormat);
    async fn security(&self, path: &FilePath);
}
