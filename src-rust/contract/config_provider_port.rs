use crate::taxonomy::FilePath;
use crate::taxonomy::ProjectConfig;
use super::*;

pub trait IConfigProviderPort: Send + Sync {
    fn load_project_config(&self, path: Option<&FilePath>) -> ProjectConfig;
}
