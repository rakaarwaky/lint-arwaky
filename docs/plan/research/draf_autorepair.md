# Implementation Draft: AI Auto-Repair Model (Bulletproof AES Dogfooding)

Draft ini mendemonstrasikan bagaimana fitur AI Auto-Repair menggunakan Rust Burn diimplementasikan di dalam `lint-arwaky` secara *dogfooding* dengan mematuhi **Clean Architecture** dan aturan **7-Layer AES** secara absolut.

Semua kebocoran arsitektural (seperti Tensors di Taxonomy, I/O di Capabilities, dan Hardcoding di Agent) telah dibersihkan.

---

## 1. Taxonomy Layer (Data & Value Objects)
Layer ini hanya berisi tipe dasar, `String`, dan data murni. **Tidak ada** dependensi ke `burn` backend di sini. Nama `struct` bersih dari *suffix* redundan `VO`.

**File:** `taxonomy_model_config_vo.rs`
```rust
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AESNamingModelConfig {
    pub vocab_size: usize,
    pub d_model: usize,
    // ...
}
```

**File:** `taxonomy_extracted_feature_vo.rs`
```rust
#[derive(Debug, Clone)]
pub struct ExtractedFeature {
    pub imports: Vec<String>,
    pub structs_traits: Vec<String>,
    pub docstrings: Vec<String>,
}
```

**File:** `taxonomy_prediction_result_vo.rs`
```rust
/// Hasil murni dari prediksi model AI, bebas dari representasi Tensor.
#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub prefix: String,
    pub concept: String,
    pub suffix: String,
    pub confidence: f32,
}
```

---

## 2. Contract Layer (Interfaces / Ports & Protocols)
Mendefinisikan *boundaries* antar layer agar Capabilities dan Infrastructure tidak saling terikat secara langsung.

**File:** `contract_model_predictor_protocol.rs`
```rust
use crate::taxonomy::{ExtractedFeature, PredictionResult};

/// Diimplementasikan oleh layer Capabilities
pub trait ModelPredictorProtocol {
    fn predict(&self, features: &ExtractedFeature) -> Result<PredictionResult, String>;
}
```

**File:** `contract_file_reader_port.rs`
```rust
use std::path::Path;

/// Diimplementasikan oleh layer Infrastructure
pub trait FileReaderPort {
    fn read_file_as_string(&self, path: &Path) -> Result<String, String>;
    fn read_file_as_bytes(&self, path: &Path) -> Result<Vec<u8>, String>;
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

## 3. Capabilities Layer (Pure Business Logic & AI Inference)
Layer ini menginisialisasi model AI dari byte array murni dan melakukan operasi komputasi berat. Tidak boleh ada `fs::read` atau penulisan disk di sini.

**File:** `capabilities_model_predictor.rs`
```rust
use burn::module::Module;
use burn::tensor::backend::Backend;
use crate::taxonomy::{ExtractedFeature, PredictionResult};
use crate::contract::ModelPredictorProtocol;

pub struct AESNamingModelPredictor<B: Backend> {
    // Neural network layers
}

impl<B: Backend> AESNamingModelPredictor<B> {
    /// Capabilities memuat bobot dari byte array memori yang dikirim oleh Root, bukan dari disk.
    pub fn new_from_bytes(weights_bytes: &[u8]) -> Self {
        // Logika inisialisasi safetensors ke dalam layer neural network
        todo!()
    }
}

impl<B: Backend> ModelPredictorProtocol for AESNamingModelPredictor<B> {
    fn predict(&self, features: &ExtractedFeature) -> Result<PredictionResult, String> {
        // 1. Eksekusi model AI (Forward Pass)
        // 2. Dapatkan Tensor Logits
        // 3. (PENTING) Decode Tensor menjadi String murni agar aman dikirim ke Agent
        
        Ok(PredictionResult {
            prefix: "infrastructure".to_string(),
            concept: "database".to_string(),
            suffix: "_adapter".to_string(),
            confidence: 0.95,
        })
    }
}
```

---

## 4. Infrastructure Layer (I/O & External Systems)
Layer ini menangani akses Disk/OS. Ia murni "kurir" data bodoh.

**File:** `infrastructure_fs_reader.rs`
```rust
use crate::contract::FileReaderPort;
use std::fs;
use std::path::Path;

pub struct FileSystemReaderAdapter;

impl FileReaderPort for FileSystemReaderAdapter {
    fn read_file_as_string(&self, path: &Path) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| e.to_string())
    }

    fn read_file_as_bytes(&self, path: &Path) -> Result<Vec<u8>, String> {
        fs::read(path).map_err(|e| e.to_string())
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
        // Operasi write & manipulasi file sistem
        Ok(())
    }
}
```

---

## 5. Agent Layer (Orchestration Workflow)
Bertindak sebagai otak perangkai alur. Agent memanggil infrastruktur dan AI, lalu merakit nama secara dinamis tanpa melakukan *hardcoding*.

**File:** `agent_autorepair_orchestrator.rs`
```rust
use crate::contract::{FileReaderPort, ReferenceModifierPort, ModelPredictorProtocol};
// PENTING: Agent mengimpor extractor murni dari Capabilities
use crate::capabilities_ast_extractor::extract_ast_from_string;
use std::path::Path;

pub struct AutorepairOrchestrator<'a> {
    reader: &'a dyn FileReaderPort,
    modifier: &'a dyn ReferenceModifierPort,
    predictor: &'a dyn ModelPredictorProtocol,
}

impl<'a> AutorepairOrchestrator<'a> {
    pub fn new(
        reader: &'a dyn FileReaderPort,
        modifier: &'a dyn ReferenceModifierPort,
        predictor: &'a dyn ModelPredictorProtocol,
    ) -> Self {
        Self { reader, modifier, predictor }
    }

    pub fn execute_fix(&self, workspace_root: &Path, target_file: &Path) -> Result<(), String> {
        // 1. Infra: Baca file sebagai string
        let content = self.reader.read_file_as_string(target_file)?;

        // 2. Capabilities: Parse AST
        let features = extract_ast_from_string(&content)?;

        // 3. Capabilities: Prediksi model AI (Mengembalikan String aman)
        let prediction = self.predictor.predict(&features)?;

        // 4. Agent: Merakit nama secara dinamis (Tidak ada Hardcode)
        let ext = target_file.extension().unwrap_or_default().to_str().unwrap_or("rs");
        let new_file_name = format!("{}_{}{}.{}", 
            prediction.prefix, 
            prediction.concept, 
            prediction.suffix, 
            ext
        );

        // 5. Infra: Modifikasi referensi seluruh file
        let old_file_name = target_file.file_name().unwrap().to_str().unwrap();
        self.modifier.propagate_rename(workspace_root, old_file_name, &new_file_name)?;

        Ok(())
    }
}
```

---

## 6. Root Layer (Dependency Injection / Composition Root)
Tempat pabrik perakitan. Membaca model dari infrastruktur lalu merangkainya (*inject*) ke *Capabilities* agar hukum AES tidak dilanggar.

**File:** `root_app_container.rs`
```rust
use crate::infrastructure_fs_reader::FileSystemReaderAdapter;
use crate::infrastructure_fs_modifier::FileSystemReferenceModifierAdapter;
use crate::capabilities_model_predictor::AESNamingModelPredictor;
use crate::agent_autorepair_orchestrator::AutorepairOrchestrator;
// Gunakan CPU backend (NdArray) secara dinamis
use burn::backend::NdArray; 
use std::path::Path;
use crate::contract::FileReaderPort;

pub fn build_autorepair_orchestrator<'a>() -> AutorepairOrchestrator<'a> {
    // 1. Inisialisasi Infrastructure (Kurir disk)
    let reader = Box::leak(Box::new(FileSystemReaderAdapter));
    let modifier = Box::leak(Box::new(FileSystemReferenceModifierAdapter));

    // 2. Root meminta kurir infra membaca `.safetensors` ke memori (I/O selesai di sini)
    let weights_bytes = reader.read_file_as_bytes(Path::new("weights/model.safetensors")).unwrap();

    // 3. Inisialisasi Capabilities (Injeksi memori murni, tanpa I/O)
    let predictor = Box::leak(Box::new(AESNamingModelPredictor::<NdArray>::new_from_bytes(&weights_bytes)));

    // 4. Rakit Agent Orchestrator
    AutorepairOrchestrator::new(reader, modifier, predictor)
}
```

---

## 7. Surface Layer (Entry Point)
Layer yang menjadi titik sentuh pengguna akhir (CLI), memanggil *Root* dan meneruskan perintah ke *Agent*.

**File:** `surface_autofix_command.rs`
```rust
use crate::root_app_container::build_autorepair_orchestrator;
use std::path::PathBuf;

pub fn handle_autofix_command(workspace_path: PathBuf, target_file: PathBuf) {
    println!("Memulai Autofix AES menggunakan AI...");
    
    // Tarik orchestrator yang sudah siap pakai
    let orchestrator = build_autorepair_orchestrator();
    
    // Eksekusi goal utama
    match orchestrator.execute_fix(&workspace_path, &target_file) {
        Ok(_) => println!("Penyembuhan otomatis sukses!"),
        Err(e) => eprintln!("Autofix gagal: {}", e),
    }
}
```
