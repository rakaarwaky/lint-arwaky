in id    pub importsala hadraft ayng saya buat tolong adna berikan kritik dan saranasaja # Implementation Draft: AI Auto-Repair Model (AES Dogfooding Architecture)

Draft ini mendemonstrasikan bagaimana fitur AI Auto-Repair yang menggunakan Rust Burn diimplementasikan di dalam `lint-arwaky` secara *dogfooding* (mematuhi aturan 7-layer AES secara ketat).

Kode tidak lagi ditulis secara monolitik, melainkan dipecah menjadi layer-layer spesifik: **Taxonomy, Contract, Capabilities, Infrastructure, dan Agent**.

---

## 1. Taxonomy Layer (Data, Value Objects)

Layer ini hanya berisi struktur data pasif (Value Objects) tanpa logika eksekusi I/O atau perhitungan berat.

**File:** `taxonomy_model_config_vo.rs`

```rust
use burn::config::Config;

#[derive(Config, Debug)]
pub struct AESNamingModelConfigVO {
    pub vocab_size: usize,
    pub d_model: usize,
    pub d_ff: usize,
    pub n_heads: usize,
    pub n_layers: usize,
    pub num_prefixes: usize,
    pub num_suffixes: usize,
    pub max_seq_len: usize,
}
```

**File:** `taxonomy_extracted_feature_vo.rs`

```rust
pub struct ExtractedFeatureVO {
: Vec<String>,
    pub structs_traits: Vec<String>,
    pub docstrings: Vec<String>,
}
```

**File:** `taxonomy_model_output_vo.rs`

```rust
use burn::tensor::{backend::Backend, Tensor};

pub struct ModelOutputVO<B: Backend> {
    pub prefix_logits: Tensor<B, 2>,
    pub suffix_logits: Tensor<B, 2>,
    pub concept_logits: Tensor<B, 3>,
}
```

---

## 2. Contract Layer (Interfaces / Ports & Protocols)

Mendefinisikan *boundaries* antar layer agar Capabilities dan Infrastructure tidak saling terikat secara langsung (dependency inversion).

**File:** `contract_model_predictor_protocol.rs`

```rust
use crate::taxonomy::{AESNamingModelConfigVO, ExtractedFeatureVO, ModelOutputVO};
use burn::tensor::backend::Backend;

/// Diimplementasikan oleh layer Capabilities
pub trait ModelPredictorProtocol<B: Backend> {
    fn predict(&self, features: &ExtractedFeatureVO) -> ModelOutputVO<B>;
}
```

**File:** `contract_ast_scanner_port.rs`

```rust
use crate::taxonomy::ExtractedFeatureVO;
use std::path::Path;

/// Diimplementasikan oleh layer Infrastructure
pub trait AstScannerPort {
    fn scan_file_features(&self, path: &Path) -> Result<ExtractedFeatureVO, String>;
}
```

**File:** `contract_reference_modifier_port.rs`

```rust
use std::path::Path;

/// Diimplementasikan oleh layer Infrastructure
pub trait ReferenceModifierPort {
    fn propagate_rename(&self, workspace_root: &Path, old_name: &str, new_name: &str) -> Result<(), String>;
}
```

---

## 3. Capabilities Layer (Pure Business Logic)

Layer ini berisi logika prediksi (Burn Model) dan *parsing* (Syn AST). Tidak boleh ada akses *file system* di sini.

**File:** `capabilities_model_predictor.rs`

```rust
use burn::module::Module;
use burn::tensor::{backend::Backend, Device, Int, Tensor};
// ... imports lain (Embedding, TransformerEncoder, Linear) ...
use crate::taxonomy::{AESNamingModelConfigVO, ExtractedFeatureVO, ModelOutputVO};
use crate::contract::ModelPredictorProtocol;

#[derive(Module, Debug)]
pub struct AESNamingModelPredictor<B: Backend> {
    // Definisi layer model...
}

impl<B: Backend> ModelPredictorProtocol<B> for AESNamingModelPredictor<B> {
    fn predict(&self, features: &ExtractedFeatureVO) -> ModelOutputVO<B> {
        // 1. Ubah ExtractedFeatureVO menjadi tensor
        // 2. Lakukan forward pass pada self.encoder, dst...
        // 3. Return ModelOutputVO
        todo!("Implement pure inference forward pass")
    }
}
```

**File:** `capabilities_ast_extractor.rs`

```rust
use crate::taxonomy::ExtractedFeatureVO;
use syn::{parse_file, File};

/// Fungsi murni (pure function) tanpa baca tulis disk
pub fn extract_ast_from_string(content: &str) -> Result<ExtractedFeatureVO, syn::Error> {
    let syntax: File = parse_file(content)?;
    // Ekstraksi murni AST ke dalam VO...
    Ok(ExtractedFeatureVO {
        imports: vec![],
        structs_traits: vec![],
        docstrings: vec![],
    })
}
```

---

## 4. Infrastructure Layer (I/O & External Systems)

Layer ini mengimplementasikan Port dari Contract layer. Layer ini menangani I/O secara langsung (akses Disk/OS).

**File:** `infrastructure_fs_ast_scanner.rs`

```rust
use crate::contract::AstScannerPort;
use crate::capabilities_ast_extractor::extract_ast_from_string;
use crate::taxonomy::ExtractedFeatureVO;
use std::fs;
use std::path::Path;

pub struct FileSystemAstScannerAdapter;

impl AstScannerPort for FileSystemAstScannerAdapter {
    fn scan_file_features(&self, path: &Path) -> Result<ExtractedFeatureVO, String> {
        // 1. Membaca file dari sistem (Infrastructure duty)
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
      
        // 2. Mendelegasikan parsing ke Capabilities
        extract_ast_from_string(&content).map_err(|e| e.to_string())
    }
}
```

**File:** `infrastructure_fs_modifier.rs`

```rust
use crate::contract::ReferenceModifierPort;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct FileSystemReferenceModifierAdapter;

impl ReferenceModifierPort for FileSystemReferenceModifierAdapter {
    fn propagate_rename(&self, workspace_root: &Path, old_name: &str, new_name: &str) -> Result<(), String> {
        // Logika WalkDir dan fs::write untuk mereplace string di dalam file workspace
        todo!("Implement disk write operations")
    }
}
```

---

## 5. Agent Layer (Orchestration Workflow)

Layer ini menjadi "otak" yang mengoordinasikan seluruh alur kerja. Layer ini tidak tahu bagaimana cara membaca file atau bagaimana model dihitung, ia murni mendelegasikan tugas ke Contract Port & Protocol.

**File:** `agent_autorepair_orchestrator.rs`

```rust
use crate::contract::{AstScannerPort, ReferenceModifierPort, ModelPredictorProtocol};
use burn::tensor::backend::Backend;
use std::path::Path;

pub struct AutorepairOrchestrator<'a, B: Backend> {
    scanner: &'a dyn AstScannerPort,
    modifier: &'a dyn ReferenceModifierPort,
    predictor: &'a dyn ModelPredictorProtocol<B>,
}

impl<'a, B: Backend> AutorepairOrchestrator<'a, B> {
    pub fn new(
        scanner: &'a dyn AstScannerPort,
        modifier: &'a dyn ReferenceModifierPort,
        predictor: &'a dyn ModelPredictorProtocol<B>,
    ) -> Self {
        Self { scanner, modifier, predictor }
    }

    /// Alur utama workflow
    pub fn execute_fix(&self, workspace_root: &Path, target_file: &Path) -> Result<(), String> {
        // 1. Panggil Infra: Baca dan ekstrak fitur file
        let features = self.scanner.scan_file_features(target_file)?;

        // 2. Panggil Capabilities: Prediksi nama yang benar
        let prediction = self.predictor.predict(&features);

        // 3. Rakit nama baru dari prediksi (mocking extraction)
        let new_file_name = "infrastructure_database_adapter.rs"; 
        let old_file_name = target_file.file_name().unwrap().to_str().unwrap();

        // 4. Panggil Infra: Perbaiki referensi di seluruh workspace
        self.modifier.propagate_rename(workspace_root, old_file_name, new_file_name)?;

        Ok(())
    }
}
```

---

## Kesimpulan Refactoring

Dengan menstrukturisasi ulang draf ke dalam standar AES ini, maka:

1. **Tidak Ada Pelanggaran I/O di Logika Bisnis (AES404)**: `capabilities` hanya melakukan prediksi matematika dan parser string murni. Akses file dipindahkan seutuhnya ke `infrastructure_fs_*.rs`.
2. **Dependensi Terbalik dengan Tepat (Contract)**: Orchestrator di layer `agent_` hanya bergantung pada *traits* (Port/Protocol) yang abstrak, bukan pada konkrit.
3. **Data Pasif Terisolasi (AES401)**: Konfigurasi model dan hasil parsing menjadi entitas `_vo` di layer `taxonomy_` yang dapat di-*share* secara aman ke semua layer tanpa *circular dependencies*.