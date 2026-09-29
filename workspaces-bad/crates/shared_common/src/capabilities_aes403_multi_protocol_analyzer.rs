// AES403 multi-protocol: a single capability implements two protocols.
pub struct MultiProtocolAnalyzer;

impl IAlphaProtocol for MultiProtocolAnalyzer {
    fn execute(&self) -> i32 { 42 }
}

impl IBetaProtocol for MultiProtocolAnalyzer {
    fn execute(&self) -> i32 { 42 }
}
