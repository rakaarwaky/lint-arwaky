// PURPOSE: Shared constants for the auto-fix feature.

use shared_common::ErrorCode;

/// Error codes that auto-fix can handle.
pub static FIXABLE_CODES: once_cell::sync::Lazy<Vec<ErrorCode>> =
    once_cell::sync::Lazy::new(|| {
        vec![
            ErrorCode::raw("AES101"),
            ErrorCode::raw("AES304"),
            ErrorCode::raw("AES203"),
        ]
    });

/// Rust keywords that cannot be used as symbol names.
pub static RUST_KEYWORDS: once_cell::sync::Lazy<Vec<&str>> = once_cell::sync::Lazy::new(|| {
    vec![
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
        "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "type",
        "unsafe", "use", "where", "while", "yield",
    ]
});
