// PURPOSE: IToolExecutorProtocol — protocol trait for executing external tools and capturing output
use crate::common::taxonomy_path_vo::FilePath;

pub use crate::maintenance::taxonomy_doctor_vo::ToolOutput;

pub trait IToolExecutorProtocol: Send + Sync {
    fn run_tool(&self, name: &str, args: &[&str]) -> ToolOutput;
    fn run_tool_in_dir(&self, name: &str, args: &[&str], dir: &FilePath) -> ToolOutput;
    fn tool_exists(&self, name: &str) -> bool;
    fn get_binary_path(&self) -> FilePath;
}
