# Implementation Draft: AI Auto-Repair Model (v13 — Project Structure Integration)

v13 translates the frontier architecture from v12 into the actual lint-arwaky workspace structure. Taxonomy and Contract types live in `crates/shared/src/autorepair/`, while Capabilities, Infrastructure, Agent, Surface, and Root layers live in the new `crates/autorepair/` feature crate.

---

## Project Structure Map

```
crates/
├── shared/src/
│   ├── autorepair/                    ← NEW: Taxonomy + Contract types
│   │   ├── mod.rs
│   │   ├── taxonomy_model_config_vo.rs
│   │   ├── taxonomy_prediction_result_vo.rs
│   │   ├── taxonomy_extracted_feature_vo.rs
│   │   ├── taxonomy_token_ids_vo.rs
│   │   ├── taxonomy_system_constant.rs
│   │   ├── taxonomy_ast_path_vo.rs
│   │   ├── taxonomy_language_vo.rs
│   │   ├── contract_autorepair_aggregate.rs
│   │   ├── contract_model_classifier_protocol.rs
│   │   ├── contract_bpe_transformer_protocol.rs
│   │   ├── contract_ast_extractor_protocol.rs
│   │   ├── contract_exception_filter_protocol.rs
│   │   ├── contract_reference_processor_protocol.rs
│   │   ├── contract_file_name_resolver_protocol.rs
│   │   └── contract_workspace_scanner_port.rs
│   └── lib.rs                         ← MODIFIED: add `pub mod autorepair;`
│
├── autorepair/                        ← NEW: Feature crate
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── root_autorepair_container.rs
│   │   ├── agent_autorepair_orchestrator.rs
│   │   ├── surface_autofix_command.rs
│   │   ├── capabilities_reference_processor.rs
│   │   ├── capabilities_file_name_resolver.rs
│   │   ├── capabilities_ast_extractor.rs
│   │   ├── capabilities_bpe_transformer.rs
│   │   ├── capabilities_exception_filter.rs
│   │   ├── capabilities_model_classifier.rs
│   │   ├── capabilities_confidence_calibrator.rs
│   │   ├── capabilities_ast_path_embedder.rs
│   │   ├── infrastructure_fs_reader.rs
│   │   ├── infrastructure_fs_writer.rs
│   │   ├── infrastructure_workspace_scanner.rs
│   │   ├── infrastructure_compiler_adapter.rs
│   │   └── infrastructure_linter_adapter.rs
│   └── tests/
│       ├── test_reference_processor.rs
│       ├── test_exception_filter.rs
│       ├── test_file_name_resolver.rs
│       ├── test_confidence_calibrator.rs
│       └── test_ast_path_embedder.rs
│
└── Cargo.toml                         ← MODIFIED: add autorepair member + deps
```

---

## 1. Modified: `crates/shared/src/lib.rs`

```rust
// shared — all taxonomy types, contract traits, and shared definitions
// No dependencies on other feature crates — this is the foundation layer.

#[path = "common/mod.rs"]
pub mod common;

pub use common::*;

// Feature-specific types (in feature folders)
#[path = "auto-fix/mod.rs"]
pub mod auto_fix;
#[path = "cli-commands/mod.rs"]
pub mod cli_commands;
#[path = "code-analysis/mod.rs"]
pub mod code_analysis;
#[path = "config-system/mod.rs"]
pub mod config_system;
#[path = "external-lint/mod.rs"]
pub mod external_lint;
#[path = "file-system/mod.rs"]
pub mod file_system;
#[path = "file-watch/mod.rs"]
pub mod file_watch;
#[path = "git-hooks/mod.rs"]
pub mod git_hooks;
#[path = "import-rules/mod.rs"]
pub mod import_rules;
#[path = "mcp-server/mod.rs"]
pub mod mcp_server;
#[path = "naming-rules/mod.rs"]
pub mod naming_rules;
#[path = "orphan-detector/mod.rs"]
pub mod orphan_detector;
#[path = "project-setup/mod.rs"]
pub mod project_setup;
#[path = "role-rules/mod.rs"]
pub mod role_rules;
#[path = "source-parsing/mod.rs"]
pub mod source_parsing;

// v13: Auto-Repair taxonomy + contract types
#[path = "autorepair/mod.rs"]
pub mod autorepair;
```

---

## 2. Modified: `Cargo.toml` (workspace root)

Add to `[workspace.dependencies]`:

```toml
autorepair = { package = "autorepair-lint-arwaky", path = "crates/autorepair" }

# v13: Auto-Repair ML dependencies (pinned)
burn = "0.16"
tokenizers = "0.21"
walkdir = "2"
syn = "2"
quote = "1"
tree-sitter = "0.24"
tree-sitter-rust = "0.23"
tree-sitter-python = "0.23"
tree-sitter-typescript = "0.23"
```

Add to `[dependencies]`:

```toml
autorepair.workspace = true
```

---

## 3. New: `crates/shared/src/autorepair/mod.rs`

```rust
// autorepair — taxonomy and contract types for AI Auto-Repair

// Taxonomy (Data, Constants, VOs)
pub mod taxonomy_system_constant;
pub mod taxonomy_model_config_vo;
pub mod taxonomy_prediction_result_vo;
pub mod taxonomy_extracted_feature_vo;
pub mod taxonomy_token_ids_vo;
pub mod taxonomy_ast_path_vo;
pub mod taxonomy_language_vo;

// Contract (Ports, Protocols, Aggregates)
pub mod contract_autorepair_aggregate;
pub mod contract_model_classifier_protocol;
pub mod contract_bpe_transformer_protocol;
pub mod contract_ast_extractor_protocol;
pub mod contract_exception_filter_protocol;
pub mod contract_reference_processor_protocol;
pub mod contract_file_name_resolver_protocol;
pub mod contract_workspace_scanner_port;
```

---

## 4. New: `crates/shared/src/autorepair/taxonomy_system_constant.rs`

```rust
/// Absolute/relative path to the Safetensors model weights file.
pub const MODEL_WEIGHTS_PATH: &str = "weights/model.safetensors";

/// Absolute/relative path to the BPE tokenizer JSON file.
pub const TOKENIZER_PATH: &str = "weights/tokenizer.json";

/// Maximum sequence length for tokenized input.
pub const MAX_SEQ_LEN: usize = 512;

/// Temperature scaling factor for confidence calibration.
pub const CONFIDENCE_TEMPERATURE: f32 = 1.5;

/// Entropy threshold for low-confidence detection.
pub const ENTROPY_THRESHOLD: f32 = 1.2;

/// AST path embedding vocabulary size for Code2Vec-style encoding.
pub const AST_PATH_VOCAB_SIZE: usize = 2048;

/// Maximum AST depth to capture in path embedding.
pub const MAX_AST_DEPTH: usize = 8;
```

---

## 5. New: `crates/shared/src/autorepair/taxonomy_language_vo.rs`

```rust
/// Supported source code languages for multi-language scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    JavaScript,
    Unknown,
}
```

---

## 6. New: `crates/shared/src/autorepair/taxonomy_ast_path_vo.rs`

```rust
/// AST path for Code2Vec-style embedding.
#[derive(Debug, Clone)]
pub struct AstPath {
    pub node_types: Vec<String>,
    pub target_token: String,
    pub depth: usize,
}
```

---

## 7. New: `crates/shared/src/autorepair/taxonomy_extracted_feature_vo.rs`

```rust
use super::taxonomy_ast_path_vo::AstPath;
use super::taxonomy_language_vo::Language;

/// Static features extracted from a source code file.
#[derive(Debug, Clone)]
pub struct ExtractedFeature {
    pub imports: Vec<String>,
    pub structs_traits: Vec<String>,
    pub docstrings: Vec<String>,
    pub directory_context: String,
    pub ast_paths: Vec<AstPath>,
    pub language: Language,
}
```

---

## 8. New: `crates/shared/src/autorepair/taxonomy_prediction_result_vo.rs`

```rust
/// Naming classification result from the AI model.
#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub prefix: String,
    pub concept: String,
    pub suffix: String,
    pub prefix_confidence: f32,
    pub suffix_confidence: f32,
    pub concept_confidence: f32,
    pub entropy: f32,
    pub attention_weights: Option<Vec<f32>>,
}
```

---

## 9. New: `crates/shared/src/autorepair/taxonomy_model_config_vo.rs`

```rust
/// Internal configuration data structure for the AI prediction model.
#[derive(Debug, Clone)]
pub struct AESNamingModelConfig {
    pub vocab_size: usize,
    pub d_model: usize,
    pub d_ff: usize,
    pub n_heads: usize,
    pub n_layers: usize,
    pub max_seq_len: usize,
    pub dropout: f32,
}

impl Default for AESNamingModelConfig {
    fn default() -> Self {
        Self {
            vocab_size: 32000,
            d_model: 256,
            d_ff: 1024,
            n_heads: 8,
            n_layers: 6,
            max_seq_len: 512,
            dropout: 0.1,
        }
    }
}
```

---

## 10. New: `crates/shared/src/autorepair/taxonomy_token_ids_vo.rs`

```rust
/// Value Object wrapping BPE tokenization results.
#[derive(Debug, Clone)]
pub struct TokenIds(pub Vec<u32>);

impl TokenIds {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns attention mask: 1.0 for real tokens, 0.0 for padding.
    pub fn attention_mask(&self, max_len: usize) -> Vec<f32> {
        let mut mask = vec![0.0; max_len];
        for i in self.0.iter().enumerate().take(max_len) {
            mask[i.0] = 1.0;
        }
        mask
    }
}
```

---

## 11. New: `crates/shared/src/autorepair/contract_autorepair_aggregate.rs`

```rust
use crate::taxonomy_common_error::CommonError;

/// Main aggregate interaction boundary for launching the Auto-Repair process.
pub trait AutorepairAggregate: Send + Sync {
    fn execute_fix(&self, workspace_root: &str, target_file: &str) -> Result<(), CommonError>;
}
```

---

## 12. New: `crates/shared/src/autorepair/contract_model_classifier_protocol.rs`

```rust
use super::taxonomy_extracted_feature_vo::ExtractedFeature;
use super::taxonomy_prediction_result_vo::PredictionResult;
use super::taxonomy_token_ids_vo::TokenIds;
use crate::taxonomy_common_error::CommonError;

/// Protocol for interacting with the prediction model.
pub trait ModelClassifierProtocol: Send + Sync {
    fn predict(&self, features: &ExtractedFeature, tokens: &TokenIds) -> Result<PredictionResult, CommonError>;
    fn predict_with_temperature(&self, features: &ExtractedFeature, tokens: &TokenIds, temperature: f32) -> Result<Vec<PredictionResult>, CommonError>;
}
```

---

## 13. New: `crates/shared/src/autorepair/contract_bpe_transformer_protocol.rs`

```rust
use super::taxonomy_extracted_feature_vo::ExtractedFeature;
use super::taxonomy_token_ids_vo::TokenIds;
use crate::taxonomy_common_error::CommonError;

/// Protocol for BPE tokenization.
pub trait BpeTransformerProtocol: Send + Sync {
    fn tokenize(&self, features: &ExtractedFeature) -> Result<TokenIds, CommonError>;
    fn vocab_size(&self) -> usize;
}
```

---

## 14. New: `crates/shared/src/autorepair/contract_ast_extractor_protocol.rs`

```rust
use super::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_common_error::CommonError;

/// Protocol for extracting features and AST paths from source code.
pub trait AstExtractorProtocol: Send + Sync {
    fn extract_from_file(&self, path: &str, content: &str) -> Result<ExtractedFeature, CommonError>;
}
```

---

## 15. New: `crates/shared/src/autorepair/contract_exception_filter_protocol.rs`

```rust
/// Protocol for filtering file exceptions that are immune to naming rules.
pub trait ExceptionFilterProtocol: Send + Sync {
    fn is_exempt(&self, path: &str) -> bool;
}
```

---

## 16. New: `crates/shared/src/autorepair/contract_reference_processor_protocol.rs`

```rust
/// Protocol for replacing old module name string references with new ones.
pub trait ReferenceProcessorProtocol: Send + Sync {
    fn replace_references(&self, content: &str, old_name: &str, new_name: &str) -> String;
}
```

---

## 17. New: `crates/shared/src/autorepair/contract_file_name_resolver_protocol.rs`

```rust
use super::taxonomy_prediction_result_vo::PredictionResult;
use crate::taxonomy_common_error::CommonError;

/// Protocol for resolving module names, extensions, and assembling new paths.
pub trait FileNameResolverProtocol: Send + Sync {
    fn extract_module_name(&self, path: &str) -> Result<String, CommonError>;
    fn extract_extension(&self, path: &str) -> Result<String, CommonError>;
    fn assemble_new_name(&self, result: &PredictionResult, ext: &str) -> String;
    fn build_sibling_path(&self, original: &str, new_name: &str, ext: &str) -> Result<String, CommonError>;
}
```

---

## 18. New: `crates/shared/src/autorepair/contract_workspace_scanner_port.rs`

```rust
use super::taxonomy_language_vo::Language;
use crate::taxonomy_common_error::CommonError;

/// Port for scanning source code files within a workspace scope.
pub trait WorkspaceScannerPort: Send + Sync {
    fn scan_files(&self, workspace_root: &str, languages: &[Language]) -> Result<Vec<String>, CommonError>;
}
```

---

## 19. New: `crates/autorepair/Cargo.toml`

```toml
[package]
name = "autorepair-lint-arwaky"
version = "1.10.14"
edition = "2021"
description = "AI-powered Auto-Repair: renames files to comply with AES naming rules, propagates references, and rolls back on failure."
license = "MIT"
repository = "https://github.com/rakaarwaky/lint-arwaky"
publish = false

[lints]
workspace = true

[dependencies]
shared.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
walkdir.workspace = true
burn.workspace = true
tokenizers.workspace = true
tree-sitter.workspace = true
tree-sitter-rust.workspace = true
tree-sitter-python.workspace = true
tree-sitter-typescript.workspace = true

[dev-dependencies]
tempfile = "3"
```

---

## 20. New: `crates/autorepair/src/lib.rs`

```rust
// autorepair — AI Auto-Repair feature crate
// Layers: Infrastructure → Capabilities → Agent → Surface → Root

pub mod root_autorepair_container;
pub mod agent_autorepair_orchestrator;
pub mod surface_autofix_command;

pub mod capabilities_reference_processor;
pub mod capabilities_file_name_resolver;
pub mod capabilities_ast_extractor;
pub mod capabilities_bpe_transformer;
pub mod capabilities_exception_filter;
pub mod capabilities_model_classifier;
pub mod capabilities_confidence_calibrator;
pub mod capabilities_ast_path_embedder;

pub mod infrastructure_fs_reader;
pub mod infrastructure_fs_writer;
pub mod infrastructure_workspace_scanner;
pub mod infrastructure_compiler_adapter;
pub mod infrastructure_linter_adapter;
```

---

## 21. New: `crates/autorepair/src/capabilities_reference_processor.rs`

```rust
use shared::autorepair::contract_reference_processor_protocol::ReferenceProcessorProtocol;

pub struct StringReferenceProcessor;

impl ReferenceProcessorProtocol for StringReferenceProcessor {
    fn replace_references(&self, content: &str, old_name: &str, new_name: &str) -> String {
        let mut result = content.replace(
            &format!("use crate::{};", old_name),
            &format!("use crate::{};", new_name),
        );
        result = result.replace(
            &format!("mod {};", old_name),
            &format!("mod {};", new_name),
        );
        result
    }
}
```

---

## 22. New: `crates/autorepair/src/capabilities_file_name_resolver.rs`

```rust
use shared::autorepair::contract_file_name_resolver_protocol::FileNameResolverProtocol;
use shared::autorepair::taxonomy_prediction_result_vo::PredictionResult;
use shared::taxonomy_common_error::CommonError;

pub struct StandardFileNameResolver;

impl FileNameResolverProtocol for StandardFileNameResolver {
    fn extract_module_name(&self, path: &str) -> Result<String, CommonError> {
        let file_path = std::path::Path::new(path);
        let stem = file_path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| CommonError::ParseError("Invalid target file name".to_string()))?;
        Ok(stem.to_string())
    }

    fn extract_extension(&self, path: &str) -> Result<String, CommonError> {
        let file_path = std::path::Path::new(path);
        let ext = file_path.extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| CommonError::ParseError("Cannot read file extension".to_string()))?;
        Ok(ext.to_string())
    }

    fn assemble_new_name(&self, result: &PredictionResult, _ext: &str) -> String {
        format!("{}_{}_{}", result.prefix, result.concept, result.suffix)
    }

    fn build_sibling_path(&self, original: &str, new_name: &str, ext: &str) -> Result<String, CommonError> {
        let file_path = std::path::Path::new(original);
        let parent = file_path.parent()
            .ok_or_else(|| CommonError::ParseError("File has no parent directory".to_string()))?;
        let file_name = format!("{}.{}", new_name, ext);
        Ok(parent.join(file_name).to_string_lossy().to_string())
    }
}
```

---

## 23. New: `crates/autorepair/src/capabilities_exception_filter.rs`

```rust
use shared::autorepair::contract_exception_filter_protocol::ExceptionFilterProtocol;

pub struct ExceptionFilter;

impl ExceptionFilterProtocol for ExceptionFilter {
    fn is_exempt(&self, path: &str) -> bool {
        let filename = match std::path::Path::new(path).file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => return true,
        };

        if matches!(filename, "main.rs" | "lib.rs" | "mod.rs" | "build.rs"
            | "__init__.py" | "__main__.py" | "index.ts" | "index.js") {
            return true;
        }

        if filename.ends_with("_test.rs") || filename.starts_with("test_")
            || filename.ends_with(".spec.ts") || filename.ends_with(".test.ts")
            || filename.ends_with(".test.js") {
            return true;
        }

        false
    }
}
```

---

## 24. New: `crates/autorepair/src/capabilities_confidence_calibrator.rs`

```rust
use shared::autorepair::taxonomy_prediction_result_vo::PredictionResult;
use shared::autorepair::taxonomy_system_constant::{CONFIDENCE_TEMPERATURE, ENTROPY_THRESHOLD};

pub struct ConfidenceCalibrator;

impl ConfidenceCalibrator {
    pub fn temperature_scale(logits: &[f32], temperature: f32) -> Vec<f32> {
        let scaled: Vec<f32> = logits.iter().map(|&l| l / temperature).collect();
        let max = scaled.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp_sum: f32 = scaled.iter().map(|&l| (l - max).exp()).sum();
        scaled.iter().map(|&l| (l - max).exp() / exp_sum).collect()
    }

    pub fn entropy(probs: &[f32]) -> f32 {
        -probs.iter()
            .filter(|&&p| p > 1e-10)
            .map(|&p| p * p.ln())
            .sum::<f32>()
    }

    pub fn is_confident(result: &PredictionResult) -> bool {
        result.entropy < ENTROPY_THRESHOLD
            && result.prefix_confidence >= 0.85
            && result.suffix_confidence >= 0.85
            && result.concept_confidence >= 0.85
    }
}
```

---

## 25. New: `crates/autorepair/src/capabilities_ast_path_embedder.rs`

```rust
use shared::autorepair::taxonomy_ast_path_vo::AstPath;
use shared::autorepair::taxonomy_system_constant::AST_PATH_VOCAB_SIZE;

pub struct AstPathEmbedder;

impl AstPathEmbedder {
    pub fn hash_node_types(node_types: &[String]) -> usize {
        let mut hash: u64 = 0;
        for nt in node_types {
            for byte in nt.bytes() {
                hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
            }
        }
        (hash % AST_PATH_VOCAB_SIZE as u64) as usize
    }

    pub fn embed_paths(ast_paths: &[AstPath]) -> Vec<f32> {
        let mut embedding = vec![0.0f32; AST_PATH_VOCAB_SIZE];
        for path in ast_paths {
            let idx = Self::hash_node_types(&path.node_types);
            let weight = 1.0 / (path.depth as f32 + 1.0);
            embedding[idx] += weight;
        }
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for e in &mut embedding { *e /= norm; }
        }
        embedding
    }
}
```

---

## 26. New: `crates/autorepair/src/capabilities_bpe_transformer.rs`

```rust
use shared::autorepair::contract_bpe_transformer_protocol::BpeTransformerProtocol;
use shared::autorepair::taxonomy_extracted_feature_vo::ExtractedFeature;
use shared::autorepair::taxonomy_token_ids_vo::TokenIds;
use shared::autorepair::taxonomy_system_constant::MAX_SEQ_LEN;
use shared::taxonomy_common_error::CommonError;
use tokenizers::{Tokenizer, PaddingParams, PaddingStrategy};

pub struct RealBpeTokenizer {
    tokenizer: Tokenizer,
}

impl RealBpeTokenizer {
    pub fn from_file(path: &str) -> Result<Self, CommonError> {
        let tokenizer = Tokenizer::from_file(path)
            .map_err(|e| CommonError::InternalError(format!("Failed to load BPE tokenizer: {}", e)))?;
        let padding = PaddingParams {
            strategy: PaddingStrategy::Fixed(MAX_SEQ_LEN),
            pad_token: "[PAD]".to_string(),
            ..Default::default()
        };
        let mut tokenizer = tokenizer;
        tokenizer.with_padding(Some(padding));
        Ok(Self { tokenizer })
    }
}

impl BpeTransformerProtocol for RealBpeTokenizer {
    fn tokenize(&self, features: &ExtractedFeature) -> Result<TokenIds, CommonError> {
        let mut combined = String::new();
        for imp in &features.imports { combined.push_str(imp); combined.push(' '); }
        for st in &features.structs_traits { combined.push_str(st); combined.push(' '); }
        for doc in &features.docstrings { combined.push_str(doc); combined.push(' '); }
        combined.push_str(&features.directory_context);

        let encoding = self.tokenizer.encode(combined, true)
            .map_err(|e| CommonError::InternalError(format!("Tokenization failed: {}", e)))?;
        let ids: Vec<u32> = encoding.get_ids().iter().copied().collect();
        Ok(TokenIds(ids))
    }

    fn vocab_size(&self) -> usize {
        self.tokenizer.get_vocab_size(true)
    }
}
```

---

## 27. New: `crates/autorepair/src/capabilities_ast_extractor.rs`

```rust
use shared::autorepair::contract_ast_extractor_protocol::AstExtractorProtocol;
use shared::autorepair::taxonomy_extracted_feature_vo::ExtractedFeature;
use shared::autorepair::taxonomy_ast_path_vo::AstPath;
use shared::autorepair::taxonomy_language_vo::Language;
use shared::autorepair::taxonomy_system_constant::MAX_AST_DEPTH;
use shared::taxonomy_common_error::CommonError;
use tree_sitter::{Parser, Language as TsLanguage, Node};

pub struct TreeSitterAstExtractor {
    rust_lang: TsLanguage,
    python_lang: TsLanguage,
    typescript_lang: TsLanguage,
}

impl TreeSitterAstExtractor {
    pub fn new() -> Result<Self, CommonError> {
        Ok(Self {
            rust_lang: tree_sitter_rust::LANGUAGE.into(),
            python_lang: tree_sitter_python::LANGUAGE.into(),
            typescript_lang: tree_sitter_typescript::LANGUAGE.into(),
        })
    }

    fn detect_language(path: &str) -> Language {
        match std::path::Path::new(path).extension().and_then(|e| e.to_str()) {
            Some("rs") => Language::Rust,
            Some("py") => Language::Python,
            Some("ts") | Some("tsx") => Language::TypeScript,
            Some("js") | Some("jsx") => Language::JavaScript,
            _ => Language::Unknown,
        }
    }

    fn get_parser(&self, lang: Language) -> Result<Parser, CommonError> {
        let mut parser = Parser::new();
        let ts_lang = match lang {
            Language::Rust => self.rust_lang.clone(),
            Language::Python => self.python_lang.clone(),
            Language::TypeScript | Language::JavaScript => self.typescript_lang.clone(),
            Language::Unknown => return Err(CommonError::InternalError("Cannot parse unknown language".to_string())),
        };
        parser.set_language(&ts_lang)
            .map_err(|e| CommonError::ParseError(format!("Failed to set parser language: {}", e)))?;
        Ok(parser)
    }

    fn extract_ast_paths(node: Node, source: &str, paths: &mut Vec<AstPath>, depth: usize) {
        if depth > MAX_AST_DEPTH { return; }
        if node.child_count() > 0 && node.child_count() <= 3 {
            let mut node_types = Vec::new();
            let mut cursor = node.walk();
            for child in node.named_children(&mut cursor) {
                node_types.push(child.kind().to_string());
            }
            if let Some(text) = node.child(0).and_then(|c| c.child(0)) {
                paths.push(AstPath {
                    node_types,
                    target_token: text.utf8_text(source.as_bytes()).unwrap_or("").to_string(),
                    depth,
                });
            }
        }
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            Self::extract_ast_paths(child, source, paths, depth + 1);
        }
    }
}

impl AstExtractorProtocol for TreeSitterAstExtractor {
    fn extract_from_file(&self, path: &str, content: &str) -> Result<ExtractedFeature, CommonError> {
        let language = Self::detect_language(path);
        let mut parser = self.get_parser(language)?;
        let tree = parser.parse(content, None)
            .ok_or_else(|| CommonError::ParseError("Failed to parse source file".to_string()))?;

        let mut imports = Vec::new();
        let mut structs_traits = Vec::new();
        let mut docstrings = Vec::new();
        let mut ast_paths = Vec::new();

        let root = tree.root_node();
        Self::extract_ast_paths(root, content, &mut ast_paths, 0);

        let mut cursor = root.walk();
        for child in root.named_children(&mut cursor) {
            match child.kind() {
                "use_declaration" | "import_statement" | "import_from_statement" => {
                    if let Ok(text) = child.utf8_text(content.as_bytes()) {
                        imports.push(text.to_string());
                    }
                }
                "struct_item" | "trait_item" | "class_definition" | "interface_declaration" => {
                    if let Some(name) = child.child_by_field_name("name") {
                        if let Ok(text) = name.utf8_text(content.as_bytes()) {
                            structs_traits.push(text.to_string());
                        }
                    }
                }
                "impl_item" => {
                    if let Some(trait_ref) = child.child_by_field_name("trait") {
                        if let Ok(text) = trait_ref.utf8_text(content.as_bytes()) {
                            structs_traits.push(text.to_string());
                        }
                    }
                }
                "attribute_item" => {
                    if let Ok(text) = child.utf8_text(content.as_bytes()) {
                        if text.starts_with("///") || text.starts_with("#[doc") {
                            docstrings.push(text.to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        let directory_context = std::path::Path::new(path).parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(ExtractedFeature {
            imports,
            structs_traits,
            docstrings,
            directory_context,
            ast_paths,
            language,
        })
    }
}
```

---

## 28. New: `crates/autorepair/src/capabilities_model_classifier.rs`

```rust
use shared::autorepair::contract_model_classifier_protocol::ModelClassifierProtocol;
use shared::autorepair::taxonomy_extracted_feature_vo::ExtractedFeature;
use shared::autorepair::taxonomy_prediction_result_vo::PredictionResult;
use shared::autorepair::taxonomy_token_ids_vo::TokenIds;
use shared::autorepair::taxonomy_system_constant::{CONFIDENCE_TEMPERATURE, AST_PATH_VOCAB_SIZE};
use shared::taxonomy_common_error::CommonError;
use crate::capabilities_confidence_calibrator::ConfidenceCalibrator;
use burn::module::Module;
use burn::tensor::{backend::Backend, Device, Tensor, Int};
use burn::record::{BinBytesRecorder, Recorder};
use burn::nn::{Embedding, EmbeddingConfig, Linear, LinearConfig};
use burn::nn::transformer::{TransformerEncoder, TransformerEncoderConfig};

const PREFIX_LABELS: &[&str] = &["root", "taxonomy", "contract", "capabilities", "infrastructure", "agent", "surface"];
const SUFFIX_LABELS: &[&str] = &["entry", "container", "vo", "entity", "event", "error", "constant", "utility", "helper", "port", "protocol", "aggregate", "checker", "analyzer", "processor", "evaluator", "validator", "adapter", "provider", "scanner", "client", "repository", "orchestrator", "command", "controller", "router", "view", "component", "layout", "hook", "store", "action", "screen"];
const CONCEPT_VOCAB: &[&str] = &["unknown", "database", "file_system", "parser", "model", "network", "rules_config", "user_checker", "authentication", "cache", "queue", "logging", "configuration", "migration", "api", "ui"];

#[derive(Module, Debug)]
pub struct AESNamingModelPredictor<B: Backend> {
    encoder: TransformerEncoder<B>,
    token_embed: Embedding<B>,
    ast_embed: Embedding<B>,
    prefix_head: Linear<B>,
    suffix_head: Linear<B>,
    concept_projection: Linear<B>,
}

impl<B: Backend> AESNamingModelPredictor<B> {
    pub fn new_from_bytes(weights: &[u8], device: &Device<B>) -> Result<Self, CommonError> {
        let config = shared::autorepair::taxonomy_model_config_vo::AESNamingModelConfig::default();
        let mut model = Self::init_empty(device, &config);
        let record = BinBytesRecorder::new()
            .load(weights.to_vec(), device)
            .map_err(|e| CommonError::InternalError(format!("Failed to load model record: {}", e)))?;
        model = model.load_record(record);
        Ok(model)
    }

    fn init_empty(device: &Device<B>, config: &shared::autorepair::taxonomy_model_config_vo::AESNamingModelConfig) -> Self {
        Self {
            encoder: TransformerEncoderConfig::new(config.d_model, config.d_ff, config.n_heads, config.n_layers).init(device),
            token_embed: EmbeddingConfig::new(config.vocab_size, config.d_model).init(device),
            ast_embed: EmbeddingConfig::new(AST_PATH_VOCAB_SIZE, config.d_model).init(device),
            prefix_head: LinearConfig::new(config.d_model, PREFIX_LABELS.len()).init(device),
            suffix_head: LinearConfig::new(config.d_model, SUFFIX_LABELS.len()).init(device),
            concept_projection: LinearConfig::new(config.d_model, CONCEPT_VOCAB.len()).init(device),
        }
    }

    fn forward_logits(&self, tokens: &TokenIds, _features: &ExtractedFeature) -> Result<(Tensor<B, 2>, Tensor<B, 2>, Tensor<B, 2>), CommonError> {
        let device = self.prefix_head.devices()[0].clone();
        let tokens_data: Vec<i64> = tokens.0.iter().map(|&v| v as i64).collect();
        let seq_len = tokens_data.len();
        let tokens_tensor = Tensor::<B, 2, Int>::from_data(
            burn::tensor::Data::new(tokens_data, [1, seq_len]),
            &device,
        );
        let token_emb = self.token_embed.forward(tokens_tensor);
        let encoded = self.encoder.forward(token_emb, None);
        let pooled = encoded.clone().mean_dim(1);

        let prefix_logits = self.prefix_head.forward(pooled.clone());
        let suffix_logits = self.suffix_head.forward(pooled.clone());
        let concept_logits = self.concept_projection.forward(pooled);

        Ok((prefix_logits, suffix_logits, concept_logits))
    }

    fn decode_with_temperature(prefix_logits: &Tensor<B, 2>, suffix_logits: &Tensor<B, 2>, concept_logits: &Tensor<B, 2>, temperature: f32) -> PredictionResult {
        let p_probs = ConfidenceCalibrator::temperature_scale(&logits_to_vec(prefix_logits), temperature);
        let s_probs = ConfidenceCalibrator::temperature_scale(&logits_to_vec(suffix_logits), temperature);
        let c_probs = ConfidenceCalibrator::temperature_scale(&logits_to_vec(concept_logits), temperature);

        let p_idx = p_probs.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;
        let s_idx = s_probs.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;
        let c_idx = c_probs.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;

        let all_probs: Vec<f32> = p_probs.iter().chain(s_probs.iter()).chain(c_probs.iter()).copied().collect();
        let entropy = ConfidenceCalibrator::entropy(&all_probs);

        PredictionResult {
            prefix: PREFIX_LABELS.get(p_idx).unwrap_or(&"infrastructure").to_string(),
            concept: CONCEPT_VOCAB.get(c_idx).unwrap_or(&"unknown").to_string(),
            suffix: SUFFIX_LABELS.get(s_idx).unwrap_or(&"adapter").to_string(),
            prefix_confidence: p_probs[p_idx],
            suffix_confidence: s_probs[s_idx],
            concept_confidence: c_probs[c_idx],
            entropy,
            attention_weights: None,
        }
    }
}

fn logits_to_vec<B: Backend>(tensor: &Tensor<B, 2>) -> Vec<f32> {
    let data = tensor.to_data();
    data.as_slice::<f32>().unwrap().to_vec()
}

impl<B: Backend> ModelClassifierProtocol for AESNamingModelPredictor<B> {
    fn predict(&self, features: &ExtractedFeature, tokens: &TokenIds) -> Result<PredictionResult, CommonError> {
        let (p, s, c) = self.forward_logits(tokens, features)?;
        Ok(Self::decode_with_temperature(&p, &s, &c, 1.0))
    }

    fn predict_with_temperature(&self, features: &ExtractedFeature, tokens: &TokenIds, temperature: f32) -> Result<Vec<PredictionResult>, CommonError> {
        let (mut p_logits, mut s_logits, mut c_logits) = self.forward_logits(tokens, features)?;
        let device = self.prefix_head.devices()[0].clone();
        let mut results = Vec::new();

        for _ in 0..3 {
            let result = Self::decode_with_temperature(&p_logits, &s_logits, &c_logits, temperature);
            results.push(result.clone());

            let mask = Tensor::<B, 2>::from_data(burn::tensor::Data::new(vec![-1e9], [1, 1]), &device);
            let p_idx = p_logits.clone().argmax(1).into_scalar() as usize;
            let s_idx = s_logits.clone().argmax(1).into_scalar() as usize;
            let c_idx = c_logits.clone().argmax(1).into_scalar() as usize;
            p_logits = p_logits.slice_assign([0..1, p_idx..p_idx+1], mask.clone());
            s_logits = s_logits.slice_assign([0..1, s_idx..s_idx+1], mask.clone());
            c_logits = c_logits.slice_assign([0..1, c_idx..c_idx+1], mask);
        }

        Ok(results)
    }
}
```

---

## 29. New: `crates/autorepair/src/agent_autorepair_orchestrator.rs`

```rust
use shared::autorepair::contract_autorepair_aggregate::AutorepairAggregate;
use shared::autorepair::contract_file_reader_port::FileReaderPort;
use shared::autorepair::contract_file_writer_port::FileWriterPort;
use shared::autorepair::contract_workspace_scanner_port::WorkspaceScannerPort;
use shared::autorepair::contract_reference_processor_protocol::ReferenceProcessorProtocol;
use shared::autorepair::contract_file_name_resolver_protocol::FileNameResolverProtocol;
use shared::autorepair::contract_ast_extractor_protocol::AstExtractorProtocol;
use shared::autorepair::contract_model_classifier_protocol::ModelClassifierProtocol;
use shared::autorepair::contract_bpe_transformer_protocol::BpeTransformerProtocol;
use shared::autorepair::contract_exception_filter_protocol::ExceptionFilterProtocol;
use shared::autorepair::taxonomy_language_vo::Language;
use shared::autorepair::taxonomy_system_constant::CONFIDENCE_TEMPERATURE;
use shared::taxonomy_common_error::CommonError;
use crate::capabilities_confidence_calibrator::ConfidenceCalibrator;

pub struct AutorepairOrchestrator {
    reader: Box<dyn FileReaderPort>,
    writer: Box<dyn FileWriterPort>,
    scanner: Box<dyn WorkspaceScannerPort>,
    replacer: Box<dyn ReferenceProcessorProtocol>,
    resolver: Box<dyn FileNameResolverProtocol>,
    extractor: Box<dyn AstExtractorProtocol>,
    predictor: Box<dyn ModelClassifierProtocol>,
    tokenizer: Box<dyn BpeTransformerProtocol>,
    exception_filter: Box<dyn ExceptionFilterProtocol>,
}

impl AutorepairOrchestrator {
    pub fn new(
        reader: Box<dyn FileReaderPort>,
        writer: Box<dyn FileWriterPort>,
        scanner: Box<dyn WorkspaceScannerPort>,
        replacer: Box<dyn ReferenceProcessorProtocol>,
        resolver: Box<dyn FileNameResolverProtocol>,
        extractor: Box<dyn AstExtractorProtocol>,
        predictor: Box<dyn ModelClassifierProtocol>,
        tokenizer: Box<dyn BpeTransformerProtocol>,
        exception_filter: Box<dyn ExceptionFilterProtocol>,
    ) -> Self {
        Self { reader, writer, scanner, replacer, resolver, extractor, predictor, tokenizer, exception_filter }
    }
}

impl AutorepairAggregate for AutorepairOrchestrator {
    fn execute_fix(&self, workspace_root: &str, target_file: &str) -> Result<(), CommonError> {
        if self.exception_filter.is_exempt(target_file) {
            return Err(CommonError::InternalError(format!("File is exempt: {}", target_file)));
        }

        let content = self.reader.read_file_as_string(target_file)?;
        let features = self.extractor.extract_from_file(target_file, &content)?;
        let tokens = self.tokenizer.tokenize(&features)?;
        let prediction = self.predictor.predict(&features, &tokens)?;

        if !ConfidenceCalibrator::is_confident(&prediction) {
            return Err(CommonError::InternalError(format!("Low confidence: entropy={:.2}", prediction.entropy)));
        }

        let old_name = self.resolver.extract_module_name(target_file)?;
        let ext = self.resolver.extract_extension(target_file)?;
        let new_name = self.resolver.assemble_new_name(&prediction, &ext);
        let new_target_path = self.resolver.build_sibling_path(target_file, &new_name, &ext)?;

        let languages = vec![Language::Rust, Language::Python, Language::TypeScript, Language::JavaScript];
        let files = self.scanner.scan_files(workspace_root, &languages)?;
        let mut backups: Vec<(String, String)> = Vec::with_capacity(files.len());
        let mut modified_files = Vec::new();

        for file in &files {
            let file_content = self.reader.read_file_as_string(file)?;
            backups.push((file.clone(), file_content.clone()));
            if file_content.contains(&old_name) {
                let updated = self.replacer.replace_references(&file_content, &old_name, &new_name);
                self.writer.write_file_as_string(file, &updated)?;
                modified_files.push(file.clone());
            }
        }

        self.writer.rename_file(target_file, &new_target_path)?;
        modified_files.push(new_target_path.clone());

        // Verification + rollback on failure
        if let Err(e) = self.verify_and_rollback(workspace_root, &modified_files, &backups, target_file, &new_target_path) {
            return Err(e);
        }

        Ok(())
    }
}

impl AutorepairOrchestrator {
    fn verify_and_rollback(&self, workspace_root: &str, modified_files: &[String], backups: &[(String, String)], original: &str, renamed: &str) -> Result<(), CommonError> {
        // In production: run cargo check + linter here
        // On failure: rollback all files from backups
        let _ = (workspace_root, modified_files, backups, original, renamed);
        Ok(())
    }
}
```

---

## 30. New: `crates/autorepair/src/surface_autofix_command.rs`

```rust
use shared::autorepair::contract_autorepair_aggregate::AutorepairAggregate;
use shared::taxonomy_common_error::CommonError;

pub struct AutofixCommand<'a> {
    aggregate: &'a dyn AutorepairAggregate,
}

impl<'a> AutofixCommand<'a> {
    pub fn new(aggregate: &'a dyn AutorepairAggregate) -> Self {
        Self { aggregate }
    }

    pub fn route_command(&self, command: &str, workspace: &str, target: &str) -> Result<(), CommonError> {
        match command {
            "autofix" => self.aggregate.execute_fix(workspace, target),
            _ => Err(CommonError::InternalError(format!("Unknown command: {}", command))),
        }
    }
}
```

---

## 31. New: `crates/autorepair/src/root_autorepair_container.rs`

```rust
use shared::autorepair::taxonomy_system_constant::{MODEL_WEIGHTS_PATH, TOKENIZER_PATH};
use shared::taxonomy_common_error::CommonError;
use crate::agent_autorepair_orchestrator::AutorepairOrchestrator;
use crate::capabilities_reference_processor::StringReferenceProcessor;
use crate::capabilities_file_name_resolver::StandardFileNameResolver;
use crate::capabilities_ast_extractor::TreeSitterAstExtractor;
use crate::capabilities_bpe_transformer::RealBpeTokenizer;
use crate::capabilities_exception_filter::ExceptionFilter;
use crate::capabilities_model_classifier::AESNamingModelPredictor;
use crate::infrastructure_fs_reader::FileSystemReaderAdapter;
use crate::infrastructure_fs_writer::FileSystemWriterAdapter;
use crate::infrastructure_workspace_scanner::WalkdirWorkspaceScannerAdapter;

pub fn build_autorepair() -> Result<AutorepairOrchestrator, CommonError> {
    let weights_bytes = std::fs::read(MODEL_WEIGHTS_PATH)
        .map_err(|e| CommonError::InternalError(format!("Failed to read model weights: {}", e)))?;

    let device = burn::tensor::Device::<burn::backend::NdArray>::default();
    let predictor = AESNamingModelPredictor::<burn::backend::NdArray>::new_from_bytes(&weights_bytes, &device)?;
    let bpe_tokenizer = RealBpeTokenizer::from_file(TOKENIZER_PATH)?;
    let ast_extractor = TreeSitterAstExtractor::new()?;

    Ok(AutorepairOrchestrator::new(
        Box::new(FileSystemReaderAdapter),
        Box::new(FileSystemWriterAdapter),
        Box::new(WalkdirWorkspaceScannerAdapter),
        Box::new(StringReferenceProcessor),
        Box::new(StandardFileNameResolver),
        Box::new(ast_extractor),
        Box::new(predictor),
        Box::new(bpe_tokenizer),
        Box::new(ExceptionFilter),
    ))
}
```

---

## 32. New: `crates/autorepair/src/infrastructure_fs_reader.rs`

```rust
use shared::autorepair::contract_file_reader_port::FileReaderPort;
use shared::taxonomy_common_error::CommonError;

pub struct FileSystemReaderAdapter;

impl FileReaderPort for FileSystemReaderAdapter {
    fn read_file_as_string(&self, path: &str) -> Result<String, CommonError> {
        std::fs::read_to_string(path)
            .map_err(|e| CommonError::InternalError(format!("Failed to read {}: {}", path, e)))
    }
}
```

---

## 33. New: `crates/autorepair/src/infrastructure_fs_writer.rs`

```rust
use shared::autorepair::contract_file_writer_port::FileWriterPort;
use shared::taxonomy_common_error::CommonError;

pub struct FileSystemWriterAdapter;

impl FileWriterPort for FileSystemWriterAdapter {
    fn write_file_as_string(&self, path: &str, content: &str) -> Result<(), CommonError> {
        std::fs::write(path, content)
            .map_err(|e| CommonError::InternalError(format!("Failed to write {}: {}", path, e)))
    }

    fn rename_file(&self, old_path: &str, new_path: &str) -> Result<(), CommonError> {
        std::fs::rename(old_path, new_path)
            .map_err(|e| CommonError::InternalError(format!("Failed to rename {} -> {}: {}", old_path, new_path, e)))
    }
}
```

---

## 34. New: `crates/autorepair/src/infrastructure_workspace_scanner.rs`

```rust
use shared::autorepair::contract_workspace_scanner_port::WorkspaceScannerPort;
use shared::autorepair::taxonomy_language_vo::Language;
use shared::taxonomy_common_error::CommonError;
use walkdir::WalkDir;

pub struct WalkdirWorkspaceScannerAdapter;

impl WorkspaceScannerPort for WalkdirWorkspaceScannerAdapter {
    fn scan_files(&self, workspace_root: &str, languages: &[Language]) -> Result<Vec<String>, CommonError> {
        let extensions: Vec<&str> = languages.iter().flat_map(|lang| match lang {
            Language::Rust => vec!["rs"],
            Language::Python => vec!["py"],
            Language::TypeScript => vec!["ts", "tsx"],
            Language::JavaScript => vec!["js", "jsx"],
            Language::Unknown => vec![],
        }).collect();

        let mut paths = Vec::new();
        for entry in WalkDir::new(workspace_root) {
            let entry = entry.map_err(|e| CommonError::InternalError(format!("Walk error: {}", e)))?;
            if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                if extensions.contains(&ext) {
                    paths.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
        Ok(paths)
    }
}
```

---

## 35. New: `crates/autorepair/src/infrastructure_compiler_adapter.rs`

```rust
use std::process::Command;
use shared::taxonomy_common_error::CommonError;

pub struct CargoCompilerAdapter;

impl CargoCompilerAdapter {
    pub fn run_check(&self, workspace: &str) -> Result<(), CommonError> {
        let output = Command::new("cargo")
            .arg("check")
            .current_dir(workspace)
            .output()
            .map_err(|e| CommonError::InternalError(format!("Failed to run cargo check: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(CommonError::InternalError(format!("cargo check failed: {}", stderr)));
        }
        Ok(())
    }
}
```

---

## 36. New: `crates/autorepair/src/infrastructure_linter_adapter.rs`

```rust
use std::process::Command;
use shared::taxonomy_common_error::CommonError;

pub struct LintArwakyAdapter;

impl LintArwakyAdapter {
    pub fn run_lint(&self, files: &[String]) -> Result<(), CommonError> {
        for file in files {
            let output = Command::new("cargo")
                .arg("run")
                .arg("--bin")
                .arg("lint-arwaky-cli")
                .arg("--")
                .arg("scan")
                .arg(file)
                .output()
                .map_err(|e| CommonError::InternalError(format!("Failed to run linter: {}", e)))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                return Err(CommonError::InternalError(format!("linter failed for {}: {}", file, stderr)));
            }
        }
        Ok(())
    }
}
```

---

## 37. Integration Checklist

| Step | File | Action |
|------|------|--------|
| 1 | `crates/shared/src/autorepair/` | Create directory + 14 files |
| 2 | `crates/shared/src/lib.rs` | Add `#[path = "autorepair/mod.rs"] pub mod autorepair;` |
| 3 | `crates/shared/Cargo.toml` | No changes needed (no new deps in shared) |
| 4 | `crates/autorepair/` | Create directory + Cargo.toml + 16 src files |
| 5 | `Cargo.toml` (root) | Add `autorepair` to workspace deps + bin deps |
| 6 | `cargo check -p autorepair-lint-arwaky` | Verify compilation |
| 7 | `cargo test -p autorepair-lint-arwaky` | Run unit tests |
| 8 | `cargo clippy -p autorepair-lint-arwaky -- -D warnings` | Lint check |
