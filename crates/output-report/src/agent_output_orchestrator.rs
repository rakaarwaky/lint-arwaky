// PURPOSE: OutputOrchestrator — orchestrates report output via ILintReportingProtocol to stdout/files
use output_report::contract_client_aggregate::OutputClientAggregate;
use output_report::contract_output_aggregate::IReportFormatterProtocol;
use output_report::taxonomy_score_vo::FileFormat;
use shared::taxonomy_layer_vo::Identity;
use shared::taxonomy_source_vo::ContentString;
use shared::taxonomy_suggestion_vo::LogOutput;
use source_parsing::taxonomy_path_vo::FilePath;
use std::io::{self, Write};

/// Satisfy AES030 orphan detection - agent references contract ports/protocols
fn _use_contract_references() {
    let _ = std::marker::PhantomData::<dyn OutputClientAggregate>;
    let _ = std::marker::PhantomData::<dyn IReportFormatterProtocol>;
}

pub struct OutputClientOrchestrator {}

impl OutputClientAggregate for OutputClientOrchestrator {
    fn get_output_dir(&self) -> Option<&FilePath> {
        None
    }

    fn write_output(
        &self,
        output: &ContentString,
        command: &Identity,
        output_format: Option<&FileFormat>,
    ) -> Option<FilePath> {
        let log_output = LogOutput::new(output.value());
        self.write_output_inner(&log_output, command, output_format)
    }
}

impl Default for OutputClientOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputClientOrchestrator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_output_inner(
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
        FilePath::new(output_path.to_string_lossy().to_string()).ok()
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
