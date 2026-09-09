// Unit tests — utility_ast_python::extract_python_metadata.
//
// Focused on the `decorated_definition` unwrapping added for decorated classes
// (e.g. `@register("engine")\nclass Foo(Base): ...`), which previously caused
// the wrapped class's name/bases to be missed entirely since tree-sitter
// represents decorated declarations as a `decorated_definition` node rather
// than a bare `class_definition`.

use filesystem_lint_arwaky::utility_ast_python::extract_python_metadata;

fn parse(content: &str) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
        .unwrap();
    parser.parse(content, None).unwrap()
}

#[test]
fn decorated_class_extracts_name_and_bases() {
    let content =
        "@register(\"engine\")\nclass AppEngine(AppProtocol):\n    def run(self):\n        pass\n";
    let tree = parse(content);
    let meta = extract_python_metadata(&tree, content);

    assert_eq!(meta.class_declarations.len(), 1);
    let class = &meta.class_declarations[0];
    assert_eq!(class.name, "AppEngine");
    assert_eq!(class.bases, vec!["AppProtocol".to_string()]);
}

#[test]
fn decorated_class_without_bases_still_extracts_name() {
    let content = "@dataclass\nclass Point:\n    x: int\n    y: int\n";
    let tree = parse(content);
    let meta = extract_python_metadata(&tree, content);

    assert_eq!(meta.class_declarations.len(), 1);
    assert_eq!(meta.class_declarations[0].name, "Point");
    assert!(meta.class_declarations[0].bases.is_empty());
}

#[test]
fn multiple_stacked_decorators_still_extracts_class() {
    let content = "@register(\"engine\")\n@final\nclass AppEngine(AppProtocol):\n    pass\n";
    let tree = parse(content);
    let meta = extract_python_metadata(&tree, content);

    assert_eq!(meta.class_declarations.len(), 1);
    assert_eq!(meta.class_declarations[0].name, "AppEngine");
    assert_eq!(
        meta.class_declarations[0].bases,
        vec!["AppProtocol".to_string()]
    );
}

#[test]
fn undecorated_class_still_extracted_as_before() {
    // Regression: plain (non-decorated) classes must keep working exactly as
    // before this change.
    let content = "class AppEngine(AppProtocol):\n    pass\n";
    let tree = parse(content);
    let meta = extract_python_metadata(&tree, content);

    assert_eq!(meta.class_declarations.len(), 1);
    assert_eq!(meta.class_declarations[0].name, "AppEngine");
    assert_eq!(
        meta.class_declarations[0].bases,
        vec!["AppProtocol".to_string()]
    );
}

#[test]
fn decorated_function_is_not_misparsed_as_class() {
    // A decorated *function* definition also produces a `decorated_definition`
    // node, but its inner definition kind is `function_definition`, not
    // `class_definition` — it must not be added to class_declarations.
    let content = "@staticmethod\ndef helper():\n    pass\n";
    let tree = parse(content);
    let meta = extract_python_metadata(&tree, content);

    assert!(meta.class_declarations.is_empty());
}

#[test]
fn mixed_decorated_and_plain_classes_both_extracted() {
    let content = "\
class PlainOne:\n    pass\n\n\
@register(\"two\")\nclass DecoratedTwo(BaseTwo):\n    pass\n\n\
class PlainThree(BaseThree):\n    pass\n";
    let tree = parse(content);
    let meta = extract_python_metadata(&tree, content);

    let names: Vec<&str> = meta
        .class_declarations
        .iter()
        .map(|c| c.name.as_str())
        .collect();
    assert_eq!(names, vec!["PlainOne", "DecoratedTwo", "PlainThree"]);
    assert_eq!(
        meta.class_declarations[1].bases,
        vec!["BaseTwo".to_string()]
    );
}

#[test]
fn decorated_class_with_multiple_bases() {
    let content = "@register(\"engine\")\nclass AppEngine(AppProtocol, Mixin):\n    pass\n";
    let tree = parse(content);
    let meta = extract_python_metadata(&tree, content);

    assert_eq!(meta.class_declarations.len(), 1);
    assert_eq!(
        meta.class_declarations[0].bases,
        vec!["AppProtocol".to_string(), "Mixin".to_string()]
    );
}

// ─── AES203 regression: identifiers inside f-string interpolations ──────

use filesystem_lint_arwaky::utility_ast_python::extract_python_identifiers;

#[test]
fn fstring_interpolation_identifiers_are_extracted() {
    // Regression: identifiers referenced inside `{...}` of an f-string must
    // count as real usages (AES203 false positive otherwise).
    let content = "def f(x):\n    return f\"a {x} b {foo(x)} c\"\n";
    let tree = parse(content);
    let ids = extract_python_identifiers(&tree, content);

    assert!(ids.iter().any(|s| s == "x"), "x not found: {ids:?}");
    assert!(ids.iter().any(|s| s == "foo"), "foo not found: {ids:?}");
    assert!(ids.iter().any(|s| s == "f"), "f not found: {ids:?}");
}

#[test]
fn plain_string_content_is_not_leaked_as_identifiers() {
    // Plain (non-f) strings must not leak their content as identifiers.
    let content = "greeting = \"hello world\"\n";
    let tree = parse(content);
    let ids = extract_python_identifiers(&tree, content);

    assert_eq!(ids, vec!["greeting".to_string()], "ids: {ids:?}");
}

#[test]
fn concatenated_fstring_interpolations_are_extracted() {
    // Adjacent literal + f-string parts: interpolations must still be counted.
    let content = "def g(x):\n    return \"pre\" f\"-{x}-\" \"post\"\n";
    let tree = parse(content);
    let ids = extract_python_identifiers(&tree, content);

    assert!(ids.iter().any(|s| s == "x"), "x not found: {ids:?}");
    assert!(ids.iter().any(|s| s == "g"), "g not found: {ids:?}");
}

#[test]
fn aes203_regression_escape_used_inside_fstring_markup() {
    // Exact shape of the real-world regression: rich.markup.escape is used
    // only inside an f-string; it must be detected as used, not flagged
    // AES203 UNUSED_IMPORT.
    let content = concat!(
        "from rich.markup import escape\n",
        "def render(v):\n",
        "    return f\"[bold red]{escape(v)}[/]\"\n",
    );
    let tree = parse(content);
    let ids = extract_python_identifiers(&tree, content);

    assert!(
        ids.iter().any(|s| s == "escape"),
        "escape not detected inside f-string: {ids:?}"
    );
    assert!(ids.iter().any(|s| s == "v"), "v not found: {ids:?}");
}
