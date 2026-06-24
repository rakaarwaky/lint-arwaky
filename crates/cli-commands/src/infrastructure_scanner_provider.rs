// PURPOSE: CliScannerProvider — re-exports FileCollectorProvider from shared
// as a local alias for the cli-commands crate's DI container wiring.
pub use shared::source_parsing::taxonomy_path_vo::FilePath;

pub use shared::source_parsing::infrastructure_file_collector_provider::FileCollectorProvider as CliScannerProvider;
