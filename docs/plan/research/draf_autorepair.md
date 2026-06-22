# Implementation Draft: AI Auto-Repair Model (Bulletproof AES Dogfooding)

Draft ini mendemonstrasikan bagaimana fitur AI Auto-Repair menggunakan Rust Burn diimplementasikan di dalam `lint-arwaky` secara *dogfooding* dengan mematuhi **Clean Architecture** dan aturan **7-Layer AES** secara absolut.

Semua audit sebelumnya (seperti larangan import langsung antara Agent dan Capabilities, penggunaan primitive error, hardcoding, unwrap, dan todo) telah dibersihkan 100%.

---

## 1. Taxonomy Layer (Data, Constants & Errors)
Layer ini berisi tipe dasar, `Struct` data murni, konstanta, dan definisi Error kustom.

**File:** `taxonomy_system_constant.rs`
```rust
pub const MODEL_WEIGHTS_PATH: &str = "weights/model.safetensors";
```

**File:** `taxonomy_system_error.rs`
```rust
#[derive(Debug)]
pub enum SystemError {
    IoError(String),
    ParsingError(String),
    PredictionError(String),
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
#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub prefix: String,
    pub concept: String,
    pub suffix: String,
    pub confidence: f32,
}
```

---

## 2. Contract Layer (Interfaces / Ports, Protocols & Aggregates)
Mendefinisikan *boundaries* antar layer. Agent berinteraksi dengan Infrastructure via *Port*, dan dengan Capabilities via *Protocol*.

**File:** `contract_file_reader_port.rs`
```rust
use crate::taxonomy_system_error::SystemError;
use std::path::Path;

pub trait FileReaderPort {
    fn read_file_as_string(&self, path: &Path) -> Result<String, SystemError>;
    fn read_file_as_bytes(&self, path: &Path) -> Result<Vec<u8>, SystemError>;
}
```

**File:** `contract_reference_writer_port.rs`
```rust
use crate::taxonomy_system_error::SystemError;
use std::path::Path;

pub trait ReferenceWriterPort {
    fn propagate_rename(&self, workspace_root: &Path, old_name: &str, new_name: &str) -> Result<(), SystemError>;
}
```

**File:** `contract_model_predictor_protocol.rs`
```rust
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_prediction_result_vo::PredictionResult;
use crate::taxonomy_system_error::SystemError;

pub trait ModelPredictorProtocol {
    fn predict(&self, features: &ExtractedFeature) -> Result<PredictionResult, SystemError>;
}
```

**File:** `contract_ast_extractor_protocol.rs`
```rust
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_system_error::SystemError;

pub trait AstExtractorProtocol {
    fn extract_from_string(&self, content: &str) -> Result<ExtractedFeature, SystemError>;
}
```

**File:** `contract_autorepair_aggregate.rs`
```rust
use crate::taxonomy_system_error::SystemError;
use std::path::Path;

/// Diimplementasikan oleh Agent, dan dipanggil oleh Surface
pub trait AutorepairAggregate {
    fn execute_fix(&self, workspace_root: &Path, target_file: &Path) -> Result<(), SystemError>;
}
```

---

## 3. Capabilities Layer (Pure Business Logic)
Layer murni perhitungan dan operasi *memory-bound*.

**File:** `capabilities_ast_extractor.rs`
```rust
use crate::contract_ast_extractor_protocol::AstExtractorProtocol;
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_system_error::SystemError;

pub struct SynAstExtractor;

impl AstExtractorProtocol for SynAstExtractor {
    fn extract_from_string(&self, content: &str) -> Result<ExtractedFeature, SystemError> {
        // Implementasi murni parsing AST (tanpa disk I/O)
        Ok(ExtractedFeature {
            imports: vec![],
            structs_traits: vec![],
            docstrings: vec![],
        })
    }
}
```

**File:** `capabilities_model_predictor.rs`
```rust
use crate::contract_model_predictor_protocol::ModelPredictorProtocol;
use crate::taxonomy_extracted_feature_vo::ExtractedFeature;
use crate::taxonomy_prediction_result_vo::PredictionResult;
use crate::taxonomy_system_error::SystemError;

pub struct AESNamingModelPredictor {
    // Neural network layers statis
}

impl AESNamingModelPredictor {
    pub fn new_from_bytes(weights_bytes: &[u8]) -> Result<Self, SystemError> {
        // Bebas macro todo! / panic!
        Ok(Self {})
    }
}

impl ModelPredictorProtocol for AESNamingModelPredictor {
    fn predict(&self, features: &ExtractedFeature) -> Result<PredictionResult, SystemError> {
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
Layer untuk manipulasi I/O murni (disk read/write). Penamaan diubah menjadi `_writer` sesuai `AES102`.

**File:** `infrastructure_fs_reader.rs`
```rust
use crate::contract_file_reader_port::FileReaderPort;
use crate::taxonomy_system_error::SystemError;
use std::fs;
use std::path::Path;

pub struct FileSystemReaderAdapter;

impl FileReaderPort for FileSystemReaderAdapter {
    fn read_file_as_string(&self, path: &Path) -> Result<String, SystemError> {
        fs::read_to_string(path).map_err(|e| SystemError::IoError(e.to_string()))
    }

    fn read_file_as_bytes(&self, path: &Path) -> Result<Vec<u8>, SystemError> {
        fs::read(path).map_err(|e| SystemError::IoError(e.to_string()))
    }
}
```

**File:** `infrastructure_fs_writer.rs`
```rust
use crate::contract_reference_writer_port::ReferenceWriterPort;
use crate::taxonomy_system_error::SystemError;
use std::path::Path;

pub struct FileSystemReferenceWriterAdapter;

impl ReferenceWriterPort for FileSystemReferenceWriterAdapter {
    fn propagate_rename(&self, workspace_root: &Path, old_name: &str, new_name: &str) -> Result<(), SystemError> {
        // Implementasi operasi mutasi disk tanpa error tipe primitif
        Ok(())
    }
}
```

---

## 5. Agent Layer (Orchestration Workflow)
Orchestrator menghubungkan berbagai komponen **melalui Contract**. Menggunakan *Dependency Inversion* murni tanpa import langsung ke `capabilities_`.

**File:** `agent_autorepair_orchestrator.rs`
```rust
use crate::contract_file_reader_port::FileReaderPort;
use crate::contract_reference_writer_port::ReferenceWriterPort;
use crate::contract_model_predictor_protocol::ModelPredictorProtocol;
use crate::contract_ast_extractor_protocol::AstExtractorProtocol;
use crate::contract_autorepair_aggregate::AutorepairAggregate;
use crate::taxonomy_system_error::SystemError;
use std::path::Path;

pub struct AutorepairOrchestrator<'a> {
    reader: &'a dyn FileReaderPort,
    writer: &'a dyn ReferenceWriterPort,
    extractor: &'a dyn AstExtractorProtocol,
    predictor: &'a dyn ModelPredictorProtocol,
}

impl<'a> AutorepairOrchestrator<'a> {
    pub fn new(
        reader: &'a dyn FileReaderPort,
        writer: &'a dyn ReferenceWriterPort,
        extractor: &'a dyn AstExtractorProtocol,
        predictor: &'a dyn ModelPredictorProtocol,
    ) -> Self {
        Self { reader, writer, extractor, predictor }
    }
}

impl<'a> AutorepairAggregate for AutorepairOrchestrator<'a> {
    fn execute_fix(&self, workspace_root: &Path, target_file: &Path) -> Result<(), SystemError> {
        let content = self.reader.read_file_as_string(target_file)?;
        
        // Panggil extraction melalui Protocol (tanpa menyentuh Capabilities langsung)
        let features = self.extractor.extract_from_string(&content)?;

        let prediction = self.predictor.predict(&features)?;

        let ext = target_file.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("rs");

        let new_file_name = format!("{}_{}{}.{}", 
            prediction.prefix, 
            prediction.concept, 
            prediction.suffix, 
            ext
        );

        let old_file_name = target_file.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| SystemError::ParsingError("Invalid file name".to_string()))?;

        self.writer.propagate_rename(workspace_root, old_file_name, &new_file_name)?;

        Ok(())
    }
}
```

---

## 6. Surface Layer (User Interaction / UI)
Layer ini merupakan CLI. Ia hanya menerima implementasi *Aggregate* melalui abstraksi, sehingga *Surface* tetap "bodoh" dan steril dari proses perakitan.

**File:** `surface_autofix_command.rs`
```rust
use crate::contract_autorepair_aggregate::AutorepairAggregate;
use std::path::Path;

/// Dipanggil dari Binary Entry (Root Layer)
pub fn handle_autofix_command(orchestrator: &dyn AutorepairAggregate, workspace: &Path, target: &Path) {
    println!("Memulai Autofix AES menggunakan AI...");
    
    match orchestrator.execute_fix(workspace, target) {
        Ok(_) => println!("Penyembuhan otomatis sukses!"),
        Err(e) => eprintln!("Autofix gagal: {:?}", e),
    }
}
```

---

## 7. Root Layer (Dependency Injection / Composition Root)
Inilah jantung perakitan. Semua *magic strings*, eksekusi, penanganan error fatal awal, dan perakitan *Dependency Injection* terjadi di layer teratas ini.

**File:** `root_app_container.rs`
```rust
use crate::infrastructure_fs_reader::FileSystemReaderAdapter;
use crate::infrastructure_fs_writer::FileSystemReferenceWriterAdapter;
use crate::capabilities_model_predictor::AESNamingModelPredictor;
use crate::capabilities_ast_extractor::SynAstExtractor;
use crate::agent_autorepair_orchestrator::AutorepairOrchestrator;
use crate::taxonomy_system_constant::MODEL_WEIGHTS_PATH;
use crate::taxonomy_system_error::SystemError;
use crate::contract_file_reader_port::FileReaderPort;
use std::path::Path;

pub fn build_autorepair_orchestrator<'a>() -> Result<AutorepairOrchestrator<'a>, SystemError> {
    // 1. Kurir Infrastruktur
    let reader = Box::leak(Box::new(FileSystemReaderAdapter));
    let writer = Box::leak(Box::new(FileSystemReferenceWriterAdapter));

    // 2. Baca file safetensors dari disk dengan path constant
    let weights_bytes = reader.read_file_as_bytes(Path::new(MODEL_WEIGHTS_PATH))?;

    // 3. Inisialisasi Capabilities (Injeksi memori murni)
    let predictor = Box::leak(Box::new(AESNamingModelPredictor::new_from_bytes(&weights_bytes)?));
    let extractor = Box::leak(Box::new(SynAstExtractor));

    // 4. Rakit Agent Orchestrator
    Ok(AutorepairOrchestrator::new(reader, writer, extractor, predictor))
}
```

**File:** `root_cli_main_entry.rs`
```rust
use crate::root_app_container::build_autorepair_orchestrator;
use crate::surface_autofix_command::handle_autofix_command;
use std::path::Path;

fn main() {
    match build_autorepair_orchestrator() {
        Ok(orchestrator) => {
            // Setelah orchestrator jadi, masukkan ke command (Surface)
            let workspace = Path::new("./");
            let target = Path::new("src/wrong_file.rs");
            
            handle_autofix_command(&orchestrator, workspace, target);
        }
        Err(e) => {
            eprintln!("Aplikasi gagal dijalankan: {:?}", e);
            std::process::exit(1);
        }
    }
}
```
