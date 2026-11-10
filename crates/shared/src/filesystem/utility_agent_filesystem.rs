// Utility helpers shared by the filesystem agent orchestrator.
// All functions here are pure: no `self`, no side effects, domain-agnostic.
// Inlined: stateless standalone function; no self-import (AES201: utility must not import utility).
use std::collections::HashMap;
use std::path::Path;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Builds a lexicographically-sorted index of file stems → file paths.
pub fn build_stem_index(file_paths: &[String]) -> HashMap<String, Vec<String>> {
    let mut stem_index: HashMap<String, Vec<String>> = HashMap::new();
    for f in file_paths {
        if let Some(stem) = Path::new(f).file_stem().and_then(|s| s.to_str()) {
            stem_index
                .entry(stem.to_string())
                .or_default()
                .push(f.clone());
        }
    }
    for v in stem_index.values_mut() {
        v.sort();
    }
    stem_index
}

/// Acquires a read guard on an `RwLock`, recovering from a poisoned lock.
pub fn read_guard<'a, T>(lock: &'a RwLock<T>) -> RwLockReadGuard<'a, T> {
    match lock.read() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}

/// Acquires a write guard on an `RwLock`, recovering from a poisoned lock.
pub fn write_guard<'a, T>(lock: &'a RwLock<T>) -> RwLockWriteGuard<'a, T> {
    match lock.write() {
        Ok(g) => g,
        Err(p) => p.into_inner(),
    }
}
