use std::fs;

pub struct InheritAdapter {
    root: String,
}

impl InheritAdapter {
    pub fn new(root: String) -> Self {
        Self { root }
    }

    pub fn read_file(&self, name: &str) -> String {
        fs::read_to_string(format!("{}/{}", self.root, name)).unwrap_or_default()
    }

    pub fn write_file(&self, name: &str, content: &str) {
        let _ = fs::write(format!("{}/{}", self.root, name), content);
    }

    pub fn exists(&self, name: &str) -> bool {
        std::path::Path::new(&format!("{}/{}", self.root, name)).exists()
    }

    pub fn list_dir(&self, dir: &str) -> Vec<String> {
        let path = format!("{}/{}", self.root, dir);
        fs::read_dir(&path)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .filter_map(|e| e.file_name().into_string().ok())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn remove_file(&self, name: &str) -> bool {
        fs::remove_file(format!("{}/{}", self.root, name)).is_ok()
    }

    pub fn create_dir(&self, dir: &str) {
        let _ = fs::create_dir_all(format!("{}/{}", self.root, dir));
    }

    pub fn file_size(&self, name: &str) -> u64 {
        fs::metadata(format!("{}/{}", self.root, name))
            .map(|m| m.len())
            .unwrap_or(0)
    }

    pub fn copy_file(&self, src: &str, dst: &str) -> bool {
        fs::copy(format!("{}/{}", self.root, src), format!("{}/{}", self.root, dst)).is_ok()
    }
}
