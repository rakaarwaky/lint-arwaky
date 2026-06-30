in id pub importsala hadraft ayng saya buat tolong adna berikan kritik dan saranasaja # Implementation Draft: AI Auto-Repair Model (AES Dogfooding Architecture)

This draft demonstrates how the AI Auto-Repair feature using Rust Burn is implemented within `lint-arwaky` following _dogfooding_ principles (strictly adhering to the 7-layer AES rules).

See [ai-autorepair-burn.md](ai-autorepair-burn.md) for the research proposal and [RULES_AES.md](../../rules/RULES_AES.md) for naming rule context.

Code is no longer written monolithically, but is instead split into specific layers: **Taxonomy, Contract, Capabilities, Infrastructure, and Agent**.

---

## 1. Taxonomy Layer (Data, Value Objects)

This layer only contains passive data structures (Value Objects) without I/O execution logic or heavy computation.

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

Defines _boundaries_ between layers so that Capabilities and Infrastructure are not directly coupled (dependency inversion).

**File:** `contract_model_predictor_protocol.rs`

```rust
use crate::taxonomy::{AESNamingModelConfigVO, ExtractedFeatureVO, ModelOutputVO};
use burn::tensor::backend::Backend;

/// Implemented by the Capabilities layer
pub trait ModelPredictorProtocol<B: Backend> {
    fn predict(&self, features: &ExtractedFeatureVO) -> ModelOutputVO<B>;
}
```

**File:** `contract_ast_scanner_port.rs`

```rust
use crate::taxonomy::ExtractedFeatureVO;
use std::path::Path;

/// Implemented by the Infrastructure layer
pub trait AstScannerPort {
    fn scan_file_features(&self, path: &Path) -> Result<ExtractedFeatureVO, String>;
}
```

**File:** `contract_reference_modifier_port.rs`

```rust
use std::path::Path;

/// Implemented by the Infrastructure layer
pub trait ReferenceModifierPort {
    fn propagate_rename(&self, workspace_root: &Path, old_name: &str, new_name: &str) -> Result<(), String>;
}
```

---

## 3. Capabilities Layer (Pure Business Logic)

This layer contains prediction logic (Burn Model) and _parsing_ (Syn AST). There should be no _file system_ access here.

**File:** `capabilities_model_predictor.rs`

```rust
use burn::module::Module;
use burn::tensor::{backend::Backend, Device, Int, Tensor};
// ... other imports (Embedding, TransformerEncoder, Linear) ...
use crate::taxonomy::{AESNamingModelConfigVO, ExtractedFeatureVO, ModelOutputVO};
use crate::contract::ModelPredictorProtocol;

#[derive(Module, Debug)]
pub struct AESNamingModelPredictor<B: Backend> {
    // Model layer definitions...
}

impl<B: Backend> ModelPredictorProtocol<B> for AESNamingModelPredictor<B> {
    fn predict(&self, features: &ExtractedFeatureVO) -> ModelOutputVO<B> {
        // 1. Convert ExtractedFeatureVO to tensor
        // 2. Perform forward pass on self.encoder, etc...
        // 3. Return ModelOutputVO
        todo!("Implement pure inference forward pass")
    }
}
```

**File:** `capabilities_ast_extractor.rs`

```rust
use crate::taxonomy::ExtractedFeatureVO;
use syn::{parse_file, File};

/// Pure function without disk I/O
pub fn extract_ast_from_string(content: &str) -> Result<ExtractedFeatureVO, syn::Error> {
    let syntax: File = parse_file(content)?;
    // Pure AST extraction into VO...
    Ok(ExtractedFeatureVO {
        imports: vec![],
        structs_traits: vec![],
        docstrings: vec![],
    })
}
```

---

## 4. Infrastructure Layer (I/O & External Systems)

This layer implements the Ports from the Contract layer. It handles I/O directly (Disk/OS access).

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
        // 1. Read file from the system (Infrastructure duty)
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;

        // 2. Delegate parsing to Capabilities
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
        // WalkDir and fs::write logic to replace strings within workspace files
        todo!("Implement disk write operations")
    }
}
```

---

## 5. Agent Layer (Orchestration Workflow)

This layer serves as the "brain" that coordinates the entire workflow. It does not know how to read files or how the model is computed — it purely delegates tasks to Contract Ports & Protocols.

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

    /// Main workflow flow
    pub fn execute_fix(&self, workspace_root: &Path, target_file: &Path) -> Result<(), String> {
        // 1. Call Infra: Read and extract file features
        let features = self.scanner.scan_file_features(target_file)?;

        // 2. Call Capabilities: Predict the correct name
        let prediction = self.predictor.predict(&features);

        // 3. Assemble new name from prediction (mocking extraction)
        let new_file_name = "infrastructure_database_adapter.rs";
        let old_file_name = target_file.file_name().unwrap().to_str().unwrap();

        // 4. Call Infra: Fix references throughout the workspace
        self.modifier.propagate_rename(workspace_root, old_file_name, new_file_name)?;

        Ok(())
    }
}
```

---

## Refactoring Conclusion

By restructuring the draft to comply with AES standards, the following are achieved:

1. **No I/O Violation in Business Logic (AES404)**: `capabilities` only performs mathematical prediction and pure string parsing. File access has been entirely moved to `infrastructure_fs_*.rs`.
2. **Correct Dependency Inversion (Contract)**: The orchestrator in the `agent_` layer only depends on abstract _traits_ (Port/Protocol), not on concrete types.
3. **Isolated Passive Data (AES401)**: Model configuration and parsing results become `_vo` entities in the `taxonomy_` layer that can be safely shared across all layers without _circular dependencies_.
