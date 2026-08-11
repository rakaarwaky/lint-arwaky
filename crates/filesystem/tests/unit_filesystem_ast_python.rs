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
    let content = "@register(\"engine\")\nclass AppEngine(AppProtocol):\n    def run(self):\n        pass\n";
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
    let content =
        "@register(\"engine\")\n@final\nclass AppEngine(AppProtocol):\n    pass\n";
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