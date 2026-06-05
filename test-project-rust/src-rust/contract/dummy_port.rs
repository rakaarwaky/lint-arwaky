// Dummy contract port
// Violates AES006 Primitive Usage by using 'String' (raw primitive) instead of a VO.
// Violates AES003 Naming Convention (only 2 words: dummy_port.rs instead of word1_word2_word3)

pub trait IDummyPort {
    fn execute_action(&self, input: String) -> bool;
}
