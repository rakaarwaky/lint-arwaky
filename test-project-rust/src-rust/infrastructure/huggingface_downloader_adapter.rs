// Infrastructure: HuggingFace downloader adapter
// Referenced by direct_infra_router.rs for AES023 test

pub struct HuggingfaceDownloader;

impl HuggingfaceDownloader {
    pub fn download(&self, _url: &str) -> Vec<u8> {
        vec![]
    }
}
