// orphan_graph_resolver — Helper to resolve import graphs and reachability for orphan detection.
// Implements IOrphanGraphProtocol: build_graph_context, resolve_import_to_file,
//   identify_entry_points, trace_reachability.

use crate::taxonomy::{FileDefinitionMap, GraphAnalysisContext, ImportGraph, InboundLinkMap, InheritanceMap, ModuleToFileMap};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

const ENTRY_POINT_NAMES: &[&str] = &[
    "cli_main_entry.py",
    "mcp_main_entry.py",
    "mcp_server_handler.py",
    "main.py",
];

const SRC_SUBDIRS: &[&str] = &["", "src", "lib", "app", "core"];

/// Helper to resolve import graphs and reachability for orphan detection.
pub struct OrphanGraphResolver;

impl OrphanGraphResolver {
    pub fn new() -> Self {
        Self
    }

    /// Pre-map modules to files to avoid FS hits during import resolution.
    pub fn discover_files(&self, project_files: &[String], root_dir: &str) -> ModuleToFileMap {
        let mut mapping: HashMap<String, String> = HashMap::new();

        for abs_path in project_files {
            // Simplified: just use the file path directly without canonicalize
            let rel = abs_path.trim_start_matches(root_dir).trim_start_matches('/');

            let mut mod_name = rel
                .trim_end_matches(".py")
                .replace('/', ".");

            if mod_name.ends_with(".__init__") {
                mod_name = mod_name[..mod_name.len()-9].to_string();
            }

            if mod_name.starts_with("src.") {
                mod_name = mod_name[4..].to_string();
            } else if mod_name.starts_with("lib.") {
                mod_name = mod_name[4..].to_string();
            }

            mapping.insert(mod_name, abs_path.clone());
        }

        ModuleToFileMap { mapping }
    }

    fn extract_imports_from_file(file: &str) -> Vec<String> {
        let Ok(content) = fs::read_to_string(file) else {
            return vec![];
        };
        let mut modules: Vec<String> = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("from ") {
                if let Some(module) = rest.splitn(2, " import ").next() {
                    modules.push(module.trim().to_string());
                }
            } else if let Some(rest) = trimmed.strip_prefix("import ") {
                for part in rest.split(',') {
                    if let Some(name) = part.trim().split_whitespace().next() {
                        modules.push(name.to_string());
                    }
                }
            }
        }
        modules
    }

    fn extract_class_bases(content: &str) -> HashMap<String, Vec<String>> {
        let mut result: HashMap<String, Vec<String>> = HashMap::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("class ") {
                // Parse: class ClassName(Base1, Base2):
                if let Some(paren_start) = trimmed.find('(') {
                    if let Some(paren_end) = trimmed.find(')') {
                        let class_name = trimmed[6..paren_start].trim().to_string();
                        let bases_str = &trimmed[paren_start + 1..paren_end];
                        let bases: Vec<String> = bases_str
                            .split(',')
                            .map(|b| b.trim().to_string())
                            .filter(|b| !b.is_empty())
                            .collect();
                        result.insert(class_name, bases);
                    }
                }
            }
        }
        result
    }

    /// Resolve a module path to an absolute file path.
    pub fn resolve_import_to_file(
        &self,
        module_str: &str,
        current_file: &str,
        root_dir: &str,
        module_to_file: &ModuleToFileMap,
    ) -> Option<String> {
        // Phase 1: Direct cache lookup
        if let Some(path) = module_to_file.mapping.get(module_str) {
            return Some(path.clone());
        }

        // Phase 2: Relative import
        if module_str.starts_with('.') {
            let dot_count = module_str.chars().take_while(|&c| c == '.').count();
            let rel_module = &module_str[dot_count..];

            let current_dir = Path::new(current_file).parent()?;
            let mut target_dir = current_dir.to_path_buf();
            for _ in 1..dot_count {
                target_dir = target_dir.parent()?.to_path_buf();
            }

            if !rel_module.is_empty() {
                let parts: Vec<&str> = rel_module.split('.').collect();
                let attempt = target_dir.join(parts.join("/")).with_extension("py");
                if attempt.exists() {
                    return Some(attempt.to_string_lossy().to_string());
                }
                let init_attempt = target_dir.join(parts.join("/")).join("__init__.py");
                if init_attempt.exists() {
                    return Some(init_attempt.to_string_lossy().to_string());
                }
            }
        }

        // Phase 3: Absolute import
        let parts: Vec<&str> = module_str.split('.').collect();
        for sd in SRC_SUBDIRS {
            let base = if sd.is_empty() {
                PathBuf::from(root_dir)
            } else {
                PathBuf::from(root_dir).join(sd)
            };

            let attempt = base.join(parts.join("/")).with_extension("py");
            if attempt.exists() {
                return Some(attempt.to_string_lossy().to_string());
            }
            let init_attempt = base.join(parts.join("/")).join("__init__.py");
            if init_attempt.exists() {
                return Some(init_attempt.to_string_lossy().to_string());
            }
        }

        None
    }

    /// Build complete import graph and inbound link map.
    pub fn collect_import_map(
        &self,
        project_files: &[String],
        root_dir: &str,
        module_to_file: &ModuleToFileMap,
    ) -> (ImportGraph, InboundLinkMap) {
        let mut import_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut inbound_map: HashMap<String, Vec<String>> = project_files.iter()
            .map(|f| (f.clone(), vec![]))
            .collect();

        for file in project_files {
            import_map.insert(file.clone(), vec![]);
            let modules = Self::extract_imports_from_file(file);

            for module in modules {
                if let Some(target) = self.resolve_import_to_file(&module, file, root_dir, module_to_file) {
                    import_map.get_mut(file).unwrap().push(target.clone());
                    inbound_map.entry(target).or_default().push(file.clone());
                }
            }
        }

        (ImportGraph { mapping: import_map }, InboundLinkMap { mapping: inbound_map })
    }

    /// Collect inheritance relationships from project files.
    pub fn collect_inheritance_chain(
        &self,
        project_files: &[String],
    ) -> InheritanceMap {
        let mut inheritance_mapping: HashMap<String, Vec<String>> = HashMap::new();

        for file in project_files {
            let Ok(content) = fs::read_to_string(file) else { continue; };
            let bases_map = Self::extract_class_bases(&content);
            for (_, bases) in bases_map {
                for base in bases {
                    inheritance_mapping.entry(base).or_default().push(file.clone());
                }
            }
        }

        InheritanceMap { mapping: inheritance_mapping }
    }

    /// Build all graph analysis maps in one pass.
    pub fn build_graph_context(
        &self,
        project_files: &[String],
        root_dir: &str,
    ) -> GraphAnalysisContext {
        let module_to_file = self.discover_files(project_files, root_dir);
        let (import_graph, inbound_links) = self.collect_import_map(project_files, root_dir, &module_to_file);
        let inheritance_map = self.collect_inheritance_chain(project_files);
        let file_definitions = self.collect_file_definitions(project_files);

        GraphAnalysisContext {
            import_graph,
            inbound_links,
            inheritance_map,
            file_definitions,
        }
    }

    /// Extract class names defined in a file (keys) and their bases (values).
    fn extract_class_names(content: &str) -> HashMap<String, Vec<String>> {
        let mut result: HashMap<String, Vec<String>> = HashMap::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("class ") {
                if let Some(paren_start) = trimmed.find('(') {
                    if let Some(paren_end) = trimmed.find(')') {
                        let class_name = trimmed[6..paren_start].trim().to_string();
                        let bases_str = &trimmed[paren_start + 1..paren_end];
                        let bases: Vec<String> = bases_str
                            .split(',')
                            .map(|b| b.trim().to_string())
                            .filter(|b| !b.is_empty())
                            .collect();
                        result.insert(class_name, bases);
                    }
                }
            }
        }
        result
    }

    /// Collect class definitions from project files.
    pub fn collect_file_definitions(&self, project_files: &[String]) -> FileDefinitionMap {
        let mut file_defs: HashMap<String, Vec<String>> = HashMap::new();

        for file in project_files {
            let Ok(content) = fs::read_to_string(file) else { continue; };
            let bases_map = Self::extract_class_names(&content);
            let class_names: Vec<String> = bases_map.keys().cloned().collect();
            file_defs.insert(file.clone(), class_names);
        }

        FileDefinitionMap { mapping: file_defs }
    }

    /// BFS from entry points to find all reachable files.
    pub fn identify_entry_points(&self, project_files: &[String]) -> Vec<String> {
        project_files.iter()
            .filter(|file| {
                let basename = Path::new(file)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("");
                ENTRY_POINT_NAMES.contains(&basename)
            })
            .cloned()
            .collect()
    }

    /// BFS from entry points to find all reachable files.
    pub fn trace_reachability(
        &self,
        entry_points: &[String],
        graph: &ImportGraph,
    ) -> Vec<String> {
        let mut reachable: HashSet<String> = entry_points.iter().cloned().collect();
        let mut queue: VecDeque<String> = entry_points.iter().cloned().collect();

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = graph.mapping.get(&current) {
                for neighbor in neighbors {
                    if reachable.insert(neighbor.clone()) {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        reachable.into_iter().collect()
    }
}
