use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;

thread_local! {
    static FILE_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub fn read_cached(path: &str) -> String {
    FILE_CACHE.with(|cache| -> String {
        let mut cache = cache.borrow_mut();
        if let Some(content) = cache.get(path) {
            return content.clone();
        }
        let content = fs::read_to_string(path).unwrap_or_default();
        cache.insert(path.to_string(), content.clone());
        content
    })
}

pub fn clear_cache() {
    FILE_CACHE.with(|c| c.borrow_mut().clear());
}
