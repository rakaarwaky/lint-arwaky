// PURPOSE: Constants: Naming constants

pub const RUST_PRIMITIVE_TYPES: &[&str] = &[
    "String", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128",
    "usize", "f32", "f64", "bool", "char", "Vec<", "HashMap<", "Option<", "Result<", "Box<",
    "Cell<", "RefCell<", "Arc<", "Mutex<", "Rc<",
];

// Python primitives
pub const PYTHON_PRIMITIVE_TYPES: &[&str] = &[
    "str",
    "int",
    "float",
    "bool",
    "list",
    "dict",
    "tuple",
    "set",
    "bytes",
    "None",
    "Any",
    "Optional",
    "Union",
    "List",
    "Dict",
    "Tuple",
    "Set",
    "FrozenSet",
];

// JavaScript / TypeScript primitives
pub const JS_PRIMITIVE_TYPES: &[&str] = &[
    "string",
    "number",
    "boolean",
    "any",
    "object",
    "Array",
    "Record",
    "Map",
    "Set",
    "Promise",
    "unknown",
    "never",
    "void",
    "null",
    "undefined",
    "bigint",
    "symbol",
];

// Legacy alias used by PrimitiveTypeList constructor
pub const CORE_PRIMITIVE_TYPES: &[&str] = &["str", "int", "float"];
