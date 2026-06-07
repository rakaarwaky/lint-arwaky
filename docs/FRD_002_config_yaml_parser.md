# FRD — Config YAML Parser

> **PRD Reference**: [FR-002](PRD.md) — Config system: multi-config support, YAML reader, language detection, config-driven rules
> **Dependency**: FR-001 (6-layer AES architecture)
> **Status**: ✅ COMPLETE — Multi-config YAML reader, language detector, config orchestrator implemented

## 1. Problem Statement

Config loading dulu dilakukan secara hardcode di banyak tempat:

| Location | Problem |
|----------|---------|
| `capabilities/architecture_lint_handler.rs` | Cari `lint_arwaky.config.rust.yaml` dengan hardcode — naik parent directory manual |
| `taxonomy/architecture_config_vo.rs` | `include_str!("../../lint_arwaky.config.rust.yaml")` — cuma Rust config yang di-embed |
| `infrastructure/javascript_linter_adapter.rs` | Malah nyari `lint_arwaky.config.python.yaml` untuk JS — nama file salah |
| `infrastructure/config_discovery_provider.rs` | Config buat JS dan Rust tidak terdaftar |

Juga, config loading ada di layer yang salah: `ArchitectureLintHandler` (capabilities) baca file langsung. Seharusnya infrastruktur yang handle I/O.

## 2. Konsep Dasar

Setiap project punya bahasa pemrograman dominan (Rust, Python, JavaScript). Masing-masing punya file config sendiri:

```
Project Rust:     ./lint_arwaky.config.rust.yaml
Project Python:   ./lint_arwaky.config.python.yaml
Project JS/TS:    ./lint_arwaky.config.javascript.yaml
```

Ketika user jalanin `lint-arwaky-cli scan /some/project`, sistem harus:
1. **Deteksi bahasa** project (dari struktur direktori)
2. **Baca config** yang sesuai dengan bahasa tersebut
3. **Parse YAML** → `ArchitectureConfig`
4. **Gunakan config** untuk semua rule checking

## 3. Mekanisme Kerja — Step by Step

### 3.1 Language Detection (`LanguageDetectorProvider`)

```
Input: /some/project

Step 1: Cek source directory (confidence 100)
  ├── /some/project/src-rust/      → Rust ✅ 
  ├── /some/project/src-python/    → Python
  └── /some/project/src-javascript/ → JavaScript

Step 2: Cek file manifest (confidence 90)
  ├── Cargo.toml → Rust
  ├── pyproject.toml / setup.py / requirements.txt → Python
  └── package.json → JavaScript

Step 3: Cek src/ + ekstensi file (confidence 70)
  ├── src/ + *.rs → Rust
  ├── src/ + *.py → Python
  └── src/ + *.js/.ts → JavaScript

Step 4: Fallback — scan semua file di project (confidence 50)
  └── Hitung ekstensi terbanyak → tebak bahasa

Output: LanguageSource { language: "rust", confidence: 100, source: "directory" }
```

### 3.2 Config File Reading (`ConfigYamlReader`)

```
Input: /some/project, language = "rust"

Step 1: Tentukan nama file config
  └── LanguageConfig:
        rust       → "lint_arwaky.config.rust.yaml"
        python     → "lint_arwaky.config.python.yaml"
        javascript → "lint_arwaky.config.javascript.yaml"

Step 2: Cari file
  ├── /some/project/lint_arwaky.config.rust.yaml  ← Cari di project root dulu
  ├── Kalau nggak ada → naik ke parent directory
  └── Kalau nggak ada sama sekali → fallback ke embedded default

Step 3: Baca file
  └── fs::read_to_string() → raw YAML string

Output: ConfigSource { language: "rust", path: "/some/project/...", raw_content: "layers:\n  ..." }
```

### 3.3 YAML Parsing (`ConfigParserProvider`)

```
Input: Raw YAML string

Step 1: serde_yaml::from_str(raw) → serde_json::Value
Step 2: Extract key "architecture"
Step 3: Flatten rules: global + internal + external
Step 4: Deserialize → ArchitectureConfig

ArchitectureConfig {
    layers: Vec<LayerDefinition>,      // Definisis setiap layer
    rules: Vec<ArchitectureRule>,       // Semua AES rules
    naming: NamingConfig,               // Aturan naming
    scoring: ScoringConfig,             // Threshold score
    bypass: BypassConfig,               // Bypass patterns
}
```

### 3.4 Config Orchestration (`ConfigOrchestrationProcessor`)

```
User: lint-arwaky-cli scan /some/project
    │
    ▼
Surface (cli_check_command)
    │
    ▼
ConfigOrchestrationProcessor.load_project_config("/some/project")
    │
    ├─► ILanguageDetectorPort.detect_language("/some/project")
    │     └─→ LanguageSource { "rust", 100 }
    │
    ├─► IConfigReaderPort.read_config("/some/project", "rust")
    │     └─→ ConfigSource (raw YAML)
    │
    ├─► IConfigParserPort.parse_raw_yaml(raw_yaml)
    │     └─→ ArchitectureConfig
    │
    └─→ ConfigResult { config: ArchitectureConfig, source: ConfigSource, warnings: [] }

Kalau nggak ada config file:
    └─→ default_aes_config()  ← embedded include_str!("../../lint_arwaky.config.rust.yaml")
         └─→ ConfigResult { warnings: ["No config file found, using built-in defaults"] }
```

## 4. Struktur Config YAML

```yaml
# lint_arwaky.config.rust.yaml
architecture:
  layers:
    - name: taxonomy
      path: src-rust/taxonomy/
      suffixes: [vo, entity, event, error, constant]
      allowed_imports: [taxonomy]
      forbidden_imports: [agent, infrastructure, surfaces, contract, capabilities]
      mandatory_imports: []
      recursive: true
      min_lines: 10
      max_lines: 500
      no_primitives: true
      
    - name: contract
      path: src-rust/contract/
      suffixes: [port, protocol, aggregate]
      allowed_imports: [taxonomy, contract]
      forbidden_imports: [agent, infrastructure, surfaces, capabilities]
      mandatory_imports: [taxonomy]
      recursive: true
      min_lines: 5
      max_lines: 300
      no_primitives: true

    # ... layer lainnya dengan struktur yang sama

  rules:
    - code: AES001
      name: import-layer-violation
      severity: HIGH
      enabled: true
    - code: AES002
      name: mandatory-import-missing
      severity: HIGH
      enabled: true
    # ... semua 31 rules

  naming:
    pattern: "^[a-z]+_[a-z]+_[a-z]+\\.rs$"
    exceptions: ["main.rs", "lib.rs", "mod.rs"]

  scoring:
    initial_score: 100.0
    penalties:
      LOW: 1.0
      MEDIUM: 2.0
      HIGH: 3.0
      CRITICAL: 5.0
    auto_fail_on_critical: true

  bypass:
    patterns:
      - "#[allow\\("]
      - "unwrap\\(\\)"
      - "panic!"
      - "noqa"
      - "type: ignore"
      - "# type:"
```

## 5. File-file Kunci

### Taxonomy (2 new VOs)
| File | Struct | Field |
|------|--------|-------|
| `taxonomy/project_language_vo.rs` | `ProjectLanguage` | `language: String` |
| | `LanguageSource` | `language`, `confidence: u8`, `source: String` |
| `taxonomy/config_source_vo.rs` | `ConfigSource` | `language`, `path`, `raw_content` |
| | `ConfigResult` | `config`, `source`, `warnings` |

### Contract (3 new ports)
| File | Trait | Method |
|------|-------|--------|
| `contract/config_reader_port.rs` | `IConfigReaderPort` | `read_config(project_root, language) → ConfigSource`, `list_config_files(project_root) → Vec<(Language, Path)>` |
| `contract/language_detector_port.rs` | `ILanguageDetectorPort` | `detect_language(project_root) → LanguageSource` |
| `contract/config_orchestration_protocol.rs` | `IConfigOrchestrationProtocol` | `load_project_config(project_root) → ConfigResult`, `load_config_for_language(project_root, language) → ConfigResult` |

### Infrastructure (2 new, 1 updated)
| File | Class | Implements |
|------|-------|------------|
| `infrastructure/config_yaml_reader.rs` | `ConfigYamlReader` | `IConfigReaderPort` |
| `infrastructure/language_detector_provider.rs` | `LanguageDetectorProvider` | `ILanguageDetectorPort` |
| `infrastructure/config_parser_provider.rs` | `ConfigParserProvider` | (updated) tambah `parse_raw_yaml(content)` |

### Capabilities (1 new)
| File | Class | Implements |
|------|-------|------------|
| `capabilities/config_orchestration_processor.rs` | `ConfigOrchestrationProcessor` | `IConfigOrchestrationProtocol` |

### Dihapus
| File | Alasan |
|------|--------|
| `infrastructure/config_discovery_provider.rs` | Diganti oleh `ConfigYamlReader` + `LanguageDetectorProvider` |

## 6. Alur Data Lengkap

```
YAML File (disk)
    │
    ▼
ConfigYamlReader.read_config()      ← Infrastructure
    │  fs::read_to_string()
    ▼
Raw YAML String (ConfigSource)
    │
    ▼
ConfigParserProvider.parse_raw_yaml()  ← Infrastructure
    │  serde_yaml::from_str()
    ▼
ArchitectureConfig (struct)
    │
    ▼
ConfigOrchestrationProcessor  ← Capabilities
    │  Gabung source + config + warnings
    ▼
ConfigResult
    │
    ▼
DI Container (agent) → Surface (CLI)
    │
    ▼
LintCheckingCoordinator.run_all_checks()
    │  Pakai ArchitectureConfig.rules untuk setiap check
    ▼
Setiap checker baca dari config:
    ├── naming_checker → config.naming.pattern
    ├── import_checker → config.layers[].allowed_imports / forbidden_imports
    ├── metric_checker → config.layers[].min_lines / max_lines
    ├── bypass_checker → config.bypass.patterns
    └── scoring → config.scoring.penalties
```

## 7. AES Compliance

| Rule | Compliance |
|------|------------|
| AES001 | `ConfigOrchestrationProcessor` (capabilities) panggil `IConfigReaderPort` (contract) — bukan langsung import infrastructure |
| AES002 | Semua class implements contract trait wajib |
| AES003 | Semua filename 3-word: `config_yaml_reader`, `language_detector_provider`, `config_orchestration_processor` |
| AES011 | Suffix: `_reader`, `_provider`, `_detector`, `_processor` — semua allowed per layer |
| AES027 | Setiap logic file implements trait (`IConfigReaderPort`, `ILanguageDetectorPort`, `IConfigOrchestrationProtocol`) |

## 8. Acceptance Criteria

| # | Kriteria | Mekanisme | Status |
|---|----------|-----------|--------|
| AC001 | Config loading pindah dari capability ke infrastructure | `ArchitectureLintHandler` dulu: `fs::read()` langsung + `serde_yaml::from_str()`. Sekarang: delegasi ke `ConfigYamlReader` + `ConfigParserProvider` | ✅ |
| AC002 | Baca 3 config berdasarkan bahasa | `IConfigReaderPort` pilih filename dari `ProjectLanguage` | ✅ |
| AC003 | Deteksi bahasa dari struktur direktori | `LanguageDetectorProvider` cek `src-{lang}/` → Cargo.toml → pyproject.toml → package.json → ekstensi | ✅ |
| AC004 | Fallback ke default jika config tidak ada | `default_aes_config()` dipanggil dengan warning | ✅ |
| AC005 | `include_str!` default sebagai built-in fallback | `taxonomy/architecture_config_vo.rs` masih nahan embedded yaml | ✅ |
| AC006 | `config_discovery_provider.rs` dihapus | File sudah tidak ada | ✅ |
| AC007 | Blocking I/O pakai `spawn_blocking` | `tokio::task::spawn_blocking(|| fs::read_to_string())` | ✅ |
| AC008 | Runtime reuse via `Handle::try_current()` | Tidak bikin Runtime baru setiap panggilan | ✅ |
| AC009 | `has_src` guard tetap jalan | Sebelum jalanin AES lint, cek dulu `src-rust/` ada | ✅ |
| AC010 | Barrel exports di semua layer diperbarui | Setiap `mod.rs` re-export file baru | ✅ |
| AC011 | `cargo check --bin lint-arwaky-cli` lulus | Compile tanpa error | ✅ |
| AC012 | `cargo test` — 46 passed, 0 failed | Unit tests lulus | ✅ |
