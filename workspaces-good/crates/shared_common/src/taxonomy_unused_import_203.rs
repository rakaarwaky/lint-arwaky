use std::time::SystemTime;

pub fn unused_import_trigger() {
    let _unused = vec![1, 2, 3];
}
