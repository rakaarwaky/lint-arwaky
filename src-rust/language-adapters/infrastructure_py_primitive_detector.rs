// PURPOSE: Detector: Py Primitive detection

pub struct PythonPrimitiveChecker {}

impl Default for PythonPrimitiveChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonPrimitiveChecker {
    pub fn new() -> Self {
        Self {}
    }
}
