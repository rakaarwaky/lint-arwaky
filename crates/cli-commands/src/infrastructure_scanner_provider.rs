// PURPOSE: CliScannerProvider — re-exports FileCollectorProvider from shared
// as a local alias for the cli-commands crate's DI container wiring.
#[allow(unused_imports)]
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;

pub use shared::source_parsing::taxonomy_path_vo::FilePath;

pub use shared::source_parsing::infrastructure_file_collector_provider::FileCollectorProvider as CliScannerProvider;
