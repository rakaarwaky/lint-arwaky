use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

shared_lint_arwaky::string_value_object!(MyTestVO);

#[test]
fn string_vo_new_and_value() {
    let v = MyTestVO::new("hello");
    assert_eq!(v.value(), "hello");
    let v2 = MyTestVO::new(String::from("world"));
    assert_eq!(v2.value(), "world");
}

#[test]
fn string_vo_default_is_empty() {
    let v = MyTestVO::default();
    assert_eq!(v.value(), "");
}

#[test]
fn string_vo_display() {
    let v = MyTestVO::new("hello");
    assert_eq!(v.to_string(), "hello");
}

#[test]
fn string_vo_from_str() {
    let v: MyTestVO = "abc".into();
    assert_eq!(v.value(), "abc");
}

#[test]
fn string_vo_from_string() {
    let v: MyTestVO = String::from("xyz").into();
    assert_eq!(v.value(), "xyz");
}

#[test]
fn string_vo_equality() {
    assert_eq!(MyTestVO::new("a"), MyTestVO::new("a"));
    assert_ne!(MyTestVO::new("a"), MyTestVO::new("b"));
}

#[test]
fn string_vo_hash_matches_inner() {
    let v = MyTestVO::new("hashable");
    let mut h1 = DefaultHasher::new();
    let mut h2 = DefaultHasher::new();
    v.hash(&mut h1);
    "hashable".hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn string_vo_deserialize_from_string() {
    let v: MyTestVO = serde_json::from_str("\"hello\"").unwrap();
    assert_eq!(v.value(), "hello");
}

#[test]
fn string_vo_deserialize_from_map() {
    let v: MyTestVO = serde_json::from_str("{\"value\":\"wrapped\"}").unwrap();
    assert_eq!(v.value(), "wrapped");
}

#[test]
fn string_vo_serialize_transparent() {
    let v = MyTestVO::new("plain");
    let s = serde_json::to_string(&v).unwrap();
    assert_eq!(s, "\"plain\"");
}

shared_lint_arwaky::primitive_value_object!(MyNum, i64);

#[test]
fn primitive_vo_new_and_value() {
    let v = MyNum::new(42);
    assert_eq!(v.value(), 42);
}

#[test]
fn primitive_vo_display() {
    let v = MyNum::new(123);
    assert_eq!(v.to_string(), "123");
}

#[test]
fn primitive_vo_from_inner() {
    let v: MyNum = 7.into();
    assert_eq!(v.value(), 7);
}

#[test]
fn primitive_vo_deserialize_from_int() {
    let v: MyNum = serde_json::from_str("99").unwrap();
    assert_eq!(v.value(), 99);
}

#[test]
fn primitive_vo_deserialize_from_map() {
    let v: MyNum = serde_json::from_str("{\"value\":11}").unwrap();
    assert_eq!(v.value(), 11);
}

#[test]
fn score_deserialize_from_integer() {
    let score: shared_lint_arwaky::common::taxonomy_common_vo::Score =
        serde_json::from_str("80").unwrap();
    assert_eq!(score.value, 80.0);
}

#[test]
fn score_deserialize_from_float() {
    let score: shared_lint_arwaky::common::taxonomy_common_vo::Score =
        serde_json::from_str("80.5").unwrap();
    assert_eq!(score.value, 80.5);
}
