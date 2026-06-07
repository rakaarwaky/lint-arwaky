# FRD — Source Code Parsing (Rust + Python + JS/TS)

> **PRD Reference**: [FR-003](PRD.md) — Source code parsing for Rust, Python, JavaScript/TypeScript
> **Dependency**: FR-001 (6-layer AES architecture)
> **Status**: ⚠️ **BASIC / NOT PRODUCTION-READY** — Regex-based line scanners, NOT true AST parsers. Missing `syn` (Rust), `ast` (Python), `swc`/`tree-sitter` (JS/TS) integration.
> **Self-lint**: `lint-arwaky-cli check .` — scanners work for basic patterns but miss complex cases

## 1. Problem Statement

Sebelum ada source parser terpusat:

| Issue | Description |
|-------|-------------|
| **Parsing tersebar** | Setiap linter adapter punya logic parsing sendiri — duplikasi besar-besaran |
| **Tidak ada interface seragam** | Caller harus tahu parser apa untuk bahasa apa |
| **Regex bertebaran** | Pattern import detection ditulis ulang di banyak file |
| **Tidak ada barrel detection** | Tiap tool punya definisi barrel sendiri |
| **Tidak ada entry point detection** | `main.rs`, `__main__`, `index.ts` tidak terdeteksi secara seragam |

## 2. Konsep Dasar

Buat satu interface `ISourceParserPort` (17 methods) yang diimplementasi oleh 3 parser language-specific, dan 1 composite orchestrator yang routing berdasarkan ekstensi file.

```
Caller (capability checker)
    │
    ▼
ISourceParserPort  (contract layer — interface)
    │
    ▼
SourceParserOrchestrator  (infrastructure — composite)
    │
    ├──► .rs   → ASTRustParserAdapter   (regex line scanner)
    ├──► .py   → ASTPythonParserAdapter (regex line scanner)
    └──► .js/.ts/.jsx/.tsx → ASTJSParserAdapter (regex line scanner)
```

### ⚠️ BATASAN PENTING

Semua parser adalah **regex-based line-by-line scanners**, BUKAN true AST parsers:

| Kemampuan | Regex Scanner | True AST Parser (syn/swc/ast) |
|-----------|--------------|------------------------------|
| Multi-line statements | ❌ Gagal | ✅ |
| Nested generics `Vec<Result<>>` | ❌ Gagal | ✅ |
| Macros `vec![]` | ❌ Gagal | ✅ |
| Attributes `#[derive()]` | ❌ Gagal | ✅ |
| String literal filtering | ❌ False positive | ✅ |
| Scope nesting | ❌ Brace count saja | ✅ |
| Arrow functions | ❌ Tidak terdeteksi | ✅ |
| JSX/TSX | ❌ Rusak | ✅ |

## 3. Mekanisme Kerja — Step by Step

### 3.1 Routing (SourceParserOrchestrator)

```
Input: FilePath("/project/src/auth/user_vo.py")

select_parser(path):
    ├── .rs   → return &self.rust_parser    (ASTRustParserAdapter)
    ├── .ts / .tsx / .js / .jsx → return &self.js_parser (ASTJSParserAdapter)
    └── .py / lainnya → return &self.python_parser (ASTPythonParserAdapter)

Kemudian semua 17 method di-delegate ke parser yang dipilih:
    orchestrator.extract_imports(path)
        └── self.select_parser(path).extract_imports(path)
```

### 3.2 Rust Scanner — Cara Kerja (ASTRustParserAdapter)

```
File dibaca baris per baris. Setiap baris dicocokkan dengan regex:

Baris: "use crate::taxonomy::FilePath;"
    ├── USE_REGEX: ^(?:pub\s+)?use\s+([^;]+);
    │     → Match: "crate::taxonomy::FilePath"
    │     → Strip: "crate::" → "taxonomy::FilePath"
    │     → Simpan: ImportInfo { source: "crate", path: "taxonomy::FilePath" }
    │

Baris: "pub struct ArchitectureConfig {"
    ├── STRUCT_REGEX: ^(?:pub\s+)?struct\s+(\w+)
    │     → Match: "ArchitectureConfig"
    │     → Simpan: symbol "ArchitectureConfig" sebagai struct
    │

Baris: "pub fn detect_layer(path: &FilePath) -> LayerNameVO {"
    ├── FN_REGEX: ^(?:pub\s+)?(?:async\s+)?fn\s+(\w+)
    │     → Match: "detect_layer"
    │     → Simpan: function "detect_layer"
    │

Baris: "if layer == LAYER_TAXONOMY {"
    ├── CF_REGEX: \b(if|for|while|match|loop)\b
    │     → Match: "if"
    │     → Increment control_flow_count
    │

Baris: "mod.rs" atau "lib.rs"
    └── is_barrel_file() → true
```

**Yang TIDAK bisa ditangani**:
```rust
// Multi-line use — ❌ GAGAL
use crate::taxonomy::{
    FilePath, DirectoryPath, LayerNameVO,
};

// Nested generics — ❌ GAGAL (regex salah urus kurung <>
fn parse<T: Clone + Debug>(input: Result<Vec<String>, Error>) -> T;

// Macro — ❌ GAGAL dicek sebagai "fn main"
vec![1, 2, 3].iter().for_each(|x| println!("{}", x));

// String literal mengandung kode — ❌ FALSE POSITIVE
let doc = "fn main() { println!(\"hello\"); }";  // Ini kedeteksi sebagai fungsi
```

### 3.3 Python Scanner — Cara Kerja (ASTPythonParserAdapter)

```
Baris: "from taxonomy import FilePath"
    ├── FROM_IMPORT_REGEX: ^from\s+(\w+(?:\.\w+)*)\s+import\s+(.+)$
    │     → Match: source="taxonomy", symbols="FilePath"
    │     → Simpan: ImportInfo
    │

Baris: "class LayerDetector:"
    ├── CLASS_REGEX: ^class\s+(\w+)\s*(?:\(([^)]*)\))?:
    │     → Match: "LayerDetector"
    │     → Simpan: class definition
    │

Baris: "def detect_layer(self, path: str) -> LayerVO:"
    ├── DEF_REGEX: ^def\s+(\w+)\s*\(
    │     → Match: "detect_layer"
    │

Baris: "    if path.startswith('src-rust'):"
    │     (indentation = 4 spasi → masuk scope detect_layer)
    │
    ├── CF_REGEX: \b(if|for|while|try|except|with|async for)\b
    │     → Match: "if"
    │

Baris: "    return LayerVO(name='taxonomy')"
    └── TYPE_ANNOT_RE: :\s*(int|str|float|bool|list|dict|tuple|set|bytes|None)\b
          → Match: ": str" → VIOLATION primitive 'str' (AES006)
```

**Yang TIDAK bisa ditangani**:
```python
# Multi-line import — ❌ GAGAL
from taxonomy import (
    FilePath,
    LayerNameVO,
)

# Decorator — ❌ GAGAL sebagai fungsi biasa
@dataclass
class Config:
    pass

# Match/case Python 3.10 — ❌ TIDAK TERDUKUNG
match value:
    case 1: ...
```

### 3.4 JS/TS Scanner — Cara Kerja (ASTJSParserAdapter)

```
Baris: "import { LayerDetector } from './detector';"
    ├── IMPORT_REGEX: ^import\s+(.+?)\s+from\s+'([^']+)'
    │     → Match: symbols="{ LayerDetector }", source="./detector"
    │

Baris: 'import { FilePath } from "./types"'
    ├── IMPORT_DOUBLE_REGEX: ^import\s+(.+?)\s+from\s+"([^"]+)"
    │     → Match (petik ganda)
    │

Baris: "const fs = require('fs');"
    ├── REQUIRE_REGEX: ^(?:const|let|var)\s+(\w+)\s*=\s*require\((?:'([^']+)'|"([^"]+)")\)
    │     → Match: var="fs", source="fs"
    │

Baris: "class LayerDetector extends BaseDetector {"
    ├── CLASS_REGEX: ^class\s+(\w+)(?:\s+extends\s+(\w+))?
    │     → Match: "LayerDetector", extends="BaseDetector"
    │     → Simpan di class_bases: { "LayerDetector": ["BaseDetector"] }
    │

Baris: "function detectLayer(path: string): LayerVO {"
    ├── FN_REGEX: ^(?:async\s+)?function\s+(\w+)
    │     → Match: "detectLayer"
    │
```

**Yang TIDAK bisa ditangani**:
```typescript
// Destructured import — ❌ GAGAL
import { Foo as Bar, Baz, Qux } from './module';

// Generic type — ❌ GAGAL (angle bracket disangka comparison)
function identity<T extends SomeType>(arg: T): T;

// Arrow function — ❌ TIDAK TERDETEKSI sebagai fungsi
const handler = (req: Request, res: Response) => { ... };

// Template literal dengan kode — ❌ FALSE POSITIVE
const code = `function hello() { return 42; }`;

// Dynamic import — ❌ TIDAK TERDETEKSI
const module = await import('./dynamic');
```

## 4. ISourceParserPort — 17 Methods

| # | Method | Return | Logic |
|---|--------|--------|-------|
| 1 | `extract_imports` | `Result<ImportInfoList>` | Scan baris dengan regex import/use/require |
| 2 | `get_raw_symbols` | `Result<ResponseData>` | Kumpulkan semua class/fn/struct/enum/trait |
| 3 | `get_class_attributes` | `ResponseData` | Ambil field dari struct/class |
| 4 | `has_all_export` | `SuccessStatus` | Cek `__all__` / `pub use *` |
| 5 | `find_primitive_violations` | `PrimitiveViolationList` | Scan type annotations → cari String/i32/int/str |
| 6 | `find_unused_imports` | `ImportInfoList` | Cek apakah setiap import dipakai sebagai symbol |
| 7 | `get_class_definitions` | `Result<MetadataVO>` | Ekstrak semua class definition |
| 8 | `get_function_definitions` | `MetadataVO` | Ekstrak semua function definition |
| 9 | `is_symbol_exported` | `SuccessStatus` | Cek apakah symbol ada di `pub` / `export` |
| 10 | `get_class_methods` | `MetadataVO` | Ambil methods dari tiap class |
| 11 | `get_class_bases_map` | `MetadataVO` | Inheritance parent: `class A extends B` → A: [B] |
| 12 | `get_assignment_targets` | `MetadataVO` | Variable assignments: `let x = ...` |
| 13 | `get_control_flow_count` | `Count` | Hitung keyword if/for/while/match/loop |
| 14 | `is_barrel_file` | `BooleanVO` | Cek nama file: mod.rs, __init__.py, index.ts |
| 15 | `get_stem` | `SymbolName` | Nama file tanpa ekstensi |
| 16 | `is_entry_point` | `BooleanVO` | Cek fn main, __main__, cli_main_entry |
| 17 | `get_supported_extensions` | `PatternList` | [".rs", ".py", ".ts", ".tsx", ".js", ".jsx"] |

## 5. File-file Kunci

| File | Baris | Isi |
|------|-------|-----|
| `contract/source_parser_port.rs` | 28 | `ISourceParserPort` trait — 17 method signature |
| `infrastructure/ast_rust_scanner.rs` | 518 | `ASTRustParserAdapter` — regex Rust scanner |
| `infrastructure/ast_py_scanner.rs` | 569 | `ASTPythonParserAdapter` — regex Python scanner |
| `infrastructure/ast_js_scanner.rs` | 603 | `ASTJSParserAdapter` — regex JS/TS scanner |
| `infrastructure/source_parser_adapter.rs` | 143 | `SourceParserOrchestrator` — composite routing by extension |

## 6. Alur Data Lengkap

```
Capability Checker butuh extract imports:
    │
    ├─► container.get_source_parser().extract_imports(path)
    │
    ▼
SourceParserOrchestrator.extract_imports(path)
    │
    ├─► select_parser(path):
    │     ".rs"  → ASTRustParserAdapter
    │
    ▼
ASTRustParserAdapter.extract_imports(path)
    │
    ├─► fs::read_to_string(path) → content string
    ├─► Bagi ke lines: Vec<&str>
    ├─► Untuk setiap line:
    │     ├─► USE_REGEX.captures(line) → simpan ImportInfo
    │     ├─► PUB_USE_REGEX.captures(line) → simpan re-export
    │     └─► ... pattern lainnya
    │
    └─► Return ImportInfoList { imports: Vec<ImportInfo> }

Caller dapat ImportInfoList — siap dipakai untuk check AES001/AES002
```

## 7. AES Compliance

| Rule | Compliance |
|------|------------|
| AES001 | Infrastructure parsers cuma import taxonomy/contract — gak import capability/agent/surface |
| AES002 | Setiap parser implements `ISourceParserPort` (mandatory contract) |
| AES003 | Filenames: `ast_rust_scanner`, `ast_py_scanner`, `ast_js_scanner`, `source_parser_adapter` — 3-word ✅ |
| AES008 | Contract file: `source_parser_port.rs` — suffix `_port` ✅ |
| AES011 | Infrastructure suffixes: `_scanner`, `_adapter` — allowed ✅ |
| AES027 | Setiap logic file implements contract trait ✅ |

## 8. Acceptance Criteria

| # | Kriteria | Status |
|---|----------|--------|
| AC001 | `extract_imports()` handle `use`, `import`, `require` untuk simple case | ⚠️ Partial — gagal di multi-line, group import, generics |
| AC002 | `is_barrel_file()` deteksi `mod.rs`, `__init__.py`, `index.ts` | ✅ Works (path-based) |
| AC003 | `find_primitive_violations()` tangkap String/i32/int di domain types | ⚠️ Partial — false positive dari string literal |
| AC004 | Routing `.rs` → Rust, `.py` → Python, `.js/.ts` → JS | ✅ Works |
| AC005 | `is_entry_point()` deteksi `fn main`, `__main__`, entry scripts | ⚠️ Partial — keyword matching simple |
| AC006 | `get_control_flow_count()` akurat | ⚠️ Partial — hitung keyword, bukan kontrol flow aktual |
| AC007 | `get_class_bases_map()` extract inheritance | ⚠️ Partial — single inheritance aja |
| AC008 | Barrel + entry point detection untuk 3 language | ✅ Works (path-based) |
| AC009 | True AST pakai `syn`/`ast`/`swc` | ❌ Missing — semua regex-based |
| AC010 | Production-ready: no false positive dari string literal | ❌ Missing |
| AC011 | `cargo check --bin lint-arwaky-cli` lulus | ✅ |
| AC012 | `cargo test` passes | ✅ |
