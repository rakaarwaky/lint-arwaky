// output_client_orchestrator — Implementation of output management logic.
use crate::contract::output_client_aggregate::OutputClientAggregate;
use crate::taxonomy::{FilePath, LogOutput, Identity, FileFormat};
use std::io::{self, Write};

pub struct OutputClientOrchestrator;

impl OutputClientAggregate for OutputClientOrchestrator {}

impl OutputClientOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub fn write_output(
        &self,
        output: &LogOutput,
        command: &Identity,
        output_format: Option<&FileFormat>,
    ) -> Option<FilePath> {
        // Write content to a timestamped file in the output directory
        let output_dir = std::path::Path::new("outputs");
        if !output_dir.exists() {
            let _ = std::fs::create_dir_all(output_dir);
        }

        let ext = output_format.map(|f| f.name.as_ref()).unwrap_or("txt");
        let cmd_str = &command.value;
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}_{}.{}", cmd_str, timestamp, ext);
        let output_path = output_dir.join(&filename);

        let _ = std::fs::write(&output_path, &output.value);
        Some(FilePath::new(output_path.to_string_lossy().to_string()).unwrap())
    }

    pub fn tee_stdout<F, R>(&self, f: F) -> io::Result<R>
    where
        F: FnOnce(&mut dyn Write) -> io::Result<R>,
    {
        // Context manager that tees stdout to both terminal and a buffer
        let mut buffer = Vec::new();
        let result = f(&mut buffer)?;
        io::stdout().write_all(&buffer)?;
        io::stdout().flush()?;
        Ok(result)
    }
}
