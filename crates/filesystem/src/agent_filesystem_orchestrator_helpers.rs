// Agent layer — helper methods extracted from FilesystemOrchestrator (AES301: file too large)
// These methods access pub(crate) fields on FilesystemOrchestrator directly.

use crate::agent_filesystem_orchestrator::FilesystemOrchestrator;
use shared::filesystem::contract_parser_protocol::IParserProtocol;
use shared::filesystem::taxonomy_filesystem_vo::{DefinitionEntry, ImplEntry};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

impl FilesystemOrchestrator {
    /// Walk filesystem from root, discover source files, read content, parse imports.
    /// Populates files, file_index, imports, warnings caches.
    /// `extra_ignored` additional patterns beyond the built-in defaults.
    pub fn build_file_index_impl(&self, root: &Path, extra_ignored: &[String]) {
        if self.files.get().is_some() {
            return;
        }

        // Merge single-source defaults + caller-provided patterns
        let mut ignored: Vec<String> = shared::common::DEFAULT_IGNORED_PATHS
            .iter()
            .map(|s| format!("{}/", s))
            .collect();
        ignored.extend_from_slice(extra_ignored);

        // Canonicalize root so discovered paths are absolute, matching the
        // absolute-path lookups used by the orphan analyzer's content_map.
        let abs_root = std::fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());

        // Detect workspace member directories (crates/, packages/, modules/).
        // When scanning a workspace root, only include files inside these member
        // directories — exclude root-level files like setup.py, package.json, etc.
        let member_dirs: Vec<&str> = ["crates", "packages", "modules"]
            .iter()
            .filter(|d| abs_root.join(d).is_dir())
            .copied()
            .collect();

        let scanned: Vec<PathBuf> =
            crate::utility_workspace_detection::discover_source_files(&abs_root, &ignored)
                .into_iter()
                .map(PathBuf::from)
                .filter(|p| {
                    if member_dirs.is_empty() {
                        // No member dirs — scanning a single member or standalone project.
                        return true;
                    }
                    if let Ok(rel) = p.strip_prefix(&abs_root) {
                        let rel_str = rel.to_string_lossy();
                        // Include files inside member directories
                        if member_dirs
                            .iter()
                            .any(|d| rel_str.starts_with(&format!("{}/", d)))
                        {
                            return true;
                        }
                        // Also include root-level source files (e.g. root_*_entry.rs)
                        !rel_str.contains('/')
                    } else {
                        true
                    }
                })
                .collect();

        let mut entries = Vec::new();
        let mut all_imports = Vec::new();
        let all_warnings = Vec::new();

        for path in &scanned {
            let language = self
                .deps
                .workspace
                .detect_language_from_path(&path.to_string_lossy());
            let content = match self.deps.io.read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let lang_enum = match language {
                shared::common::taxonomy_config_language_vo::ConfigLanguage::Rust => {
                    shared::filesystem::taxonomy_filesystem_vo::Language::Rust
                }
                shared::common::taxonomy_config_language_vo::ConfigLanguage::Python => {
                    shared::filesystem::taxonomy_filesystem_vo::Language::Python
                }
                shared::common::taxonomy_config_language_vo::ConfigLanguage::TypeScript => {
                    shared::filesystem::taxonomy_filesystem_vo::Language::TypeScript
                }
            };
            let imports = self.deps.parser.extract(path, &content, lang_enum);
            all_imports.extend(imports);

            // parse_ok: true when content is non-empty and language is recognized
            let parse_ok = !content.is_empty()
                && lang_enum != shared::filesystem::taxonomy_filesystem_vo::Language::Unknown;

            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();
            let size = content.len() as u64;
            entries.push(shared::filesystem::taxonomy_filesystem_vo::FileEntry {
                path: path.clone(),
                extension,
                language: lang_enum,
                size,
                content,
                parse_ok,
                parse_metadata: None,
            });
        }

        // Populate parse_metadata (including used_identifiers) via AST parser BEFORE caching.
        self.parse_all(&mut entries);

        // Resolve imports through barrel files (__init__.py, mod.rs, index.ts, etc.)
        // so that resolved_path is populated for the forward dependency graph.
        self.resolve_barrel_imports(&abs_root);
        let resolved_imports = self.deps.parser.import_list().to_vec();

        let _ = self.files.set(entries.clone());
        let _ = self.imports.set(resolved_imports);
        let _ = self.warnings.set(all_warnings);

        let index: HashMap<PathBuf, usize> = entries
            .iter()
            .enumerate()
            .map(|(i, entry)| (entry.path.clone(), i))
            .collect();
        let _ = self.file_index.set(index);
    }

    pub(crate) fn ensure_graph_built(&self) {
        if self.cached_reverse_links.get().is_some() {
            return;
        }
        let files = self.files.get().cloned().unwrap_or_default();
        let imports = self.imports.get().cloned().unwrap_or_default();

        // Extract definitions and implementations from parsed file metadata
        let mut definitions: Vec<DefinitionEntry> = Vec::new();
        let mut implementations: Vec<ImplEntry> = Vec::new();
        for entry in &files {
            if !entry.parse_ok {
                continue;
            }
            if let Some(ref meta) = entry.parse_metadata {
                match meta {
                    shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::Rust(m) => {
                        let lang = entry.language;
                        // Collect all symbol definitions from Rust metadata
                        for name in m
                            .struct_definitions
                            .iter()
                            .chain(m.enum_definitions.iter())
                            .chain(m.trait_definitions.iter())
                            .chain(m.type_definitions.iter())
                        {
                            definitions.push(DefinitionEntry {
                                name: name.clone(),
                                file_path: entry.path.clone(),
                                language: lang,
                            });
                        }
                        // Collect trait implementations
                        for item in &m.impl_blocks {
                            if let Some(ref trait_name) = item.trait_name {
                                implementations.push(ImplEntry {
                                    trait_name: trait_name.clone(),
                                    file_path: entry.path.clone(),
                                    language: lang,
                                });
                            }
                        }
                    }
                    shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::Python(m) => {
                        let lang = entry.language;
                        for class in &m.class_declarations {
                            definitions.push(DefinitionEntry {
                                name: class.name.clone(),
                                file_path: entry.path.clone(),
                                language: lang,
                            });
                        }
                    }
                    shared::filesystem::taxonomy_filesystem_vo::ParseMetadata::TypeScript(m) => {
                        let lang = entry.language;
                        for class in &m.class_declarations {
                            definitions.push(DefinitionEntry {
                                name: class.name.clone(),
                                file_path: entry.path.clone(),
                                language: lang,
                            });
                        }
                        for iface in &m.interface_declarations {
                            definitions.push(DefinitionEntry {
                                name: iface.clone(),
                                file_path: entry.path.clone(),
                                language: lang,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        self.deps
            .graph
            .build_graph(&imports, &files, &definitions, &implementations);

        if let Some(rl) = self.deps.graph.reverse_links().clone().into() {
            let _ = self.cached_reverse_links.set(rl);
        }
        if let Some(sd) = self.deps.graph.symbol_definitions().clone().into() {
            let _ = self.cached_definitions.set(sd);
        }
        if let Some(imp) = self.deps.graph.implementations().clone().into() {
            let _ = self.cached_implementations.set(imp);
        }
    }
}
