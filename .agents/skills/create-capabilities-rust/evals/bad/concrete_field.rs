// BAD: Concrete service field instead of DI (AES403)
use std::sync::Arc;

pub struct Capabilities<NameCapability> {
    collaborator: <NameCollaborator>, // BAD: concrete type
}

impl Capabilities<NameCapability> {
    pub fn new(collaborator: <NameCollaborator>) -> Self {
        Self { collaborator }
    }
}
