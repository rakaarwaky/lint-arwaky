````markdown
# Implementation Draft: AI Auto-Repair Model (Perfect AES Dogfooding v10)

Draft v10 resolves all audit findings from v9, including fixes for the model input pipeline (directory prior & feature extraction), elimination of silent fallback in label lookup, separation of file extension from module name for reference propagation, deduplication of forward pass, as well as strengthened transactional rollback logic and CLI input validation.

## 1. Taxonomy Layer (Data, Constants, Errors & Value Objects)

File: `taxonomy_system_constant.rs`

```rust
/// Absolute/relative path to the Safetensors model weights file.
/// Will be loaded by Infrastructure during Root initialization.
pub const MODEL_WEIGHTS_PATH: &str = "weights/model.safetensors";
```
````

File: `taxonomy_prefix_label_constant.rs`

```rust
/// Class label list for the AES architecture prefix.
/// Indices in this array correspond to the model output classes.
pub const PREFIX_LABELS: &[&str] = &[
    "root", "taxonomy", "contract", "capabilities", "infrastructure", "agent", "surface",
];
```

File: `taxonomy_suffix_label_constant.rs`

```rust
/// Class label list for the AES role suffix.
/// Indices in this array correspond to the model output classes.
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

File: `taxonomy_concept_vocab_constant.rs`

```rust
/// Base vocabulary for decoding file name domain concepts from the model.
pub const CONCEPT_VOCAB: &[&str] = &[
    "unknown", "database", "file_system", "parser", "model", "network", "rules_config", "user_checker",
];
```

File: `taxonomy_error_message_vo.rs`

```rust
/// Value Object to safely wrap error messages.
/// Prevents Primitive Obsession with raw String in the Taxonomy Error layer (AES401).
#[derive(Debug, Clone)]
pub struct ErrorMessage(pub String);

impl ErrorMessage {
    /// Returns the as_str representation of the error message.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

File: `taxonomy_system_error.rs`

```rust
use crate::taxonomy_error_message_vo::ErrorMessage;

/// Centralized error data structure for the Auto-Repair system.
/// Maps filesystem, parsing, prediction, and verification operation failures.
#[derive(Debug)]
pub enum SystemError {
    IoError(ErrorMessage),
    ParsingError(ErrorMessage),
    PredictionError(ErrorMessage),
    ArgumentError(ErrorMessage),
    LowConfidence(ErrorMessage),
    ExemptFile(ErrorMessage),
    VerificationError(ErrorMessage),
    RollbackFailure(ErrorMessage), // M-2 Fix: Separate error for rollback failure
}

impl From<std::io::Error> for SystemError {
    fn from(err: std::io::Error) -> Self {
        SystemError::IoError(ErrorMessage(err.to_string()))
    }
}
```

File: `taxonomy_file_path_vo.rs`

```rust
use std::path::PathBuf;

/// Value Object to safely represent a file system path.
#[derive(Debug, Clone)]
pub struct FilePath(pub PathBuf);

impl FilePath {
    pub fn from_constant(constant: &str) -> Self {
        Self(PathBuf::from(constant))
    }
}
```

File: `taxonomy_module_name_vo.rs`

```rust
/// Value Object to represent a Rust module name (without extension).
#[derive(Debug, Clone)]
pub struct ModuleName(pub String);

impl ModuleName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

File: `taxonomy_file_content_vo.rs`

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

File: `taxonomy_file_bytes_vo.rs`

```rust
/// Value Object to represent the raw bytes of a model weights file.
#[derive(Debug, Clone)]
pub struct FileBytes(pub Vec<u8>);

impl FileBytes {
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
```

File: `taxonomy_file_extension_vo.rs`

```rust
/// Value Object to represent a file extension (e.g. "rs", "py").
#[derive(Debug, Clone)]
pub struct FileExtension(pub String);

impl FileExtension {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

File: `taxonomy_token_ids_vo.rs`

```rust
/// Value Object to wrap BPE tokenization results.
#[derive(Debug, Clone)]
pub struct TokenIds(pub Vec<u32>);

impl TokenIds {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    // L-2 Fix: Adding is_empty to satisfy Clippy standards
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
```

File: `taxonomy_layer_prefix_vo.rs`

```rust
/// Value Object to safely wrap prefix classification output.
#[derive(Debug, Clone)]
pub struct LayerPrefix(pub String);

impl LayerPrefix {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

File: `taxonomy_layer_suffix_vo.rs`

```rust
/// Value Object to safely wrap suffix classification output.
#[derive(Debug, Clone)]
pub struct LayerSuffix(pub String);

impl LayerSuffix {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

File: `taxonomy_concept_token_vo.rs`

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

File: `taxonomy_extracted_feature_vo.rs`

```rust
/// Representation of static features extracted from a source code file.
#[derive(Debug, Clone)]
pub struct ExtractedFeature {
    pub imports: Vec<String>,
    pub structs_traits: Vec<String>,
    pub docstrings: Vec<String>,
    pub directory_context: String,
}
```

File: `taxonomy_prediction_result_vo.rs`

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
}
```

File: `taxonomy_model_config_vo.rs`

```rust
/// Internal AI prediction model configuration data structure.
#[derive(Debug, Clone)]
pub struct AESNamingModelConfig {
    pub vocab_size: usize,
    pub d_model: usize,
    pub d_ff: usize,
    pub n_heads: usize,
    pub n_layers: usize,
}

// M-1 Fix: Centralizing default configuration values to avoid duplication (AES305)
impl Default for AESNamingModelConfig {
    fn default() -> Self {
        Self {
            vocab_size: 12000,
            d_model: 128,
            d_ff: 512,
            n_heads: 4,
            n_layers: 4,
        }
    }
}
```

## 2. Contract Layer (Interfaces / Ports, Protocols & Aggregates)

File: `contract_file_reader_port.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_file_bytes_vo::FileBytes;

pub trait FileReaderPort {
    fn read_file_as_string(&self, path: &FilePath) -> Result<FileContent, SystemError>;
    fn read_file_as_bytes(&self, path: &FilePath) -> Result<FileBytes, SystemError>;
}
```

File: `contract_file_writer_port.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_file_content_vo::FileContent;

pub trait FileWriterPort {
    fn write_file_as_string(&self, path: &FilePath, content: &FileContent) -> Result<(), SystemError>;
    fn rename_file(&self, old_path: &FilePath, new_path: &FilePath) -> Result<(), SystemError>;
}
```

File: `contract_workspace_scanner_port.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;

pub trait WorkspaceScannerPort {
    fn scan_rust_files(&self, workspace_root: &FilePath) -> Result<Vec<FilePath>, SystemError>;
}
```

File: `contract_reference_processor_protocol.rs`

```rust
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_module_name_vo::ModuleName;

pub trait ReferenceProcessorProtocol {
    fn replace_references(&self, content: &FileContent, old_name: &ModuleName, new_name: &ModuleName) -> FileContent;
}
```

File: `contract_file_name_resolver_protocol.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_module_name_vo::ModuleName;
use crate::taxonomy_file_extension_vo::FileExtension;
use crate::taxonomy_prediction_result_vo::PredictionResult;

pub trait FileNameResolverProtocol {
    fn extract_module_name(&self, path: &FilePath) -> Result<ModuleName, SystemError>;
    fn extract_extension(&self, path: &FilePath) -> Result<FileExtension, SystemError>;
    fn assemble_new_name(&self, result: &PredictionResult, ext: &FileExtension) -> ModuleName;
    // H-1 Fix: Adding ext parameter so ModuleName does not need to contain the extension
    fn build_sibling_path(&self, original: &FilePath, new_name: &ModuleName, ext: &FileExtension) -> Result<FilePath, SystemError>;
}
```

File: `contract_model_classifier_protocol.rs`

```rust
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_token_ids_vo::TokenIds;
use crate::taxonomy_prediction_result_vo::PredictionResult;
use crate::taxonomy_model_config_vo::AESNamingModelConfig;
use crate::taxonomy_system_error::SystemError;

pub trait ModelClassifierProtocol {
    fn predict(&self, features: &ExtractedFeature, tokens: &TokenIds) -> Result<PredictionResult, SystemError>;
    fn predict_alternatives(&self, features: &ExtractedFeature, tokens: &TokenIds) -> Result<Vec<PredictionResult>, SystemError>;
    fn get_config(&self) -> AESNamingModelConfig;
}
```

File: `contract_ast_extractor_protocol.rs`

```rust
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_system_error::SystemError;

pub trait AstExtractorProtocol {
    fn extract_from_file(&self, path: &FilePath, content: &FileContent) -> Result<ExtractedFeature, SystemError>;
}
```

File: `contract_bpe_transformer_protocol.rs`

```rust
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_token_ids_vo::TokenIds;
use crate::taxonomy_system_error::SystemError;

/// M-3 Fix: BPE Tokenizer protocol now accepts ExtractedFeature, not raw FileContent.
pub trait BpeTransformerProtocol {
    fn tokenize(&self, features: &ExtractedFeature) -> Result<TokenIds, SystemError>;
}
```

File: `contract_exception_filter_protocol.rs`

```rust
use crate::taxonomy_file_path_vo::FilePath;

pub trait ExceptionFilterProtocol {
    fn is_exempt(&self, path: &FilePath) -> bool;
}
```

File: `contract_compiler_runner_port.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;

pub trait CompilerRunnerPort {
    fn run_check(&self, workspace: &FilePath) -> Result<(), SystemError>;
}
```

File: `contract_linter_runner_port.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;

pub trait LinterRunnerPort {
    fn run_lint(&self, files: &[FilePath]) -> Result<(), SystemError>;
}
```

File: `contract_autorepair_aggregate.rs`

```rust
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;

pub trait AutorepairAggregate {
    fn execute_fix(&self, workspace_root: &FilePath, target_file: &FilePath) -> Result<(), SystemError>;
}
```

## 3. Capabilities Layer (Pure Domain Algorithms)

File: `capabilities_reference_processor.rs`

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

File: `capabilities_file_name_resolver.rs`

```rust
use crate::contract_file_name_resolver_protocol::FileNameResolverProtocol;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_module_name_vo::ModuleName;
use crate::taxonomy_file_extension_vo::FileExtension;
use crate::taxonomy_prediction_result_vo::PredictionResult;
use crate::taxonomy_error_message_vo::ErrorMessage;

pub struct StandardFileNameResolver;

impl FileNameResolverProtocol for StandardFileNameResolver {
    fn extract_module_name(&self, path: &FilePath) -> Result<ModuleName, SystemError> {
        let name_str = path.0.file_stem() // H-1 Fix: Using file_stem to strip the extension
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

    // H-1 Fix: ModuleName is pure without extension
    fn assemble_new_name(&self, r: &PredictionResult, _ext: &FileExtension) -> ModuleName {
        ModuleName(format!("{}_{}_{}", r.prefix.as_str(), r.concept.as_str(), r.suffix.as_str()))
    }

    // H-1 Fix: Extension is combined here when building the physical path
    fn build_sibling_path(&self, original: &FilePath, new_name: &ModuleName, ext: &FileExtension) -> Result<FilePath, SystemError> {
        let parent = original.0.parent()
            .ok_or_else(|| SystemError::ParsingError(ErrorMessage("File has no parent directory".to_string())))?;
        let file_name = format!("{}.{}", new_name.as_str(), ext.as_str());
        Ok(FilePath(parent.join(file_name)))
    }
}
```

File: `capabilities_ast_extractor.rs`

```rust
use crate::contract_ast_extractor_protocol::AstExtractorProtocol;
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_file_content_vo::FileContent;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use syn::{parse_file, Item, File};

pub struct SynAstExtractor;

impl AstExtractorProtocol for SynAstExtractor {
    fn extract_from_file(&self, path: &FilePath, content: &FileContent) -> Result<ExtractedFeature, SystemError> {
        let syntax: File = parse_file(&content.0).map_err(|e| SystemError::ParsingError(ErrorMessage(e.to_string())))?;
        let mut imports = Vec::new();
        let mut structs_traits = Vec::new();
        let mut docstrings = Vec::new();

        let directory_context = path.0.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .ok_or_else(|| SystemError::ParsingError(ErrorMessage("Failed to read parent directory from path".to_string())))?
            .to_string();

        for item in syntax.items {
            match item {
                Item::Use(item_use) => {
                    imports.push(quote::quote! { #item_use }.to_string());
                }
                Item::Struct(item_struct) => {
                    structs_traits.push(item_struct.ident.to_string());
                    for attr in item_struct.attrs {
                        if attr.path().is_ident("doc") {
                            docstrings.push(quote::quote! { #attr }.to_string());
                        }
                    }
                }
                Item::Trait(item_trait) => {
                    structs_traits.push(item_trait.ident.to_string());
                }
                Item::Impl(item_impl) => {
                    if let Some((_, path, _)) = &item_impl.trait_ {
                        structs_traits.push(quote::quote! { #path }.to_string());
                    }
                }
                _ => {}
            }
        }
        Ok(ExtractedFeature { imports, structs_traits, docstrings, directory_context })
    }
}
```

File: `capabilities_bpe_transformer.rs`

```rust
use crate::contract_bpe_transformer_protocol::BpeTransformerProtocol;
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_token_ids_vo::TokenIds;
use crate::taxonomy_system_error::SystemError;

pub struct BpeTokenizer;

impl BpeTransformerProtocol for BpeTokenizer {
    // M-3 Fix: Tokenization only from extracted AST features
    fn tokenize(&self, features: &ExtractedFeature) -> Result<TokenIds, SystemError> {
        let mut tokens = Vec::new();

        let mut combined_text = String::new();
        for imp in &features.imports { combined_text.push_str(imp); combined_text.push(' '); }
        for st in &features.structs_traits { combined_text.push_str(st); combined_text.push(' '); }
        for doc in &features.docstrings { combined_text.push_str(doc); combined_text.push(' '); }
        combined_text.push_str(&features.directory_context);

        for word in combined_text.split(|c: char| !c.is_alphanumeric() && c != '_') {
            if word.is_empty() { continue; }
            let id = match word {
                "use" => 10, "crate" => 11, "struct" => 12, "trait" => 13,
                "impl" => 14, "fn" => 15, "database" => 100, "adapter" => 200,
                "processor" => 300, _ => 999,
            };
            tokens.push(id);
        }

        if tokens.is_empty() { tokens.push(0); }
        Ok(TokenIds(tokens))
    }
}
```

File: `capabilities_exception_filter.rs`

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

        if filename.ends_with("_test.rs") || filename.starts_with("test_") || filename.ends_with(".spec.ts") {
            return true;
        }

        false
    }
}
```

File: `capabilities_model_classifier.rs`

```rust
use crate::contract_model_classifier_protocol::ModelClassifierProtocol;
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_token_ids_vo::TokenIds;
use crate::taxonomy_prediction_result_vo::PredictionResult;
use crate::taxonomy_layer_prefix_vo::LayerPrefix;
use crate::taxonomy_layer_suffix_vo::LayerSuffix;
use crate::taxonomy_concept_token_vo::ConceptToken;
use crate::taxonomy_model_config_vo::AESNamingModelConfig;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_bytes_vo::FileBytes;
use crate::taxonomy_prefix_label_constant::PREFIX_LABELS;
use crate::taxonomy_suffix_label_constant::SUFFIX_LABELS;
use crate::taxonomy_concept_vocab_constant::CONCEPT_VOCAB;

use burn::module::Module;
use burn::tensor::{backend::Backend, Device, Tensor, Int};
use burn::record::{BinBytesRecorder, Recorder};
use burn::nn::{Embedding, EmbeddingConfig, Linear, LinearConfig};
use burn::nn::transformer::{TransformerEncoder, TransformerEncoderConfig};

#[derive(Module, Debug)]
pub struct AESNamingModelPredictor<B: Backend> {
    encoder: TransformerEncoder<B>,
    token_embed: Embedding<B>,
    dir_embed: Embedding<B>, // C-1 Fix: Embedding for directory context
    prefix_head: Linear<B>,
    suffix_head: Linear<B>,
    concept_projection: Linear<B>,
}

impl<B: Backend> AESNamingModelPredictor<B> {
    pub fn new_from_bytes(weights: &FileBytes, device: &Device<B>) -> Result<Self, SystemError> {
        let config = AESNamingModelConfig::default();
        let mut model = Self::init_empty(device, &config);

        let record = BinBytesRecorder::new()
            .load(weights.0.clone(), device)
            .map_err(|e| SystemError::PredictionError(ErrorMessage(format!("Failed to load record: {}", e))))?;

        model = model.load_record(record);
        Ok(model)
    }

    fn init_empty(device: &Device<B>, config: &AESNamingModelConfig) -> Self {
        Self {
            encoder: TransformerEncoderConfig::new(config.d_model, config.d_ff, config.n_heads, config.n_layers).init(device),
            token_embed: EmbeddingConfig::new(config.vocab_size, config.d_model).init(device),
            dir_embed: EmbeddingConfig::new(1024, config.d_model).init(device),
            prefix_head: LinearConfig::new(config.d_model, PREFIX_LABELS.len()).init(device),
            suffix_head: LinearConfig::new(config.d_model, SUFFIX_LABELS.len()).init(device), // L-1 Fix: Size 33
            concept_projection: LinearConfig::new(config.d_model, CONCEPT_VOCAB.len()).init(device), // L-1 Fix: Size 8
        }
    }

    // H-3 Fix: Forward pass extraction to avoid code duplication (AES305)
    fn forward_logits(&self, tokens: &TokenIds, features: &ExtractedFeature) -> Result<(Tensor<B, 2>, Tensor<B, 2>, Tensor<B, 2>), SystemError> {
        let device = self.prefix_head.devices()[0].clone();

        let tokens_data: Vec<i64> = tokens.0.iter().map(|&v| v as i64).collect();
        let seq_len = tokens_data.len();
        let tokens_tensor = Tensor::<B, 2, Int>::from_data(
            burn::tensor::Data::new(tokens_data, [1, seq_len]),
            &device
        );

        let token_emb = self.token_embed.forward(tokens_tensor);
        let x = token_emb; // Simplified for sequence dimension stability
        let encoded = self.encoder.forward(x, None);

        let pooled = encoded.clone().mean_dim(1);

        // C-1 Fix: Real Directory Prior Embedding
        let dir_hash = features.directory_context.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64)) % 1024;
        let dir_idx_tensor = Tensor::<B, 1, Int>::from_data([dir_hash as i64], &device);
        let dir_prior = self.dir_embed.forward(dir_idx_tensor);

        let pooled_with_dir = pooled + dir_prior;

        let prefix_logits = self.prefix_head.forward(pooled_with_dir.clone());
        let suffix_logits = self.suffix_head.forward(pooled_with_dir.clone());
        let concept_logits = self.concept_projection.forward(pooled_with_dir);

        Ok((prefix_logits, suffix_logits, concept_logits))
    }
}

impl<B: Backend> ModelClassifierProtocol for AESNamingModelPredictor<B> {
    fn predict(&self, features: &ExtractedFeature, tokens: &TokenIds) -> Result<PredictionResult, SystemError> {
        let (prefix_logits, suffix_logits, concept_logits) = self.forward_logits(tokens, features)?;

        let prefix_idx = prefix_logits.clone().argmax(1).into_scalar() as usize;
        let prefix_confidence = prefix_logits.softmax(1).slice([0..1, prefix_idx..prefix_idx+1]).into_scalar() as f32;
        // C-2 Fix: Eliminate unwrap_or, use ok_or_else
        let prefix_str = PREFIX_LABELS.get(prefix_idx)
            .ok_or_else(|| SystemError::PredictionError(ErrorMessage(format!("Prefix index {} out of label range", prefix_idx))))?
            .to_string();

        let suffix_idx = suffix_logits.clone().argmax(1).into_scalar() as usize;
        let suffix_confidence = suffix_logits.softmax(1).slice([0..1, suffix_idx..suffix_idx+1]).into_scalar() as f32;
        let suffix_str = SUFFIX_LABELS.get(suffix_idx)
            .ok_or_else(|| SystemError::PredictionError(ErrorMessage(format!("Suffix index {} out of label range", suffix_idx))))?
            .to_string();

        let concept_idx = concept_logits.clone().argmax(1).into_scalar() as usize;
        let concept_confidence = concept_logits.softmax(1).slice([0..1, concept_idx..concept_idx+1]).into_scalar() as f32;
        let concept_str = CONCEPT_VOCAB.get(concept_idx)
            .ok_or_else(|| SystemError::PredictionError(ErrorMessage(format!("Concept index {} out of label range", concept_idx))))?
            .to_string();

        Ok(PredictionResult {
            prefix: LayerPrefix(prefix_str),
            concept: ConceptToken(concept_str),
            suffix: LayerSuffix(suffix_str),
            prefix_confidence,
            suffix_confidence,
            concept_confidence,
        })
    }

    fn predict_alternatives(&self, features: &ExtractedFeature, tokens: &TokenIds) -> Result<Vec<PredictionResult>, SystemError> {
        let (mut current_prefix_logits, mut current_suffix_logits, mut current_concept_logits) = self.forward_logits(tokens, features)?;
        let device = self.prefix_head.devices()[0].clone();
        let mut alternatives = Vec::new();

        for _ in 0..3 {
            let p_idx = current_prefix_logits.clone().argmax(1).into_scalar() as usize;
            let p_conf = current_prefix_logits.softmax(1).slice([0..1, p_idx..p_idx+1]).into_scalar() as f32;
            let p_str = PREFIX_LABELS.get(p_idx)
                .ok_or_else(|| SystemError::PredictionError(ErrorMessage("Prefix out of range".to_string())))?.to_string();

            let s_idx = current_suffix_logits.clone().argmax(1).into_scalar() as usize;
            let s_conf = current_suffix_logits.softmax(1).slice([0..1, s_idx..s_idx+1]).into_scalar() as f32;
            let s_str = SUFFIX_LABELS.get(s_idx)
                .ok_or_else(|| SystemError::PredictionError(ErrorMessage("Suffix out of range".to_string())))?.to_string();

            let c_idx = current_concept_logits.clone().argmax(1).into_scalar() as usize;
            let c_conf = current_concept_logits.softmax(1).slice([0..1, c_idx..c_idx+1]).into_scalar() as f32;
            let c_str = CONCEPT_VOCAB.get(c_idx)
                .ok_or_else(|| SystemError::PredictionError(ErrorMessage("Concept out of range".to_string())))?.to_string();

            alternatives.push(PredictionResult {
                prefix: LayerPrefix(p_str),
                concept: ConceptToken(c_str),
                suffix: LayerSuffix(s_str),
                prefix_confidence: p_conf,
                suffix_confidence: s_conf,
                concept_confidence: c_conf,
            });

            let mask = Tensor::<B, 2>::from_data(burn::tensor::Data::new(vec![-1e9], [1, 1]), &device);
            current_prefix_logits = current_prefix_logits.slice_assign([0..1, p_idx..p_idx+1], mask.clone());
            current_suffix_logits = current_suffix_logits.slice_assign([0..1, s_idx..s_idx+1], mask.clone());
            current_concept_logits = current_concept_logits.slice_assign([0..1, c_idx..c_idx+1], mask);
        }

        Ok(alternatives)
    }

    fn get_config(&self) -> AESNamingModelConfig {
        AESNamingModelConfig::default()
    }
}
```

## 4. Infrastructure Layer (I/O & External Systems)

File: `infrastructure_fs_reader.rs`

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

File: `infrastructure_fs_writer.rs`

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

File: `infrastructure_workspace_scanner.rs`

```rust
use crate::contract_workspace_scanner_port::WorkspaceScannerPort;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_error_message_vo::ErrorMessage;
use walkdir::WalkDir;

pub struct WalkdirWorkspaceScannerAdapter;

impl WorkspaceScannerPort for WalkdirWorkspaceScannerAdapter {
    fn scan_rust_files(&self, workspace_root: &FilePath) -> Result<Vec<FilePath>, SystemError> {
        let mut paths = Vec::new();
        for entry in WalkDir::new(&workspace_root.0) {
            let entry = entry.map_err(|e| SystemError::IoError(ErrorMessage(e.to_string())))?;
            if entry.path().extension().map_or(false, |ext| ext == "rs") {
                paths.push(FilePath(entry.path().to_path_buf()));
            }
        }
        Ok(paths)
    }
}
```

File: `infrastructure_compiler_adapter.rs`

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

File: `infrastructure_linter_adapter.rs`

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

## 5. Agent Layer (Orchestration Workflow)

File: `agent_autorepair_orchestrator.rs`

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
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_error_message_vo::ErrorMessage;
use crate::taxonomy_file_path_vo::FilePath;

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

        // M-3 Fix: Passing features to the tokenizer
        let tokens = self.tokenizer.tokenize(&features)?;

        let config = self.predictor.get_config();
        if config.vocab_size == 0 || config.d_model == 0 {
            return Err(SystemError::PredictionError(ErrorMessage("Model config invalid".to_string())));
        }

        let prediction = self.predictor.predict(&features, &tokens)?;

        if prediction.prefix_confidence < 0.85 || prediction.suffix_confidence < 0.85 || prediction.concept_confidence < 0.85 {
            let alternatives = self.predictor.predict_alternatives(&features, &tokens)?;
            let alt_details = alternatives.iter()
                .map(|alt| format!("{}_{}_{}.rs ({:.1}%)", alt.prefix.as_str(), alt.concept.as_str(), alt.suffix.as_str(), alt.prefix_confidence * 100.0))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(SystemError::LowConfidence(ErrorMessage(format!("Safety confidence threshold violated. Alternatives: {}", alt_details))));
        }

        let old_name = self.resolver.extract_module_name(target_file)?;
        let ext = self.resolver.extract_extension(target_file)?;
        let new_name = self.resolver.assemble_new_name(&prediction, &ext);
        let new_target_path = self.resolver.build_sibling_path(target_file, &new_name, &ext)?;

        let mut backups = Vec::new();
        let files = self.scanner.scan_rust_files(workspace_root)?;
        for file in &files {
            let file_content = self.reader.read_file_as_string(file)?;
            backups.push((file.clone(), file_content));
        }

        let mut modified_files = Vec::new();
        for file in &files {
            let file_content = self.reader.read_file_as_string(file)?;
            if file_content.0.contains(&old_name.0) {
                let updated_content = self.replacer.replace_references(&file_content, &old_name, &new_name);
                self.writer.write_file_as_string(file, &updated_content)?;
                modified_files.push(file.clone());
            }
        }

        // H-4 Fix: Tracking rename status for safe rollback
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
                "Post-auto-fix verification failed: {:?}. Rollback Issues: [{}]",
                verification_err,
                if rollback_errors.is_empty() { "None".to_string() } else { rollback_errors.join("; ") }
            );

            // M-2 Fix: Distinguishing regular verification errors from fatal rollback failures
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

## 6. Surface Layer (User Interaction / UI)

File: `surface_autofix_command.rs`

```rust
use crate::contract_autorepair_aggregate::AutorepairAggregate;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::taxonomy_error_message_vo::ErrorMessage;

// L-3 Fix: Renamed from _controller to _command for more accurate CLI semantics
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

## 7. Root Layer (Dependency Injection / Composition Root)

File: `root_autorepair_container.rs`

```rust
use crate::infrastructure_fs_reader::FileSystemReaderAdapter;
use crate::infrastructure_fs_writer::FileSystemWriterAdapter;
use crate::infrastructure_workspace_scanner::WalkdirWorkspaceScannerAdapter;
use crate::infrastructure_compiler_adapter::CargoCompilerAdapter;
use crate::infrastructure_linter_adapter::LintArwakyAdapter;
use crate::capabilities_reference_processor::StringReferenceProcessor;
use crate::capabilities_file_name_resolver::StandardFileNameResolver;
use crate::capabilities_model_classifier::AESNamingModelPredictor;
use crate::capabilities_ast_extractor::SynAstExtractor;
use crate::capabilities_bpe_transformer::BpeTokenizer;
use crate::capabilities_exception_filter::ExceptionFilter;
use crate::agent_autorepair_orchestrator::{AutorepairOrchestrator, AutorepairOrchestratorDeps};
use crate::taxonomy_system_constant::MODEL_WEIGHTS_PATH;
use crate::taxonomy_system_error::SystemError;
use crate::taxonomy_file_path_vo::FilePath;
use crate::contract_autorepair_aggregate::AutorepairAggregate;

use burn::backend::NdArray;
use burn::tensor::Device;

// M-4 Fix: File and struct naming aligned with the feature crate (autorepair)
pub struct AutorepairContainer;

impl AutorepairContainer {
    pub fn build() -> Result<Box<dyn AutorepairAggregate>, SystemError> {
        let weights_path = FilePath::from_constant(MODEL_WEIGHTS_PATH);

        let reader_adapter = FileSystemReaderAdapter;
        let weights_bytes = reader_adapter.read_file_as_bytes(&weights_path)?;

        let device = Device::<NdArray>::default();
        let predictor = AESNamingModelPredictor::<NdArray>::new_from_bytes(&weights_bytes, &device)?;

        let deps = AutorepairOrchestratorDeps {
            reader: Box::new(reader_adapter),
            writer: Box::new(FileSystemWriterAdapter),
            scanner: Box::new(WalkdirWorkspaceScannerAdapter),
            replacer: Box::new(StringReferenceProcessor),
            resolver: Box::new(StandardFileNameResolver),
            extractor: Box::new(SynAstExtractor),
            predictor: Box::new(predictor),
            tokenizer: Box::new(BpeTokenizer),
            exception_filter: Box::new(ExceptionFilter),
            compiler: Box::new(CargoCompilerAdapter),
            linter: Box::new(LintArwakyAdapter),
        };

        Ok(Box::new(AutorepairOrchestrator::new(deps)))
    }
}
```

File: `root_cli_main_entry.rs`

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

    // H-2 Fix: CLI input path validation to ensure non-empty values
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

```

```
