use crate::taxonomy::FilePath;
use crate::taxonomy::ProjectConfig;


pub trait IConfigProviderPort: Send + Sync {
    fn load_project_config(&self, path: Option<&FilePath>) -> ProjectConfig;
}
