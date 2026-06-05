// This surface file violates AES023 (surface-dependency-violation)
// because it directly imports from the infrastructure layer (huggingface_downloader) instead of staying decoupled.
use crate::infrastructure::huggingface_downloader::HuggingfaceDownloader;
use crate::taxonomy::removal_types::RemovalType;

pub struct DirectInfraRouter {
    pub active: bool,
}

impl DirectInfraRouter {
    pub fn execute(&self) {
        // Direct coupling violates dependency inversion!
        let _downloader = HuggingfaceDownloader;
    }
}
