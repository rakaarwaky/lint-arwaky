// Utility helpers shared by the filesystem agent orchestrator.
// All functions here are pure: no `self`, no side effects, domain-agnostic.
use crate::utility_workspace_detection;
use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Builds a lexicographically-sorted index of file stems → file paths.
/// Delegates to `utility_workspace_detection::build_stem_index`; kept as a
/// thin agent wrapper for direct callers (e.g. tests).
pub fn build_stem_index(file_paths: &[String]) -> HashMap<String, Vec<String>> {
    utility_workspace_detection::build_stem_index(file_paths)
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
