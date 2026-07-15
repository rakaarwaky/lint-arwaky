// PURPOSE: ILayerPrefixPort — contract trait for layer prefix extraction
pub trait ILayerPrefixPort: Send + Sync {
    fn extract_layer_from_prefix(&self, filename: &str) -> Option<String>;
}
