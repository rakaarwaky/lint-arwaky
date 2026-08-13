// Unit tests — import extraction (FR-001): alias binding tracking.
// Regression: AES203 false positive on `import ... as <alias>` bindings.
// The module-scope binding must be the alias (`z` in `from x import y as z`),
// not the original name, so usage analysis can resolve it.

use filesystem_lint_arwaky::utility_import_extractor::extract_imports;
use shared::filesystem::taxonomy_filesystem_vo::Language;
use std::path::PathBuf;

fn symbols_of(content: &str, language: Language) -> Vec<String> {
    let imports = extract_imports(
        &PathBuf::from("/tmp/test/src/file.py"),
        content,
        language,
        None,
    );
    imports.iter().flat_map(|i| i.symbols.clone()).collect()
}

#[test]
fn python_aliased_from_import_uses_alias_binding() {
    // `from x import y as z` binds `z` in module scope — the original name `y`
    // must NOT be treated as the binding (AES203 regression).
    let symbols = symbols_of(
        "from modules.shared.src.utility_core_validation import validate_file as _validate_file_util\n",
        Language::Python,
    );
    assert_eq!(
        symbols,
        vec!["_validate_file_util".to_string()],
        "Aliased import must record the alias as the module-scope binding"
    );
}

#[test]
fn python_plain_from_import_keeps_original_name() {
    // Non-aliased imports keep the original name as binding.
    let symbols = symbols_of("from typing import Any\n", Language::Python);
    assert_eq!(symbols, vec!["Any".to_string()]);
}

#[test]
fn python_aliased_import_statement_uses_alias_binding() {
    // `import json as j` binds `j` — the original module name must not be the binding.
    let symbols = symbols_of("import json as j\n", Language::Python);
    assert_eq!(symbols, vec!["j".to_string()]);
}

#[test]
fn python_dotted_import_statement_binds_top_level_module() {
    // `import os.path` binds `os` (top-level package), not `os.path`.
    let symbols = symbols_of("import os.path\n", Language::Python);
    assert_eq!(symbols, vec!["os".to_string()]);
}

#[test]
fn python_mixed_import_statement_bindings() {
    // `import os, sys as s` → bindings `os` and `s`.
    let symbols = symbols_of("import os, sys as s\n", Language::Python);
    assert_eq!(symbols, vec!["os".to_string(), "s".to_string()]);
}

#[test]
fn python_parenthesized_from_import_bindings() {
    // `from pkg import (a, b as c)` → bindings `a` and `c`.
    let symbols = symbols_of("from pkg import (a, b as c)\n", Language::Python);
    assert_eq!(symbols, vec!["a".to_string(), "c".to_string()]);
}

#[test]
fn rust_aliased_use_import_uses_alias_binding() {
    // `use foo::bar as baz` binds `baz`; the raw path stays `foo::bar`.
    let imports = extract_imports(
        &PathBuf::from("/tmp/test/src/file.rs"),
        "use foo::bar as baz;\n",
        Language::Rust,
        None,
    );
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].raw_path, "foo::bar");
    assert_eq!(imports[0].symbols, vec!["baz".to_string()]);
}

#[test]
fn rust_grouped_aliased_use_import_uses_alias_binding() {
    // `use qux::{a as aa, b}` → raw paths `qux::a`/`qux::b`, bindings `aa`/`b`.
    let imports = extract_imports(
        &PathBuf::from("/tmp/test/src/file.rs"),
        "use qux::{a as aa, b};\n",
        Language::Rust,
        None,
    );
    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0].raw_path, "qux::a");
    assert_eq!(imports[0].symbols, vec!["aa".to_string()]);
    assert_eq!(imports[1].raw_path, "qux::b");
    assert!(imports[1].symbols.is_empty());
}

#[test]
fn rust_plain_use_import_has_no_symbols() {
    // Plain `use foo::bar` keeps the existing shape (empty symbols → path fallback).
    let imports = extract_imports(
        &PathBuf::from("/tmp/test/src/file.rs"),
        "use foo::bar;\n",
        Language::Rust,
        None,
    );
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].raw_path, "foo::bar");
    assert!(imports[0].symbols.is_empty());
}

#[test]
fn typescript_aliased_named_import_uses_alias_binding() {
    // `import { Foo as Bar } from './mod'` binds `Bar`, not `Foo`.
    let imports = extract_imports(
        &PathBuf::from("/tmp/test/src/file.ts"),
        "import { Foo as Bar } from './mod';\n",
        Language::TypeScript,
        None,
    );
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].raw_path, "./mod");
    assert_eq!(imports[0].symbols, vec!["Bar".to_string()]);
}

#[test]
fn typescript_default_import_uses_binding() {
    // `import Foo from './mod'` binds `Foo` — previously the raw path was
    // used as the alias key, causing an AES203 false positive.
    let imports = extract_imports(
        &PathBuf::from("/tmp/test/src/file.ts"),
        "import Foo from './mod';\n",
        Language::TypeScript,
        None,
    );
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].raw_path, "./mod");
    assert_eq!(imports[0].symbols, vec!["Foo".to_string()]);
}

#[test]
fn typescript_default_import_with_named_sibling_binds_default() {
    // `import Foo, { Bar } from './mod'` binds `Foo` (no trailing comma leaked).
    let imports = extract_imports(
        &PathBuf::from("/tmp/test/src/file.ts"),
        "import Foo, { Bar } from './mod';\n",
        Language::TypeScript,
        None,
    );
    assert_eq!(imports.len(), 1);
    assert_eq!(
        imports[0].symbols,
        vec!["Bar".to_string(), "Foo".to_string()]
    );
}

#[test]
fn typescript_default_import_multiline_binds_binding() {
    // `import Foo\nfrom './mod'` binds `Foo` across the newline.
    let imports = extract_imports(
        &PathBuf::from("/tmp/test/src/file.ts"),
        "import Foo\n  from './mod';\n",
        Language::TypeScript,
        None,
    );
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].symbols, vec!["Foo".to_string()]);
}

#[test]
fn typescript_namespace_import_binds_namespace_alias() {
    // `import * as utils from './utils'` binds `utils`; wildcard flag stays set.
    let imports = extract_imports(
        &PathBuf::from("/tmp/test/src/file.ts"),
        "import * as utils from './utils';\n",
        Language::TypeScript,
        None,
    );
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].raw_path, "./utils");
    assert_eq!(imports[0].symbols, vec!["utils".to_string()]);
    assert!(imports[0].is_wildcard);
}

#[test]
fn rust_grouped_use_with_visibility_modifier_extracts_clean_path() {
    // `pub(crate) use qux::{a as aa, b}` — visibility keywords must not leak
    // into the module path.
    let imports = extract_imports(
        &PathBuf::from("/tmp/test/src/file.rs"),
        "pub(crate) use qux::{a as aa, b};\n",
        Language::Rust,
        None,
    );
    assert_eq!(imports.len(), 2);
    assert_eq!(imports[0].raw_path, "qux::a");
    assert_eq!(imports[0].symbols, vec!["aa".to_string()]);
    assert_eq!(imports[1].raw_path, "qux::b");
    assert!(imports[1].symbols.is_empty());
}
