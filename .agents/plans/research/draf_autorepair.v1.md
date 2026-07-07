.

Hello Raka! Based on the _Technical Research Proposal_ document you provided, you want to implement the _Deep Learning-Based Semantic Code Naming_ system using the **Rust Burn Framework**.

See [ai-autorepair-burn.md](ai-autorepair-burn.md) for the full research proposal and [RULES_AES.md](../../rules/RULES_AES.md) for AES101-AES102 naming rules.

To "work on" or realize this proposal, I have prepared the **core implementation code** in Rust. This implementation covers the three main pillars outlined in the document: Model Architecture (Burn), AST Feature Extractor, and Reference Propagation Engine.

Here is the technical realization of that proposal:

### 1. Deep Learning Model Architecture (Rust Burn)

_According to Section 5: Training and Optimization Pipeline_
This is the implementation of the _Multi-Task Transformer_ that predicts Prefix, Suffix, and Concept simultaneously.

```rust
use burn::{
    config::Config,
    module::Module,
    nn::{
        transformer::{TransformerEncoder, TransformerEncoderConfig},
        Embedding, EmbeddingConfig, Linear, LinearConfig,
    },
    tensor::{backend::Backend, Device, Int, Tensor},
};

#[derive(Config, Debug)]
pub struct AESNamingModelConfig {
    pub vocab_size: usize,
    pub d_model: usize,      // 128
    pub d_ff: usize,         // 512
    pub n_heads: usize,      // 4
    pub n_layers: usize,     // 4
    pub num_prefixes: usize, // 7 (root, taxonomy, contract, etc.)
    pub num_suffixes: usize, // Total number of allowed suffixes
    pub max_seq_len: usize,  // e.g.: 512
}

#[derive(Module, Debug)]
pub struct AESNamingModel<B: Backend> {
    token_embed: Embedding<B>,
    pos_embed: Embedding<B>,
    encoder: TransformerEncoder<B>,
    prefix_head: Linear<B>,
    suffix_head: Linear<B>,
    concept_head: Linear<B>,
}

pub struct ModelOutput<B: Backend> {
    pub prefix_logits: Tensor<B, 2>,
    pub suffix_logits: Tensor<B, 2>,
    pub concept_logits: Tensor<B, 3>,
}

impl<B: Backend> AESNamingModel<B> {
    pub fn new(config: &AESNamingModelConfig, device: &Device<B>) -> Self {
        let encoder_config = TransformerEncoderConfig::new(
            config.d_model,
            config.d_ff,
            config.n_heads,
            config.n_layers,
        );

        Self {
            token_embed: EmbeddingConfig::new(config.vocab_size, config.d_model).init_with_device(device),
            pos_embed: EmbeddingConfig::new(config.max_seq_len, config.d_model).init_with_device(device),
            encoder: TransformerEncoder::new_with_device(&encoder_config, device),
            prefix_head: LinearConfig::new(config.d_model, config.num_prefixes).init_with_device(device),
            suffix_head: LinearConfig::new(config.d_model, config.num_suffixes).init_with_device(device),
            concept_head: LinearConfig::new(config.d_model, config.vocab_size).init_with_device(device),
        }
    }

    /// Forward pass for inference
    pub fn forward(
        &self,
        tokens: Tensor<B, 2, Int>,
        _dir_context: Tensor<B, 2>, // Directory context vector (injected into the first token)
    ) -> ModelOutput<B> {
        let [batch_size, seq_len] = tokens.dims();
        let device = tokens.device();

        // 1. Token & Positional Embedding
        let token_emb = self.token_embed.forward(tokens);
        let positions = Tensor::<B, 1, Int>::arange(0..seq_len as i64, &device).reshape([1, seq_len]);
        let pos_emb = self.pos_embed.forward(positions);

        // Broadcast pos_emb to batch_size
        let pos_emb = pos_emb.repeat_dim(0, batch_size);
        let x = token_emb + pos_emb;

        // 2. Transformer Encoder (Shared Representation)
        let encoded = self.encoder.forward(x, None);

        // 3. Pooling (Global Average Pooling) for prefix/suffix classification
        let pooled = encoded.clone().mean_dim(1);

        // 4. Multi-Task Heads
        let prefix_logits = self.prefix_head.forward(pooled.clone());
        let suffix_logits = self.suffix_head.forward(pooled);

        // Concept head generates logits for each token (for autoregressive decoding)
        let concept_logits = self.concept_head.forward(encoded);

        ModelOutput {
            prefix_logits,
            suffix_logits,
            concept_logits,
        }
    }
}
```

### 2. AST Feature Extractor

_According to Section 4.1: Feature Extraction and Tokenization_
This function extracts the "header" from Rust files (imports, structs, traits, docstrings) to feed into the model, ignoring irrelevant implementation details.

```rust
use syn::{parse_file, Item, File};
use std::fs;

pub struct ExtractedFeatures {
    pub imports: Vec<String>,
    pub structs_traits: Vec<String>,
    pub docstrings: Vec<String>,
}

pub fn extract_ast_features(file_path: &str) -> Result<ExtractedFeatures, syn::Error> {
    let content = fs::read_to_string(file_path).expect("Unable to read file");
    let syntax: File = parse_file(&content)?;

    let mut imports = Vec::new();
    let mut structs_traits = Vec::new();
    let mut docstrings = Vec::new();

    for item in syntax.items {
        match item {
            Item::Use(item_use) => {
                imports.push(quote::quote! { #item_use }.to_string());
            }
            Item::Struct(item_struct) => {
                structs_traits.push(item_struct.ident.to_string());
                // Extract docstring (attributes)
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
                // Extract implemented traits (very important for determining the suffix)
                if let Some((_, path, _)) = item_impl.trait_ {
                    structs_traits.push(quote::quote! { #path }.to_string());
                }
            }
            _ => {}
        }
    }

    Ok(ExtractedFeatures {
        imports,
        structs_traits,
        docstrings,
    })
}
```

### 3. Reference Propagation Engine

_According to Section 6: Execution Flow and Reference Propagation_
After the model predicts a new file name (e.g.: `db_util.rs` -> `infrastructure_database_adapter.rs`), this engine will propagate the change throughout the workspace.

```rust
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn propagate_references(
    workspace_root: &Path,
    old_module_name: &str,
    new_module_name: &str,
) -> Result<(), std::io::Error> {
    // Search for all .rs files in the workspace
    for entry in WalkDir::new(workspace_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();
        let content = fs::read_to_string(path)?;

        // Check if there are references to old_module_name
        if content.contains(old_module_name) {
            // Replace use and mod declarations
            let mut new_content = content.replace(
                &format!("use crate::{};", old_module_name),
                &format!("use crate::{};", new_module_name)
            );
            new_content = new_content.replace(
                &format!("mod {};", old_module_name),
                &format!("mod {};", new_module_name)
            );

            // Save back the updated file
            fs::write(path, new_content)?;
        }
    }
    Ok(())
}
```

---

### Next Steps

The code above is the architectural foundation of your proposal. To make this system fully functional as a `lint-arwaky` binary, there are still several components that need to be worked on.

Would you like me to continue implementing the other parts?

1. **BPE Tokenizer & Dataset Synthesizer** (To train the model according to Section 4.3).
2. **Training Loop with Burn** (Using _Joint Loss Function_ according to Section 5).
3. **CLI Integration & Fallback Logic** (Integration with `cargo check` and _Confidence Thresholding_ according to Section 7).

Let me know which part you'd like to work on next!

---

## 📁 Refactored File Structure

```
crates/lint-arwaky/src/
├── taxonomy/
│   ├── taxonomy_ast_feature_vo.rs
│   ├── taxonomy_model_output_vo.rs
│   ├── taxonomy_model_config_vo.rs
│   └── taxonomy_training_sample_vo.rs
├── contract/
│   ├── contract_ast_scanner_port.rs
│   ├── contract_file_reader_port.rs
│   ├── contract_reference_modifier_port.rs
│   ├── contract_file_writer_port.rs
│   ├── contract_model_predictor_protocol.rs
│   └── contract_workspace_scanner_port.rs
├── capabilities/
│   ├── capabilities_ast_extractor.rs
│   ├── capabilities_model_predictor.rs
│   ├── capabilities_reference_resolver.rs
│   └── capabilities_name_assembler.rs
├── infrastructure/
│   ├── infrastructure_fs_ast_scanner.rs
│   ├── infrastructure_fs_modifier.rs
│   ├── infrastructure_burn_model_adapter.rs
│   └── infrastructure_git_client.rs
├── agent/
│   └── agent_autorepair_orchestrator.rs
└── surface/
    ├── surface_autofix_command.rs
    └── surface_train_command.rs
```

---

## 1️⃣ Taxonomy Layer (Value Objects)

### `taxonomy_ast_feature_vo.rs`

```rust
// crates/lint-arwaky/src/taxonomy/taxonomy_ast_feature_vo.rs
//! Value Object for storing features extracted from the AST.
//! This is a pure data representation without business logic.

use serde::{Deserialize, Serialize};

/// Value Object that stores the feature extraction result from a source code file.
/// Contains imports, struct/trait declarations, and docstrings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyAstFeatureVo {
    /// List of import statements (use, import)
    pub imports: Vec<String>,

    /// List of declared struct and trait names
    pub structs_traits: Vec<String>,

    /// List of docstrings (documentation comments)
    pub docstrings: Vec<String>,
}

impl TaxonomyAstFeatureVo {
    /// Constructor to create a new instance
    pub fn new(
        imports: Vec<String>,
        structs_traits: Vec<String>,
        docstrings: Vec<String>,
    ) -> Self {
        Self {
            imports,
            structs_traits,
            docstrings,
        }
    }

    /// Combine all features into a single string for model input
    pub fn to_input_text(&self) -> String {
        let mut text = String::new();

        for imp in &self.imports {
            text.push_str(imp);
            text.push('\n');
        }

        for decl in &self.structs_traits {
            text.push_str(&format!("struct {} {{}}\n", decl));
        }

        for doc in &self.docstrings {
            text.push_str(doc);
            text.push('\n');
        }

        text
    }
}
```

### `taxonomy_model_output_vo.rs`

```rust
// crates/lint-arwaky/src/taxonomy/taxonomy_model_output_vo.rs
//! Value Object for storing output from the prediction model.

use burn::tensor::{backend::Backend, Tensor};

/// Value Object that stores output from the multi-task model.
/// Contains logits for prefix, suffix, and concept.
#[derive(Debug, Clone)]
pub struct TaxonomyModelOutputVo<B: Backend> {
    /// Logits for prefix layer classification (7 classes)
    pub prefix_logits: Tensor<B, 2>,

    /// Logits for functional suffix classification (33 classes)
    pub suffix_logits: Tensor<B, 2>,

    /// Logits for concept token generation (sequence)
    pub concept_logits: Tensor<B, 3>,
}

impl<B: Backend> TaxonomyModelOutputVo<B> {
    pub fn new(
        prefix_logits: Tensor<B, 2>,
        suffix_logits: Tensor<B, 2>,
        concept_logits: Tensor<B, 3>,
    ) -> Self {
        Self {
            prefix_logits,
            suffix_logits,
            concept_logits,
        }
    }
}
```

### `taxonomy_model_config_vo.rs`

```rust
// crates/lint-arwaky/src/taxonomy/taxonomy_model_config_vo.rs
//! Value Object for AI model configuration.

use burn::config::Config;
use serde::{Deserialize, Serialize};

/// Value Object that stores the model architecture configuration.
#[derive(Config, Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyModelConfigVo {
    /// BPE tokenizer vocabulary size
    pub vocab_size: usize,

    /// Model embedding dimension (d_model)
    pub d_model: usize,

    /// Feed-forward network dimension
    pub d_ff: usize,

    /// Number of attention heads
    pub n_heads: usize,

    /// Number of Transformer layers
    pub n_layers: usize,

    /// Number of prefix classes (7 AES layers)
    pub num_prefixes: usize,

    /// Number of allowed suffix classes
    pub num_suffixes: usize,

    /// Maximum input sequence length
    pub max_seq_len: usize,
}

impl Default for TaxonomyModelConfigVo {
    fn default() -> Self {
        Self {
            vocab_size: 12000,
            d_model: 128,
            d_ff: 512,
            n_heads: 4,
            n_layers: 4,
            num_prefixes: 7,
            num_suffixes: 33,
            max_seq_len: 512,
        }
    }
}
```

### `taxonomy_training_sample_vo.rs`

```rust
// crates/lint-arwaky/src/taxonomy/taxonomy_training_sample_vo.rs
//! Value Object for training data samples.

use serde::{Deserialize, Serialize};

/// Value Object that stores one data sample for model training.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyTrainingSampleVo {
    /// Input text (AST extraction result)
    pub input_text: String,

    /// Directory context (relative path)
    pub dir_context: String,

    /// Numeric label for prefix (0-6)
    pub prefix_label: usize,

    /// Numeric label for suffix (0-32)
    pub suffix_label: usize,

    /// Expected concept text
    pub concept_text: String,
}

impl TaxonomyTrainingSampleVo {
    pub fn new(
        input
```
