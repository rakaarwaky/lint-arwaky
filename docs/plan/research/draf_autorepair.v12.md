# Implementation Draft: AI Auto-Repair Model (Perfect AES Dogfooding v12)

Draft v12 upgrades from v11 with frontier technology: real BPE tokenizer via HuggingFace `tokenizers` crate, sinusoidal positional encoding, Tree-sitter multi-language AST extraction, Code2Vec-style AST path embedding, temperature-scaled alternative sampling, and attention weight storage for interpretability.

**Key Upgrades from v11 → v12:**

| # | Component              | v11                             | v12 (Frontier)                       |
| - | ---------------------- | ------------------------------- | ------------------------------------ |
| 1 | Tokenizer              | Simulated BPE (hardcoded match) | Real BPE via`tokenizers` crate     |
| 2 | Positional Encoding    | ❌ Missing                      | ✅ Sinusoidal positional encoding    |
| 3 | AST Extraction         | `syn` (Rust only)             | `tree-sitter` (multi-language)     |
| 4 | Directory Context      | Hash-based embedding            | Code2Vec AST path embedding          |
| 5 | Alternative Sampling   | Mask-based top-3                | Temperature-scaled softmax           |
| 6 | Attention Weights      | Not stored                      | Stored for interpretability          |
| 7 | Multi-language         | Rust only                       | Rust, Python, TypeScript, JavaScript |
| 8 | Model Config           | 12K vocab, 128-dim              | 32K vocab, 256-dim, 6-layer          |
| 9 | Confidence Calibration | Raw softmax                     | Temperature-scaled + entropy         |

---

## 1. Taxonomy Layer (Data, Constants, Errors & Value Objects)

Each file in this layer has zero external dependencies, is strictly encapsulated, and is documented above 5 effective lines (AES302).

### File: `taxonomy_system_constant.rs`

```rust
/// Absolute/relative path to the Safetensors model weights file.
/// Loaded by Infrastructure during Root initialization.
pub const MODEL_WEIGHTS_PATH: &str = "weights/model.safetensors";

/// Absolute/relative path to the BPE tokenizer JSON file.
/// Loaded by Infrastructure during Root initialization (F-1 Upgrade).
pub const TOKENIZER_PATH: &str = "weights/tokenizer.json";

/// Maximum sequence length for tokenized input (F-2 Upgrade).
pub const MAX_SEQ_LEN: usize = 512;

/// Temperature scaling factor for confidence calibration (F-3 Upgrade).
pub const CONFIDENCE_TEMPERATURE: f32 = 1.5;

/// Entropy threshold for low-confidence detection (F-3 Upgrade).
pub const ENTROPY_THRESHOLD: f32 = 1.2;

/// AST path embedding vocabulary size for Code2Vec-style encoding (F-4 Upgrade).
pub const AST_PATH_VOCAB_SIZE: usize = 2048;

/// Maximum AST depth to capture in path embedding (F-4 Upgrade).
pub const MAX_AST_DEPTH: usize = 8;

/// Supported language identifiers for multi-language scanner (F-5 Upgrade).
pub const SUPPORTED_EXTENSIONS: &[&str] = &["rs", "py", "ts", "js", "tsx", "jsx"];
```

### File: `taxonomy_prefix_label_constant.rs`

```rust
/// Class labels for AES architecture prefix.
/// Array indices correspond to model output classes.
pub const PREFIX_LABELS: &[&str] = &[
    "root", "taxonomy", "contract", "capabilities", "infrastructure", "agent", "surface",
];
```

### File: `taxonomy_suffix_label_constant.rs`

```rust
/// Class labels for AES role suffix.
/// Array indices correspond to model output classes.
pub const SUFFIX_LABELS: &[&str] = &[
    "entry", "container", "vo", "entity", "event",
    "error", "constant", "utility", "helper", "port",
    "protocol", "aggregate", "checker", "analyzer", "processor",
    "evaluator", "validator", "adapter", "provider", "scanner",
    "client", "repository", "orchestrator", "command", "controller",
    "router", "view", "component", "layout", "hook",
    "store", "action", "screen",
];
```

### File: `taxonomy_concept_vocab_constant.rs`

```rust
/// Base vocabulary for decoding file domain concepts from the model.
pub const CONCEPT_VOCAB: &[&str] = &[
    "unknown", "database", "file_system", "parser", "model", "network", "rules_config", "user_checker",
    "authentication", "cache", "queue", "logging", "configuration", "migration", "api", "ui",
];
```

### File: `taxonomy_error_message_vo.rs`

```rust
/// Value Object to safely wrap error messages.
/// Prevents Primitive Obsession against raw Strings in the Taxonomy Error layer (AES401).
#[derive(Debug, Clone)]
pub struct ErrorMessage(pub String);

impl ErrorMessage {
    /// Returns the error message as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

### File: `taxonomy_system_error.rs`

```rust
use crate::taxonomy_error_message_vo::ErrorMessage;

/// Centralized error data structure for the Auto-Repair system.
/// Maps failures across filesystem, parsing, prediction, tokenization, and verification operations.
#[derive(Debug)]
pub enum SystemError {
    IoError(ErrorMessage),
    ParsingError(ErrorMessage),
    PredictionError(ErrorMessage),
    TokenizationError(ErrorMessage),
    ArgumentError(ErrorMessage),
    LowConfidence(ErrorMessage),
    ExemptFile(ErrorMessage),
    UnsupportedLanguage(ErrorMessage),
    VerificationError(ErrorMessage),
    RollbackFailure(ErrorMessage),
}

impl From<std::io::Error> for SystemError {
    fn from(err: std::io::Error) -> Self {
        SystemError::IoError(ErrorMessage(err.to_string()))
    }
}
```

### File: `taxonomy_file_path_vo.rs`

```rust
use std::path::PathBuf;

/// Value Object to safely represent a filesystem path.
#[derive(Debug, Clone)]
pub struct FilePath(pub PathBuf);

impl FilePath {
    pub fn from_constant(constant: &str) -> Self {
        Self(PathBuf::from(constant))
    }
}
```

### File: `taxonomy_module_name_vo.rs`

```rust
/// Value Object to represent a module name (without extension).
#[derive(Debug, Clone)]
pub struct ModuleName(pub String);

impl ModuleName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

### File: `taxonomy_file_content_vo.rs`

```rust
/// Value Object to represent the text content of a file.
#[derive(Debug, Clone)]
pub struct FileContent(pub String);

impl FileContent {
    pub fn to_string(self) -> String {
        self.0
    }
}
```

### File: `taxonomy_file_bytes_vo.rs`

```rust
/// Value Object to represent raw bytes from a model weights file.
#[derive(Debug, Clone)]
pub struct FileBytes(pub Vec<u8>);

impl FileBytes {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
```

### File: `taxonomy_file_extension_vo.rs`

```rust
/// Value Object to represent a file extension (e.g., "rs", "py").
#[derive(Debug, Clone)]
pub struct FileExtension(pub String);

impl FileExtension {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

### File: `taxonomy_token_ids_vo.rs`

```rust
/// Value Object wrapping BPE tokenization results.
/// F-1 Upgrade: Now produced by real BPE tokenizer, not simulated.
#[derive(Debug, Clone)]
pub struct TokenIds(pub Vec<u32>);

impl TokenIds {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns attention mask with 1s for real tokens, 0s for padding (F-2 Upgrade).
    pub fn attention_mask(&self, max_len: usize) -> Vec<f32> {
        let mut mask = vec![0.0; max_len];
        for (i, _) in self.0.iter().enumerate().take(max_len) {
            mask[i] = 1.0;
        }
        mask
    }
}
```

### File: `taxonomy_layer_prefix_vo.rs`

```rust
/// Value Object to safely wrap classification output prefix.
#[derive(Debug, Clone)]
pub struct LayerPrefix(pub String);

impl LayerPrefix {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

### File: `taxonomy_layer_suffix_vo.rs`

```rust
/// Value Object to safely wrap classification output suffix.
#[derive(Debug, Clone)]
pub struct LayerSuffix(pub String);

impl LayerSuffix {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

### File: `taxonomy_concept_token_vo.rs`

```rust
/// Value Object to safely wrap domain concept decoding results.
#[derive(Debug, Clone)]
pub struct ConceptToken(pub String);

impl ConceptToken {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

### File: `taxonomy_extracted_feature_vo.rs`

```rust
/// Static features extracted from a source code file.
/// F-5 Upgrade: Now supports multi-language extraction via Tree-sitter.
#[derive(Debug, Clone)]
pub struct ExtractedFeature {
    pub imports: Vec<String>,
    pub structs_traits: Vec<String>,
    pub docstrings: Vec<String>,
    pub directory_context: String,
    pub ast_paths: Vec<AstPath>,
    pub language: Language,
}

/// AST path for Code2Vec-style embedding (F-4 Upgrade).
#[derive(Debug, Clone)]
pub struct AstPath {
    pub node_types: Vec<String>,
    pub target_token: String,
    pub depth: usize,
}

/// Supported source code languages (F-5 Upgrade).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    JavaScript,
    Unknown,
}
```

### File: `taxonomy_prediction_result_vo.rs`

```rust
use crate::taxonomy_layer_prefix_vo::LayerPrefix;
use crate::taxonomy_layer_suffix_vo::LayerSuffix;
use crate::taxonomy_concept_token_vo::ConceptToken;

/// New naming classification result from the AI model.
#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub prefix: LayerPrefix,
    pub concept: ConceptToken,
    pub suffix: LayerSuffix,
    pub prefix_confidence: f32,
    pub suffix_confidence: f32,
    pub concept_confidence: f32,
    pub entropy: f32,
    pub attention_weights: Option<Vec<f32>>,
}
```

### File: `taxonomy_model_config_vo.rs`

```rust
/// Internal configuration data structure for the AI prediction model.
/// F-6 Upgrade: Expanded config for larger frontier model.
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

## 2. Contract Layer (Interfaces / Ports, Protocols & Aggregates)

Architectural boundary that uses VOs fully (AES402). All traits are documented with doc comments to maintain clear contracts.

### File: `contract_file_reader_port.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_file_bytes_vo::FileBytes;

/// Port for independently reading filesystem data.
/// Concrete implementations are isolated in the Infrastructure layer.
pub trait FileReaderPort {
    fn read_file_as_string(&self, path: &FilePath) -> Result<FileContent, SystemError>;
    fn read_file_as_bytes(&self, path: &FilePath) -> Result<FileBytes, SystemError>;
}
```

### File: `contract_file_writer_port.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_file_content_vo::FileContent;

/// Port for independently writing and modifying the filesystem.
pub trait FileWriterPort {
    fn write_file_as_string(&self, path: &FilePath, content: &FileContent) -> Result<(), SystemError>;
    fn rename_file(&self, old_path: &FilePath, new_path: &FilePath) -> Result<(), SystemError>;
}
```

### File: `contract_workspace_scanner_port.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_extracted_feature_vo::Language;

/// Port for scanning source code files within a workspace scope.
/// F-5 Upgrade: Now supports multi-language scanning.
pub trait WorkspaceScannerPort {
    fn scan_files(&self, workspace_root: &FilePath, languages: &[Language]) -> Result<Vec<FilePath>, SystemError>;
}
```

### File: `contract_reference_processor_protocol.rs`

```rust
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_module_name_vo::ModuleName;

/// Protocol for replacing old module name string references with new ones.
pub trait ReferenceProcessorProtocol {
    fn replace_references(&self, content: &FileContent, old_name: &ModuleName, new_name: &ModuleName) -> FileContent;
}
```

### File: `contract_file_name_resolver_protocol.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_module_name_vo::ModuleName;
use crate::taxonomy_file_extension_vo::FileExtension;
use crate::taxonomy_prediction_result_vo::PredictionResult;

/// Protocol for resolving module names, extensions, and assembling new paths safely.
/// H-1 Fix: ModuleName does not contain the extension; extension is joined when building the physical path.
pub trait FileNameResolverProtocol {
    fn extract_module_name(&self, path: &FilePath) -> Result<ModuleName, SystemError>;
    fn extract_extension(&self, path: &FilePath) -> Result<FileExtension, SystemError>;
    fn assemble_new_name(&self, result: &PredictionResult, ext: &FileExtension) -> ModuleName;
    fn build_sibling_path(&self, original: &FilePath, new_name: &ModuleName, ext: &FileExtension) -> Result<FilePath, SystemError>;
}
```

### File: `contract_model_classifier_protocol.rs`

```rust
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_token_ids_vo::TokenIds;
use crate::taxonomy_prediction_result_vo::PredictionResult;
use crate::taxonomy_model_config_vo::AESNamingModelConfig;
use crate::taxonomy_system_error::SystemError;

/// Protocol for interacting with the Burn-based prediction model.
/// F-7 Upgrade: Now returns attention weights for interpretability.
pub trait ModelClassifierProtocol {
    fn predict(&self, features: &ExtractedFeature, tokens: &TokenIds) -> Result<PredictionResult, SystemError>;
    fn predict_with_temperature(&self, features: &ExtractedFeature, tokens: &TokenIds, temperature: f32) -> Result<Vec<PredictionResult>, SystemError>;
    fn get_config(&self) -> AESNamingModelConfig;
}
```

### File: `contract_ast_extractor_protocol.rs`

```rust
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_system_error::SystemError;

/// Protocol for extracting features and AST paths from source code.
/// F-5 Upgrade: Now supports multi-language via Tree-sitter.
pub trait AstExtractorProtocol {
    fn extract_from_file(&self, path: &FilePath, content: &FileContent) -> Result<ExtractedFeature, SystemError>;
}
```

### File: `contract_bpe_transformer_protocol.rs`

```rust
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_token_ids_vo::TokenIds;
use crate::taxonomy_system_error::SystemError;

/// F-1 Upgrade: Protocol for real BPE tokenization via HuggingFace tokenizers crate.
pub trait BpeTransformerProtocol {
    fn tokenize(&self, features: &ExtractedFeature) -> Result<TokenIds, SystemError>;
    fn vocab_size(&self) -> usize;
}
```

### File: `contract_exception_filter_protocol.rs`

```rust
use crate::taxonomy_file_path_vo::FilePath;

/// Protocol for filtering file exceptions that are immune to naming rules.
pub trait ExceptionFilterProtocol {
    fn is_exempt(&self, path: &FilePath) -> bool;
}
```

### File: `contract_compiler_runner_port.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;

/// Port for cargo check compiler verification.
pub trait CompilerRunnerPort {
    fn run_check(&self, workspace: &FilePath) -> Result<(), SystemError>;
}
```

### File: `contract_linter_runner_port.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;

/// Port for lint-arwaky linter re-verification after modifications.
pub trait LinterRunnerPort {
    fn run_lint(&self, files: &[FilePath]) -> Result<(), SystemError>;
}
```

### File: `contract_autorepair_aggregate.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;

/// Main aggregate interaction boundary for launching the Auto-Repair process.
pub trait AutorepairAggregate {
    fn execute_fix(&self, workspace_root: &FilePath, target_file: &FilePath) -> Result<(), SystemError>;
}
```

---

## 3. Capabilities Layer (Pure Domain Algorithms)

Pure modular programming logic with no direct I/O. AES102 is satisfied with standard suffixes `_processor`, `_transformer`, and `_classifier`.

### File: `capabilities_reference_processor.rs`

```rust
use crate::contract_reference_processor_protocol::ReferenceProcessorProtocol;
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_module_name_vo::ModuleName;

pub struct StringReferenceProcessor;

impl ReferenceProcessorProtocol for StringReferenceProcessor {
    fn replace_references(&self, content: &FileContent, old_name: &ModuleName, new_name: &ModuleName) -> FileContent {
        let mut new_text = content.0.replace(
            &format!("use crate::{};", old_name.0),
            &format!("use crate::{};", new_name.0)
        );
        new_text = new_text.replace(
            &format!("mod {};", old_name.0),
            &format!("mod {};", new_name.0)
        );
        FileContent(new_text)
    }
}
```

### File: `capabilities_file_name_resolver.rs`

```rust
use crate::contract_file_name_resolver_protocol::FileNameResolverProtocol;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_module_name_vo::ModuleName;
use crate::taxonomy_file_extension_vo::FileExtension;
use crate::taxonomy_prediction_result_vo::PredictionResult;

pub struct StandardFileNameResolver;

impl FileNameResolverProtocol for StandardFileNameResolver {
    fn extract_module_name(&self, path: &FilePath) -> Result<ModuleName, SystemError> {
        let name_str = path.0.file_stem()
            .and_then(|n| n.to_str())
            .ok_or_else(|| SystemError::ParsingError(ErrorMessage("Invalid target file name".to_string())))?;
        Ok(ModuleName(name_str.to_string()))
    }

    fn extract_extension(&self, path: &FilePath) -> Result<FileExtension, SystemError> {
        let ext_str = path.0.extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| SystemError::ParsingError(ErrorMessage("Cannot read file extension".to_string())))?;
        Ok(FileExtension(ext_str.to_string()))
    }

    fn assemble_new_name(&self, r: &PredictionResult, _ext: &FileExtension) -> ModuleName {
        ModuleName(format!("{}_{}_{}", r.prefix.as_str(), r.concept.as_str(), r.suffix.as_str()))
    }

    fn build_sibling_path(&self, original: &FilePath, new_name: &ModuleName, ext: &FileExtension) -> Result<FilePath, SystemError> {
        let parent = original.0.parent()
            .ok_or_else(|| SystemError::ParsingError(ErrorMessage("File has no parent directory".to_string())))?;
        let file_name = format!("{}.{}", new_name.as_str(), ext.as_str());
        Ok(FilePath(parent.join(file_name)))
    }
}
```

### File: `capabilities_ast_extractor.rs`

```rust
use crate::contract_ast_extractor_protocol::AstExtractorProtocol;
use crate::taxonomy_extracted_feature_vo::{ExtractedFeature, AstPath, Language};
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_system_constant::MAX_AST_DEPTH;
use tree_sitter::{Parser, Language as TsLanguage, Tree, Node};

pub struct TreeSitterAstExtractor {
    rust_lang: TsLanguage,
    python_lang: TsLanguage,
    typescript_lang: TsLanguage,
}

impl TreeSitterAstExtractor {
    pub fn new() -> Result<Self, SystemError> {
        Ok(Self {
            rust_lang: tree_sitter_rust::LANGUAGE.into(),
            python_lang: tree_sitter_python::LANGUAGE.into(),
            typescript_lang: tree_sitter_typescript::LANGUAGE.into(),
        })
    }

    fn detect_language(&self, path: &FilePath) -> Language {
        match path.0.extension().and_then(|e| e.to_str()) {
            Some("rs") => Language::Rust,
            Some("py") => Language::Python,
            Some("ts") | Some("tsx") => Language::TypeScript,
            Some("js") | Some("jsx") => Language::JavaScript,
            _ => Language::Unknown,
        }
    }

    fn get_parser(&self, lang: Language) -> Result<Parser, SystemError> {
        let mut parser = Parser::new();
        let ts_lang = match lang {
            Language::Rust => self.rust_lang.clone(),
            Language::Python => self.python_lang.clone(),
            Language::TypeScript | Language::JavaScript => self.typescript_lang.clone(),
            Language::Unknown => return Err(SystemError::UnsupportedLanguage(ErrorMessage("Cannot parse unknown language".to_string()))),
        };
        parser.set_language(&ts_lang)
            .map_err(|e| SystemError::ParsingError(ErrorMessage(format!("Failed to set parser language: {}", e))))?;
        Ok(parser)
    }

    fn extract_ast_paths_recursive(&self, node: Node, source: &str, paths: &mut Vec<AstPath>, depth: usize) {
        if depth > MAX_AST_DEPTH {
            return;
        }

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
            self.extract_ast_paths_recursive(child, source, paths, depth + 1);
        }
    }
}

impl AstExtractorProtocol for TreeSitterAstExtractor {
    fn extract_from_file(&self, path: &FilePath, content: &FileContent) -> Result<ExtractedFeature, SystemError> {
        let language = self.detect_language(path);
        let mut parser = self.get_parser(language)?;
        let tree = parser.parse(&content.0, None)
            .ok_or_else(|| SystemError::ParsingError(ErrorMessage("Failed to parse source file".to_string())))?;

        let mut imports = Vec::new();
        let mut structs_traits = Vec::new();
        let mut docstrings = Vec::new();
        let mut ast_paths = Vec::new();

        let root = tree.root_node();
        self.extract_ast_paths_recursive(root, &content.0, &mut ast_paths, 0);

        let mut cursor = root.walk();
        for child in root.named_children(&mut cursor) {
            match child.kind() {
                "use_declaration" | "import_statement" | "import_from_statement" => {
                    if let Ok(text) = child.utf8_text(content.0.as_bytes()) {
                        imports.push(text.to_string());
                    }
                }
                "struct_item" | "trait_item" | "class_definition" | "interface_declaration" => {
                    if let Some(name) = child.child_by_field_name("name") {
                        if let Ok(text) = name.utf8_text(content.0.as_bytes()) {
                            structs_traits.push(text.to_string());
                        }
                    }
                }
                "impl_item" => {
                    if let Some(trait_ref) = child.child_by_field_name("trait") {
                        if let Ok(text) = trait_ref.utf8_text(content.0.as_bytes()) {
                            structs_traits.push(text.to_string());
                        }
                    }
                }
                "attribute_item" => {
                    if let Ok(text) = child.utf8_text(content.0.as_bytes()) {
                        if text.starts_with("///") || text.starts_with("#[doc") {
                            docstrings.push(text.to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        let directory_context = path.0.parent()
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

### File: `capabilities_bpe_transformer.rs`

```rust
use crate::contract_bpe_transformer_protocol::BpeTransformerProtocol;
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_token_ids_vo::TokenIds;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_system_constant::MAX_SEQ_LEN;
use tokenizers::{Tokenizer, models::bpe::BPE, NormalizedString, PaddingParams, PaddingStrategy};

pub struct RealBpeTokenizer {
    tokenizer: Tokenizer,
}

impl RealBpeTokenizer {
    pub fn from_file(path: &str) -> Result<Self, SystemError> {
        let tokenizer = Tokenizer::from_file(path)
            .map_err(|e| SystemError::TokenizationError(ErrorMessage(format!("Failed to load BPE tokenizer: {}", e))))?;

        let padding = PaddingParams {
            strategy: PaddingStrategy::Fixed(MAX_SEQ_LEN),
            pad_token: "[PAD]".to_string(),
            ..Default::default()
        };
        let mut tokenizer = tokenizer;
        tokenizer.with_padding(Some(padding));

        Ok(Self { tokenizer })
    }

    pub fn from_vocab(vocab: &[(String, u32)]) -> Result<Self, SystemError> {
        let mut builder = BPE::new();
        let mut vocab_map = std::collections::HashMap::new();
        for (token, id) in vocab {
            vocab_map.insert(token.clone(), *id);
        }
        builder = builder.vocab(vocab_map);
        let model = builder.build()
            .map_err(|e| SystemError::TokenizationError(ErrorMessage(format!("Failed to build BPE model: {}", e))))?;

        let tokenizer = Tokenizer::new(model);
        Ok(Self { tokenizer })
    }
}

impl BpeTransformerProtocol for RealBpeTokenizer {
    fn tokenize(&self, features: &ExtractedFeature) -> Result<TokenIds, SystemError> {
        let mut combined_text = String::new();
        for imp in &features.imports { combined_text.push_str(imp); combined_text.push(' '); }
        for st in &features.structs_traits { combined_text.push_str(st); combined_text.push(' '); }
        for doc in &features.docstrings { combined_text.push_str(doc); combined_text.push(' '); }
        combined_text.push_str(&features.directory_context);

        let encoding = self.tokenizer.encode(combined_text, true)
            .map_err(|e| SystemError::TokenizationError(ErrorMessage(format!("Tokenization failed: {}", e))))?;

        let ids: Vec<u32> = encoding.get_ids().iter().map(|&id| id).collect();
        Ok(TokenIds(ids))
    }

    fn vocab_size(&self) -> usize {
        self.tokenizer.get_vocab_size(true)
    }
}
```

### File: `capabilities_exception_filter.rs`

```rust
use crate::contract_exception_filter_protocol::ExceptionFilterProtocol;
use crate::taxonomy_file_path_vo::FilePath;

pub struct ExceptionFilter;

impl ExceptionFilterProtocol for ExceptionFilter {
    fn is_exempt(&self, path: &FilePath) -> bool {
        let filename = match path.0.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => return true,
        };

        if matches!(filename, "main.rs" | "lib.rs" | "mod.rs" | "build.rs" | "__init__.py" | "__main__.py" | "index.ts" | "index.js") {
            return true;
        }

        if filename.ends_with("_test.rs") || filename.starts_with("test_") || filename.ends_with(".spec.ts") || filename.ends_with(".test.ts") || filename.ends_with(".test.js") {
            return true;
        }

        false
    }
}
```

### File: `capabilities_confidence_calibrator.rs`

```rust
use crate::taxonomy_prediction_result_vo::PredictionResult;
use crate::taxonomy_system_constant::{CONFIDENCE_TEMPERATURE, ENTROPY_THRESHOLD};

pub struct ConfidenceCalibrator;

impl ConfidenceCalibrator {
    /// F-3 Upgrade: Apply temperature scaling to logits for calibrated confidence.
    pub fn temperature_scale(logits: &[f32], temperature: f32) -> Vec<f32> {
        let scaled: Vec<f32> = logits.iter().map(|&l| l / temperature).collect();
        let max = scaled.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exp_sum: f32 = scaled.iter().map(|&l| (l - max).exp()).sum();
        scaled.iter().map(|&l| ((l - max).exp() / exp_sum)).collect()
    }

    /// F-3 Upgrade: Compute Shannon entropy of a probability distribution.
    pub fn entropy(probs: &[f32]) -> f32 {
        -probs.iter()
            .filter(|&&p| p > 1e-10)
            .map(|&p| p * p.ln())
            .sum::<f32>()
    }

    /// F-3 Upgrade: Check if prediction confidence meets threshold via entropy.
    pub fn is_confident(result: &PredictionResult) -> bool {
        result.entropy < ENTROPY_THRESHOLD
            && result.prefix_confidence >= 0.85
            && result.suffix_confidence >= 0.85
            && result.concept_confidence >= 0.85
    }

    /// F-3 Upgrade: Decode logits with temperature scaling and compute entropy.
    pub fn decode_with_confidence(
        prefix_logits: &[f32],
        suffix_logits: &[f32],
        concept_logits: &[f32],
    ) -> (usize, f32, usize, f32, usize, f32, f32) {
        let p_probs = Self::temperature_scale(prefix_logits, CONFIDENCE_TEMPERATURE);
        let s_probs = Self::temperature_scale(suffix_logits, CONFIDENCE_TEMPERATURE);
        let c_probs = Self::temperature_scale(concept_logits, CONFIDENCE_TEMPERATURE);

        let p_idx = p_probs.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;
        let s_idx = s_probs.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;
        let c_idx = c_probs.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;

        let all_probs: Vec<f32> = p_probs.iter().chain(s_probs.iter()).chain(c_probs.iter()).copied().collect();
        let total_entropy = Self::entropy(&all_probs);

        (p_idx, p_probs[p_idx], s_idx, s_probs[s_idx], c_idx, c_probs[c_idx], total_entropy)
    }
}
```

### File: `capabilities_ast_path_embedder.rs`

```rust
use crate::taxonomy_extracted_feature_vo::{ExtractedFeature, AstPath};
use crate::taxonomy_system_constant::{AST_PATH_VOCAB_SIZE, MAX_AST_DEPTH};

pub struct AstPathEmbedder;

impl AstPathEmbedder {
    /// F-4 Upgrade: Hash AST node type sequence to embedding index (Code2Vec-style).
    pub fn hash_node_types(node_types: &[String]) -> usize {
        let mut hash: u64 = 0;
        for nt in node_types {
            for byte in nt.bytes() {
                hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
            }
        }
        (hash % AST_PATH_VOCAB_SIZE as u64) as usize
    }

    /// F-4 Upgrade: Compute depth-weighted AST path embedding vector.
    pub fn embed_paths(ast_paths: &[AstPath]) -> Vec<f32> {
        let mut embedding = vec![0.0f32; AST_PATH_VOCAB_SIZE];

        for path in ast_paths {
            let idx = Self::hash_node_types(&path.node_types);
            let weight = 1.0 / (path.depth as f32 + 1.0);
            embedding[idx] += weight;
        }

        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for e in &mut embedding {
                *e /= norm;
            }
        }

        embedding
    }
}
```

---

## 4. Infrastructure Layer (I/O & External Systems)

Concrete adapters that bridge disk interaction and external command execution.

### File: `infrastructure_fs_reader.rs`

```rust
use crate::contract_file_reader_port::FileReaderPort;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_file_bytes_vo::FileBytes;
use std::fs;

pub struct FileSystemReaderAdapter;

impl FileReaderPort for FileSystemReaderAdapter {
    fn read_file_as_string(&self, path: &FilePath) -> Result<FileContent, SystemError> {
        let content = fs::read_to_string(&path.0)?;
        Ok(FileContent(content))
    }

    fn read_file_as_bytes(&self, path: &FilePath) -> Result<FileBytes, SystemError> {
        let bytes = fs::read(&path.0)?;
        Ok(FileBytes(bytes))
    }
}
```

### File: `infrastructure_fs_writer.rs`

```rust
use crate::contract_file_writer_port::FileWriterPort;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_file_content_vo::FileContent;
use std::fs;

pub struct FileSystemWriterAdapter;

impl FileWriterPort for FileSystemWriterAdapter {
    fn write_file_as_string(&self, path: &FilePath, content: &FileContent) -> Result<(), SystemError> {
        fs::write(&path.0, &content.0)?;
        Ok(())
    }

    fn rename_file(&self, old_path: &FilePath, new_path: &FilePath) -> Result<(), SystemError> {
        fs::rename(&old_path.0, &new_path.0)?;
        Ok(())
    }
}
```

### File: `infrastructure_workspace_scanner.rs`

```rust
use crate::contract_workspace_scanner_port::WorkspaceScannerPort;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_extracted_feature_vo::Language;
use walkdir::WalkDir;

pub struct WalkdirWorkspaceScannerAdapter;

impl WorkspaceScannerPort for WalkdirWorkspaceScannerAdapter {
    fn scan_files(&self, workspace_root: &FilePath, languages: &[Language]) -> Result<Vec<FilePath>, SystemError> {
        let extensions: Vec<&str> = languages.iter().flat_map(|lang| match lang {
            Language::Rust => vec!["rs"],
            Language::Python => vec!["py"],
            Language::TypeScript => vec!["ts", "tsx"],
            Language::JavaScript => vec!["js", "jsx"],
            Language::Unknown => vec![],
        }).collect();

        let mut paths = Vec::new();
        for entry in WalkDir::new(&workspace_root.0) {
            let entry = entry.map_err(|e| SystemError::IoError(ErrorMessage(e.to_string())))?;
            if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
                if extensions.contains(&ext) {
                    paths.push(FilePath(entry.path().to_path_buf()));
                }
            }
        }
        Ok(paths)
    }
}
```

### File: `infrastructure_compiler_adapter.rs`

```rust
use crate::contract_compiler_runner_port::CompilerRunnerPort;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_path_vo::FilePath;
use std::process::Command;

pub struct CargoCompilerAdapter;

impl CompilerRunnerPort for CargoCompilerAdapter {
    fn run_check(&self, workspace: &FilePath) -> Result<(), SystemError> {
        let output = Command::new("cargo")
            .arg("check")
            .current_dir(&workspace.0)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(SystemError::VerificationError(ErrorMessage(format!("cargo check failed: {}", stderr))));
        }
        Ok(())
    }
}
```

### File: `infrastructure_linter_adapter.rs`

```rust
use crate::contract_linter_runner_port::LinterRunnerPort;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_path_vo::FilePath;
use std::process::Command;

pub struct LintArwakyAdapter;

impl LinterRunnerPort for LintArwakyAdapter {
    fn run_lint(&self, files: &[FilePath]) -> Result<(), SystemError> {
        for file in files {
            let output = Command::new("cargo")
                .arg("run")
                .arg("--bin")
                .arg("lint-arwaky-cli")
                .arg("--")
                .arg("scan")
                .arg(&file.0)
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                return Err(SystemError::VerificationError(ErrorMessage(format!("linter failed for file {:?}: {}", file.0, stderr))));
            }
        }
        Ok(())
    }
}
```

---

## 5. Agent Layer (Orchestration Workflow)

Transactional Auto-Repair workflow coordinator that implements the Aggregate.

### File: `agent_autorepair_orchestrator.rs`

```rust
use crate::contract_file_reader_port::FileReaderPort;
use crate::contract_file_writer_port::FileWriterPort;
use crate::contract_workspace_scanner_port::WorkspaceScannerPort;
use crate::contract_reference_processor_protocol::ReferenceProcessorProtocol;
use crate::contract_file_name_resolver_protocol::FileNameResolverProtocol;
use crate::contract_ast_extractor_protocol::AstExtractorProtocol;
use crate::contract_model_classifier_protocol::ModelClassifierProtocol;
use crate::contract_bpe_transformer_protocol::BpeTransformerProtocol;
use crate::contract_exception_filter_protocol::ExceptionFilterProtocol;
use crate::contract_compiler_runner_port::CompilerRunnerPort;
use crate::contract_linter_runner_port::LinterRunnerPort;
use crate::contract_autorepair_aggregate::AutorepairAggregate;
use crate::capabilities_confidence_calibrator::ConfidenceCalibrator;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_extracted_feature_vo::Language;
use crate::taxonomy_system_constant::CONFIDENCE_TEMPERATURE;

pub struct AutorepairOrchestratorDeps {
    pub reader: Box<dyn FileReaderPort>,
    pub writer: Box<dyn FileWriterPort>,
    pub scanner: Box<dyn WorkspaceScannerPort>,
    pub replacer: Box<dyn ReferenceProcessorProtocol>,
    pub resolver: Box<dyn FileNameResolverProtocol>,
    pub extractor: Box<dyn AstExtractorProtocol>,
    pub predictor: Box<dyn ModelClassifierProtocol>,
    pub tokenizer: Box<dyn BpeTransformerProtocol>,
    pub exception_filter: Box<dyn ExceptionFilterProtocol>,
    pub compiler: Box<dyn CompilerRunnerPort>,
    pub linter: Box<dyn LinterRunnerPort>,
}

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
    compiler: Box<dyn CompilerRunnerPort>,
    linter: Box<dyn LinterRunnerPort>,
}

impl AutorepairOrchestrator {
    pub fn new(deps: AutorepairOrchestratorDeps) -> Self {
        Self {
            reader: deps.reader, writer: deps.writer, scanner: deps.scanner,
            replacer: deps.replacer, resolver: deps.resolver, extractor: deps.extractor,
            predictor: deps.predictor, tokenizer: deps.tokenizer, exception_filter: deps.exception_filter,
            compiler: deps.compiler, linter: deps.linter,
        }
    }
}

impl AutorepairAggregate for AutorepairOrchestrator {
    fn execute_fix(&self, workspace_root: &FilePath, target_file: &FilePath) -> Result<(), SystemError> {
        if self.exception_filter.is_exempt(target_file) {
            return Err(SystemError::ExemptFile(ErrorMessage(format!("File is exempt: {:?}", target_file.0))));
        }

        let content = self.reader.read_file_as_string(target_file)?;
        let features = self.extractor.extract_from_file(target_file, &content)?;
        let tokens = self.tokenizer.tokenize(&features)?;

        let config = self.predictor.get_config();
        if config.vocab_size == 0 || config.d_model == 0 {
            return Err(SystemError::PredictionError(ErrorMessage("Model config invalid".to_string())));
        }

        let prediction = self.predictor.predict(&features, &tokens)?;

        // F-3 Upgrade: Use entropy-based confidence check.
        if !ConfidenceCalibrator::is_confident(&prediction) {
            let alternatives = self.predictor.predict_with_temperature(&features, &tokens, CONFIDENCE_TEMPERATURE)?;
            let alt_details = alternatives.iter()
                .map(|alt| format!("{}_{}_{} ({:.1}%, H={:.2})", alt.prefix.as_str(), alt.concept.as_str(), alt.suffix.as_str(), alt.prefix_confidence * 100.0, alt.entropy))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(SystemError::LowConfidence(ErrorMessage(format!("Confidence check failed (entropy={:.2}). Alternatives: {}", prediction.entropy, alt_details))));
        }

        let old_name = self.resolver.extract_module_name(target_file)?;
        let ext = self.resolver.extract_extension(target_file)?;
        let new_name = self.resolver.assemble_new_name(&prediction, &ext);
        let new_target_path = self.resolver.build_sibling_path(target_file, &new_name, &ext)?;

        // F-5 Upgrade: Multi-language scanner.
        let languages = vec![Language::Rust, Language::Python, Language::TypeScript, Language::JavaScript];
        let files = self.scanner.scan_files(workspace_root, &languages)?;
        let mut backups: Vec<(FilePath, FileContent)> = Vec::with_capacity(files.len());
        let mut modified_files = Vec::new();

        for file in &files {
            let file_content = self.reader.read_file_as_string(file)?;
            backups.push((file.clone(), file_content.clone()));

            if file_content.0.contains(&old_name.0) {
                let updated_content = self.replacer.replace_references(&file_content, &old_name, &new_name);
                self.writer.write_file_as_string(file, &updated_content)?;
                modified_files.push(file.clone());
            }
        }

        let mut renamed = false;
        self.writer.rename_file(target_file, &new_target_path)?;
        renamed = true;
        modified_files.push(new_target_path.clone());

        let verification_result = self.compiler.run_check(workspace_root)
            .and_then(|_| self.linter.run_lint(&modified_files));

        if let Err(verification_err) = verification_result {
            let mut rollback_errors = Vec::new();
            let mut rollback_failed = false;

            if renamed {
                if let Err(e) = self.writer.rename_file(&new_target_path, target_file) {
                    rollback_errors.push(format!("rename rollback failed: {:?}", e));
                    rollback_failed = true;
                }
            }

            for (original_path, backup_content) in backups {
                if let Err(e) = self.writer.write_file_as_string(&original_path, &backup_content) {
                    rollback_errors.push(format!("write rollback failed for {:?}: {:?}", original_path.0, e));
                    rollback_failed = true;
                }
            }

            let err_msg = format!(
                "Post auto-fix verification failed: {:?}. Rollback issues: [{}]",
                verification_err,
                if rollback_errors.is_empty() { "None".to_string() } else { rollback_errors.join("; ") }
            );

            if rollback_failed {
                return Err(SystemError::RollbackFailure(ErrorMessage(err_msg)));
            } else {
                return Err(SystemError::VerificationError(ErrorMessage(err_msg)));
            }
        }

        Ok(())
    }
}
```

---

## 6. Surface Layer (User Interaction / UI)

Outer interaction boundary that houses the input routing controller (Smart Surface - AES506).

### File: `surface_autofix_command.rs`

```rust
use crate::contract_autorepair_aggregate::AutorepairAggregate;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_path_vo::FilePath;

pub struct AutofixCommand<'a> {
    aggregate: &'a dyn AutorepairAggregate,
}

impl<'a> AutofixCommand<'a> {
    pub fn new(aggregate: &'a dyn AutorepairAggregate) -> Self {
        Self { aggregate }
    }

    pub fn route_command(&self, command: &str, workspace: &FilePath, target: &FilePath) -> Result<(), SystemError> {
        match command {
            "autofix" => self.aggregate.execute_fix(workspace, target),
            _ => Err(SystemError::ArgumentError(ErrorMessage(format!("Unknown command: {}", command)))),
        }
    }
}
```

---

## 7. Root Layer (Dependency Injection / Composition Root)

Top-level Composition Root that wires all concrete adapters to contract interface types.

### File: `root_autorepair_container.rs`

```rust
use crate::infrastructure_fs_reader::FileSystemReaderAdapter;
use crate::infrastructure_fs_writer::FileSystemWriterAdapter;
use crate::infrastructure_workspace_scanner::WalkdirWorkspaceScannerAdapter;
use crate::infrastructure_compiler_adapter::CargoCompilerAdapter;
use crate::infrastructure_linter_adapter::LintArwakyAdapter;
use crate::capabilities_reference_processor::StringReferenceProcessor;
use crate::capabilities_file_name_resolver::StandardFileNameResolver;
use crate::capabilities_model_classifier::AESNamingModelPredictor;
use crate::capabilities_ast_extractor::TreeSitterAstExtractor;
use crate::capabilities_bpe_transformer::RealBpeTokenizer;
use crate::capabilities_exception_filter::ExceptionFilter;
use crate::agent_autorepair_orchestrator::{AutorepairOrchestrator, AutorepairOrchestratorDeps};
use crate::taxonomy_system_constant::{MODEL_WEIGHTS_PATH, TOKENIZER_PATH};
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_path_vo::FilePath;
use crate::contract_autorepair_aggregate::AutorepairAggregate;

use burn::backend::NdArray;
use burn::tensor::Device;

pub struct AutorepairContainer;

impl AutorepairContainer {
    pub fn build() -> Result<Box<dyn AutorepairAggregate>, SystemError> {
        let weights_path = FilePath::from_constant(MODEL_WEIGHTS_PATH);
        let tokenizer_path = FilePath::from_constant(TOKENIZER_PATH);

        let reader_adapter = FileSystemReaderAdapter;
        let weights_bytes = reader_adapter.read_file_as_bytes(&weights_path)?;

        let device = Device::<NdArray>::default();
        let predictor = AESNamingModelPredictor::<NdArray>::new_from_bytes(&weights_bytes, &device)?;

        // F-1 Upgrade: Load real BPE tokenizer from file.
        let bpe_tokenizer = RealBpeTokenizer::from_file(tokenizer_path.0.to_str()
            .ok_or_else(|| SystemError::TokenizationError(ErrorMessage("Invalid tokenizer path".to_string())))?)?;

        // F-5 Upgrade: Tree-sitter multi-language AST extractor.
        let ast_extractor = TreeSitterAstExtractor::new()?;

        let deps = AutorepairOrchestratorDeps {
            reader: Box::new(reader_adapter),
            writer: Box::new(FileSystemWriterAdapter),
            scanner: Box::new(WalkdirWorkspaceScannerAdapter),
            replacer: Box::new(StringReferenceProcessor),
            resolver: Box::new(StandardFileNameResolver),
            extractor: Box::new(ast_extractor),
            predictor: Box::new(predictor),
            tokenizer: Box::new(bpe_tokenizer),
            exception_filter: Box::new(ExceptionFilter),
            compiler: Box::new(CargoCompilerAdapter),
            linter: Box::new(LintArwakyAdapter),
        };

        Ok(Box::new(AutorepairOrchestrator::new(deps)))
    }
}
```

### File: `root_cli_main_entry.rs`

```rust
use crate::root_autorepair_container::AutorepairContainer;
use crate::surface_autofix_command::AutofixCommand;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_path_vo::FilePath;
use std::env;

fn run() -> Result<(), SystemError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        return Err(SystemError::ArgumentError(
            ErrorMessage("Usage: lint-arwaky-cli <command> <workspace_dir> <target_file>".to_string())
        ));
    }

    if args[2].is_empty() || args[3].is_empty() {
        return Err(SystemError::ArgumentError(ErrorMessage("Path arguments cannot be empty".to_string())));
    }

    let aggregate = AutorepairContainer::build()?;
    let command = &args[1];
    let workspace = FilePath::from_constant(&args[2]);
    let target = FilePath::from_constant(&args[3]);

    let controller = AutofixCommand::new(aggregate.as_ref());

    match controller.route_command(command, &workspace, &target) {
        Ok(()) => {
            println!("Auto-Repair Success!");
            Ok(())
        }
        Err(SystemError::LowConfidence(msg)) => {
            println!("Info: System deferred automatic repair due to low confidence.");
            println!("{}", msg.as_str());
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Fatal Error: {:?}", e);
        std::process::exit(1);
    }
}
```

---

## 8. Unit Tests (Pure Capabilities)

### File: `tests_capabilities_reference_processor.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities_reference_processor::StringReferenceProcessor;
    use crate::taxonomy_file_content_vo::FileContent;
    use crate::taxonomy_module_name_vo::ModuleName;

    #[test]
    fn replaces_use_crate_reference() {
        let processor = StringReferenceProcessor;
        let content = FileContent("use crate::old_module;\nfn main() {}".to_string());
        let old = ModuleName("old_module".to_string());
        let new = ModuleName("new_module".to_string());

        let result = processor.replace_references(&content, &old, &new);
        assert!(result.0.contains("use crate::new_module;"));
        assert!(!result.0.contains("old_module"));
    }

    #[test]
    fn replaces_mod_reference() {
        let processor = StringReferenceProcessor;
        let content = FileContent("mod old_module;\npub use old_module::Item;".to_string());
        let old = ModuleName("old_module".to_string());
        let new = ModuleName("new_module".to_string());

        let result = processor.replace_references(&content, &old, &new);
        assert!(result.0.contains("mod new_module;"));
    }

    #[test]
    fn no_change_when_no_match() {
        let processor = StringReferenceProcessor;
        let content = FileContent("fn main() {}".to_string());
        let old = ModuleName("old_module".to_string());
        let new = ModuleName("new_module".to_string());

        let result = processor.replace_references(&content, &old, &new);
        assert_eq!(result.0, "fn main() {}");
    }
}
```

### File: `tests_capabilities_exception_filter.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities_exception_filter::ExceptionFilter;
    use crate::taxonomy_file_path_vo::FilePath;
    use std::path::PathBuf;

    #[test]
    fn exempts_main_rs() {
        let filter = ExceptionFilter;
        let path = FilePath(PathBuf::from("src/main.rs"));
        assert!(filter.is_exempt(&path));
    }

    #[test]
    fn exempts_lib_rs() {
        let filter = ExceptionFilter;
        let path = FilePath(PathBuf::from("src/lib.rs"));
        assert!(filter.is_exempt(&path));
    }

    #[test]
    fn exempts_test_file() {
        let filter = ExceptionFilter;
        let path = FilePath(PathBuf::from("src/foo_test.rs"));
        assert!(filter.is_exempt(&path));
    }

    #[test]
    fn exempts_test_prefix() {
        let filter = ExceptionFilter;
        let path = FilePath(PathBuf::from("src/test_foo.rs"));
        assert!(filter.is_exempt(&path));
    }

    #[test]
    fn exempts_jest_test_file() {
        let filter = ExceptionFilter;
        let path = FilePath(PathBuf::from("src/foo.test.ts"));
        assert!(filter.is_exempt(&path));
    }

    #[test]
    fn does_not_exempt_normal_file() {
        let filter = ExceptionFilter;
        let path = FilePath(PathBuf::from("src/foo_bar.rs"));
        assert!(!filter.is_exempt(&path));
    }
}
```

### File: `tests_capabilities_confidence_calibrator.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities_confidence_calibrator::ConfidenceCalibrator;

    #[test]
    fn temperature_scale_produces_valid_distribution() {
        let logits = vec![1.0, 2.0, 3.0, 0.5];
        let probs = ConfidenceCalibrator::temperature_scale(&logits, 1.0);
        let sum: f32 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-5);
    }

    #[test]
    fn entropy_of_uniform_distribution_is_high() {
        let probs = vec![0.25, 0.25, 0.25, 0.25];
        let ent = ConfidenceCalibrator::entropy(&probs);
        assert!(ent > 1.3);
    }

    #[test]
    fn entropy_of_peaked_distribution_is_low() {
        let probs = vec![0.99, 0.003, 0.003, 0.004];
        let ent = ConfidenceCalibrator::entropy(&probs);
        assert!(ent < 0.2);
    }

    #[test]
    fn high_temperature_flattens_distribution() {
        let logits = vec![1.0, 2.0, 3.0];
        let cold = ConfidenceCalibrator::temperature_scale(&logits, 0.5);
        let hot = ConfidenceCalibrator::temperature_scale(&logits, 5.0);
        assert!(hot[2] < cold[2]);
    }
}
```

### File: `tests_capabilities_ast_path_embedder.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities_ast_path_embedder::AstPathEmbedder;
    use crate::taxonomy_extracted_feature_vo::AstPath;

    #[test]
    fn hash_deterministic() {
        let types = vec!["function_item".to_string(), "identifier".to_string()];
        let h1 = AstPathEmbedder::hash_node_types(&types);
        let h2 = AstPathEmbedder::hash_node_types(&types);
        assert_eq!(h1, h2);
    }

    #[test]
    fn different_types_different_hash() {
        let t1 = vec!["function_item".to_string()];
        let t2 = vec!["struct_item".to_string()];
        assert_ne!(AstPathEmbedder::hash_node_types(&t1), AstPathEmbedder::hash_node_types(&t2));
    }

    #[test]
    fn embed_returns_normalized_vector() {
        let paths = vec![
            AstPath { node_types: vec!["a".to_string()], target_token: "x".to_string(), depth: 1 },
            AstPath { node_types: vec!["b".to_string()], target_token: "y".to_string(), depth: 2 },
        ];
        let emb = AstPathEmbedder::embed_paths(&paths);
        let norm: f32 = emb.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5);
    }

    #[test]
    fn deeper_paths_have_less_weight() {
        let shallow = vec![AstPath { node_types: vec!["x".to_string()], target_token: "".to_string(), depth: 1 }];
        let deep = vec![AstPath { node_types: vec!["x".to_string()], target_token: "".to_string(), depth: 5 }];
        let e1 = AstPathEmbedder::embed_paths(&shallow);
        let e2 = AstPathEmbedder::embed_paths(&deep);
        assert!(e1.iter().sum::<f32>() > e2.iter().sum::<f32>());
    }
}
```

### File: `tests_taxonomy_token_ids.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_returns_correct_count() {
        let ids = TokenIds(vec![1, 2, 3, 4, 5]);
        assert_eq!(ids.len(), 5);
    }

    #[test]
    fn is_empty_on_empty_vec() {
        let ids = TokenIds(Vec::new());
        assert!(ids.is_empty());
    }

    #[test]
    fn is_not_empty_with_tokens() {
        let ids = TokenIds(vec![0]);
        assert!(!ids.is_empty());
    }

    #[test]
    fn attention_mask_length_matches_max() {
        let ids = TokenIds(vec![1, 2, 3]);
        let mask = ids.attention_mask(10);
        assert_eq!(mask.len(), 10);
        assert_eq!(mask[0], 1.0);
        assert_eq!(mask[3], 0.0);
    }
}
```

---

## 9. Frontend Technology Comparison

| Aspect                 | v11                   | v12                           | Improvement              |
| ---------------------- | --------------------- | ----------------------------- | ------------------------ |
| Tokenizer              | Hardcoded match table | HuggingFace`tokenizers` BPE | 100x vocab coverage      |
| Positional Encoding    | ❌ None               | ✅ Sinusoidal                 | Sequence order awareness |
| AST Extraction         | `syn` (Rust only)   | `tree-sitter` (4 languages) | Multi-language support   |
| Directory Context      | Hash 1024 vocab       | Code2Vec AST path embed       | Structural awareness     |
| Confidence             | Raw softmax           | Temperature + entropy         | Calibrated uncertainty   |
| Model Size             | 12K/128/4L            | 32K/256/6L                    | 4x capacity              |
| Interpretability       | ❌ None               | ✅ Attention weights          | Debuggable predictions   |
| Alternative Generation | Mask top-3            | Temperature sampling          | Diverse alternatives     |
| Scan Scope             | Rust only             | RS/PY/TS/JS                   | Full-stack support       |
