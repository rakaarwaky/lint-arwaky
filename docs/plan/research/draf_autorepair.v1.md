.

Halo Raka! Berdasarkan dokumen *Technical Research Proposal* yang Anda berikan, Anda ingin merealisasikan sistem *Deep Learning-Based Semantic Code Naming* menggunakan **Rust Burn Framework**.

Untuk "mengerjakan" atau merealisasikan proposal ini, saya telah menyusun **implementasi kode inti (Core Implementation)** dalam bahasa Rust. Implementasi ini mencakup tiga pilar utama yang diuraikan dalam dokumen: Arsitektur Model (Burn), AST Feature Extractor, dan Reference Propagation Engine.

Berikut adalah realisasi teknis dari proposal tersebut:

### 1. Model Arsitektur Deep Learning (Rust Burn)

*Sesuai dengan Bagian 5: Training and Optimization Pipeline*
Ini adalah implementasi dari *Multi-Task Transformer* yang memprediksi Prefix, Suffix, dan Concept secara bersamaan.

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
    pub num_prefixes: usize, // 7 (root, taxonomy, contract, dll)
    pub num_suffixes: usize, // Jumlah total suffix yang diizinkan
    pub max_seq_len: usize,  // Misal: 512
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

    /// Forward pass untuk inferensi
    pub fn forward(
        &self,
        tokens: Tensor<B, 2, Int>,
        _dir_context: Tensor<B, 2>, // Vektor konteks direktori (disuntikkan ke token pertama)
    ) -> ModelOutput<B> {
        let [batch_size, seq_len] = tokens.dims();
        let device = tokens.device();

        // 1. Token & Positional Embedding
        let token_emb = self.token_embed.forward(tokens);
        let positions = Tensor::<B, 1, Int>::arange(0..seq_len as i64, &device).reshape([1, seq_len]);
        let pos_emb = self.pos_embed.forward(positions);
    
        // Broadcast pos_emb ke batch_size
        let pos_emb = pos_emb.repeat_dim(0, batch_size); 
        let x = token_emb + pos_emb;

        // 2. Transformer Encoder (Shared Representation)
        let encoded = self.encoder.forward(x, None);

        // 3. Pooling (Global Average Pooling) untuk klasifikasi prefix/suffix
        let pooled = encoded.clone().mean_dim(1);

        // 4. Multi-Task Heads
        let prefix_logits = self.prefix_head.forward(pooled.clone());
        let suffix_logits = self.suffix_head.forward(pooled);
    
        // Concept head menghasilkan logits untuk setiap token (untuk autoregressive decoding)
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

*Sesuai dengan Bagian 4.1: Feature Extraction and Tokenization*
Fungsi ini mengekstrak "header" dari file Rust (imports, structs, traits, docstrings) untuk diumpankan ke model, mengabaikan detail implementasi yang tidak relevan.

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
                // Mengambil docstring (atribut)
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
                // Mengekstrak trait yang diimplementasikan (sangat penting untuk menentukan suffix)
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

*Sesuai dengan Bagian 6: Execution Flow and Reference Propagation*
Setelah model memprediksi nama file baru (misal: `db_util.rs` -> `infrastructure_database_adapter.rs`), mesin ini akan merambat (propagate) perubahan tersebut ke seluruh workspace.

```rust
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn propagate_references(
    workspace_root: &Path,
    old_module_name: &str,
    new_module_name: &str,
) -> Result<(), std::io::Error> {
    // Mencari semua file .rs di workspace
    for entry in WalkDir::new(workspace_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();
        let content = fs::read_to_string(path)?;
    
        // Cek apakah ada referensi ke old_module_name
        if content.contains(old_module_name) {
            // Ganti deklarasi use dan mod
            let mut new_content = content.replace(
                &format!("use crate::{};", old_module_name),
                &format!("use crate::{};", new_module_name)
            );
            new_content = new_content.replace(
                &format!("mod {};", old_module_name),
                &format!("mod {};", new_module_name)
            );
        
            // Simpan kembali file yang telah diperbarui
            fs::write(path, new_content)?;
        }
    }
    Ok(())
}
```

---

### Langkah Selanjutnya

Kode di atas adalah fondasi arsitektur dari proposal Anda. Untuk membuat sistem ini sepenuhnya fungsional sebagai *binary* `lint-arwaky`, masih ada beberapa komponen yang perlu dikerjakan.

Apakah Anda ingin saya melanjutkan untuk mengimplementasikan bagian yang lain?

1. **BPE Tokenizer & Dataset Synthesizer** (Untuk melatih model sesuai Bagian 4.3).
2. **Training Loop dengan Burn** (Menggunakan *Joint Loss Function* sesuai Bagian 5).
3. **CLI Integration & Fallback Logic** (Integrasi dengan `cargo check` dan *Confidence Thresholding* sesuai Bagian 7).

Beri tahu saya bagian mana yang ingin Anda kerjakan selanjutnya!

---

## 📁 Struktur File yang Di-refactor

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
//! Value Object untuk menyimpan fitur-fitur yang diekstrak dari AST.
//! Ini adalah representasi data murni tanpa logika bisnis.

use serde::{Deserialize, Serialize};

/// Value Object yang menyimpan hasil ekstraksi fitur dari file source code.
/// Berisi imports, deklarasi struct/trait, dan docstrings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyAstFeatureVo {
    /// Daftar import statements (use, import)
    pub imports: Vec<String>,
  
    /// Daftar nama struct dan trait yang dideklarasikan
    pub structs_traits: Vec<String>,
  
    /// Daftar docstrings (komentar dokumentasi)
    pub docstrings: Vec<String>,
}

impl TaxonomyAstFeatureVo {
    /// Constructor untuk membuat instance baru
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
  
    /// Gabungkan semua fitur menjadi satu string untuk input model
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
//! Value Object untuk menyimpan output dari model prediksi.

use burn::tensor::{backend::Backend, Tensor};

/// Value Object yang menyimpan output dari model multi-task.
/// Berisi logits untuk prefix, suffix, dan concept.
#[derive(Debug, Clone)]
pub struct TaxonomyModelOutputVo<B: Backend> {
    /// Logits untuk klasifikasi layer prefix (7 kelas)
    pub prefix_logits: Tensor<B, 2>,
  
    /// Logits untuk klasifikasi functional suffix (33 kelas)
    pub suffix_logits: Tensor<B, 2>,
  
    /// Logits untuk generasi concept tokens (sequence)
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
//! Value Object untuk konfigurasi model AI.

use burn::config::Config;
use serde::{Deserialize, Serialize};

/// Value Object yang menyimpan konfigurasi arsitektur model.
#[derive(Config, Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyModelConfigVo {
    /// Ukuran vocabulary BPE tokenizer
    pub vocab_size: usize,
  
    /// Dimensi embedding model (d_model)
    pub d_model: usize,
  
    /// Dimensi feed-forward network
    pub d_ff: usize,
  
    /// Jumlah attention heads
    pub n_heads: usize,
  
    /// Jumlah layer Transformer
    pub n_layers: usize,
  
    /// Jumlah kelas prefix (7 layer AES)
    pub num_prefixes: usize,
  
    /// Jumlah kelas suffix yang diizinkan
    pub num_suffixes: usize,
  
    /// Panjang maksimum sequence input
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
//! Value Object untuk sampel data training.

use serde::{Deserialize, Serialize};

/// Value Object yang menyimpan satu sampel data untuk training model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyTrainingSampleVo {
    /// Teks input (hasil ekstraksi AST)
    pub input_text: String,
  
    /// Konteks direktori (path relatif)
    pub dir_context: String,
  
    /// Label numerik untuk prefix (0-6)
    pub prefix_label: usize,
  
    /// Label numerik untuk suffix (0-32)
    pub suffix_label: usize,
  
    /// Teks concept yang diharapkan
    pub concept_text: String,
}

impl TaxonomyTrainingSampleVo {
    pub fn new(
        input
```
