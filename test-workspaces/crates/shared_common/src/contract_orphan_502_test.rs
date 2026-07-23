// AES502: Orphan contract file - not exported in lib.rs, no inbound imports
pub trait OrphanContractPort {
    fn execute(&self) -> bool;
}
