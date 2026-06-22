// AES505 violation: agent orphan — this aggregate is never called by any surface
pub trait IOrphanAggregate: Send + Sync {
    fn do_something(&self) -> String;
}

pub struct OrphanContainer;

impl IOrphanAggregate for OrphanContainer {
    fn do_something(&self) -> String {
        "orphan".to_string()
    }
}
