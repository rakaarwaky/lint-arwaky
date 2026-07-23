// AES502: Contract Orphan violation - this contract protocol is not implemented by expected layer
pub trait OrphanPort {
    fn required_method(&self) -> bool;
}