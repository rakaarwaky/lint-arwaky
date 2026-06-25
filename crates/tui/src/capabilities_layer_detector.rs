use crate::taxonomy_file_entry_vo::AesLayer;

pub struct LayerDetector;

impl LayerDetector {
    pub fn detect(filename: &str) -> AesLayer {
        AesLayer::from_filename(filename)
    }

    pub fn detect_batch(filenames: &[String]) -> Vec<AesLayer> {
        filenames.iter().map(|f| Self::detect(f)).collect()
    }
}
