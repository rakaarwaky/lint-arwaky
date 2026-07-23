use crate::taxonomy::DescriptionVO;

fn _use_mandatory_imports(s: DescriptionVO) {
    let _ = s;
}

pub struct DummyFunctionEntity {
    pub id: u64,
}

impl DummyFunctionEntity {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
