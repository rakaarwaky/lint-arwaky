// Unit tests — shared language parsers (Python/TypeScript/Rust).
//
// Provides a regression safety net for the core parsing functions that feed
// import-rules, role-rules, and orphan-rules. These parsers had no inline
// coverage; this file pins their current (correct) behavior so subsequent
// complexity refactors cannot silently change results.
mod common;

use shared_lint_arwaky::common::utility_python_parser::parse_python;
use shared_lint_arwaky::common::utility_rust_parser::parse_rust;
use shared_lint_arwaky::common::utility_ts_parser::parse_ts;

// ── Python ───────────────────────────────────────────────────
#[test]
fn python_parses_from_and_plain_imports() {
    let src = "from os import path, getcwd\nimport sys as system\n";
    let r = parse_python(src);
    assert!(r.parse_ok);

    let raw_paths: Vec<&str> = r.imports.iter().map(|i| i.raw_path.as_str()).collect();
    assert!(raw_paths.contains(&"os"), "from-import base module missing: {:?}", raw_paths);
    assert!(raw_paths.contains(&"sys"), "plain import missing: {:?}", raw_paths);
    // `from os import path, getcwd` expands to dotted name imports.
    assert!(
        raw_paths.contains(&"os.path"),
        "expanded name import missing: {:?}",
        raw_paths
    );
    assert!(raw_paths.contains(&"os.getcwd"));
}

#[test]
fn python_parses_class_bases_and_functions() {
    // `def foo(): ...` is a dummy (ellipsis body detected from the signature
    // line); `async def bar()` is not (dummy detection only inspects the
    // signature line, not the body).
    let src = "class Animal(Mammal):\n    pass\n\ndef foo(): ...\n\nasync def bar():\n    pass\n";
    let r = parse_python(src);

    assert_eq!(r.class_bases, vec![("Animal".to_string(), vec!["Mammal".to_string()])]);

    let names: Vec<&str> = r.functions.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names, vec!["foo", "bar"]);

    let foo = r.functions.iter().find(|f| f.name == "foo").unwrap();
    assert!(foo.is_dummy, "def with `...` body should be dummy");
    let bar = r.functions.iter().find(|f| f.name == "bar").unwrap();
    assert!(!bar.is_dummy);

    // Used identifiers exclude import/from/# lines; `foo` from `def foo` is collected.
    assert!(r.used_identifiers.contains(&"foo".to_string()));
}

#[test]
fn python_strips_comments_before_parsing() {
    let src = "# this is a comment\nx = 1\n";
    let r = parse_python(src);
    assert!(r.used_identifiers.contains(&"x".to_string()));
    assert!(!r.used_identifiers.iter().any(|i| i.contains("comment")));
}

// ── TypeScript / JavaScript ─────────────────────────────────
#[test]
fn ts_parses_named_and_glob_imports() {
    let src = "import { foo } from './bar';\nimport * as baz from './qux';\nexport { x } from './y';\n";
    let r = parse_ts(src);
    assert!(r.parse_ok);

    let raw_paths: Vec<&str> = r.imports.iter().map(|i| i.raw_path.as_str()).collect();
    assert!(raw_paths.contains(&"./bar"));
    let glob = r.imports.iter().find(|i| i.raw_path == "./qux").unwrap();
    assert!(glob.is_glob, "import * as should be a glob");
    let reexport = r.imports.iter().find(|i| i.raw_path == "./y").unwrap();
    assert!(reexport.is_reexport, "export ... from should be a reexport");
}

#[test]
fn ts_parses_class_implements_and_interfaces() {
    // Note: only *exported* interfaces are captured by the parser (bare
    // `interface` declarations are ignored), so we exercise the exported form.
    let src = "export interface IFoo {}\nclass C implements IFoo {}\nfunction f() {}\n";
    let r = parse_ts(src);
    assert_eq!(r.interface_names, vec!["IFoo".to_string()]);
    assert_eq!(r.class_implements, vec![("C".to_string(), vec!["IFoo".to_string()])]);
    assert_eq!(r.functions.len(), 1);
    assert_eq!(r.functions[0].name, "f");
}

#[test]
fn ts_strips_block_comments_before_parsing() {
    let src = "/* ignored */\nconst y = 2;\n";
    let r = parse_ts(src);
    assert!(r.used_identifiers.contains(&"y".to_string()));
}

// ── Rust ────────────────────────────────────────────────────
#[test]
fn rust_parses_use_and_trait_impls() {
    let src = "use std::collections::HashMap;\ntrait Foo {}\nstruct Bar;\nimpl Foo for Bar {}\nfn helper() {}\n";
    let r = parse_rust(src);
    assert!(r.parse_ok);

    let raw_paths: Vec<&str> = r.imports.iter().map(|i| i.raw_path.as_str()).collect();
    assert!(raw_paths.contains(&"std::collections::HashMap"));
    assert_eq!(r.structs.len(), 1);
    assert_eq!(r.structs[0].name, "Bar");
    assert!(r.has_trait_impl("Foo"));
    assert_eq!(r.functions.len(), 1);
    assert_eq!(r.functions[0].name, "helper");
}
