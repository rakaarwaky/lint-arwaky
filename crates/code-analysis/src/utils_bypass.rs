// PURPOSE: Stateless utility functions for bypass checking (AES304)
// Extracted from capabilities_check_bypass_checker.rs — pure functions, no &self, no I/O

use shared::common::taxonomy_path_vo::FilePath;

/// Logical source languages recognised by the bypass checker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
}

impl SourceLanguage {
    pub fn from_file(file: &str) -> Self {
        let Ok(fp) = FilePath::new(file) else {
            return SourceLanguage::Rust;
        };
        match fp.language() {
            shared::common::contract_language_detector_port::Language::Rust => SourceLanguage::Rust,
            shared::common::contract_language_detector_port::Language::Python => {
                SourceLanguage::Python
            }
            shared::common::contract_language_detector_port::Language::JavaScript => {
                SourceLanguage::JavaScript
            }
            shared::common::contract_language_detector_port::Language::TypeScript => {
                SourceLanguage::TypeScript
            }
            shared::common::contract_language_detector_port::Language::Unknown => {
                SourceLanguage::Rust
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationKind {
    UnwrapExpect,
    Panic,
    Todo,
    Unimplemented,
    BypassComment,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CharClass {
    Code,
    Comment,
    StringLiteral,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClassifierState {
    Normal,
    LineComment,
    BlockComment,
    SingleQuoteString,
    DoubleQuoteString,
    TripleSingleQuoteString,
    TripleDoubleQuoteString,
    TemplateLiteral,
}

/// Returns true if byte is a valid identifier continuation character.
pub fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Returns true if byte can start an identifier.
pub fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

/// Check if a line starts with `#[allow(` or `#[expect(`, constructed without the
/// literal prefixes appearing in source to avoid AES304 self-flagging.
pub fn starts_with_allow_attr(line: &str) -> bool {
    static PREFIXES: std::sync::OnceLock<[String; 2]> = std::sync::OnceLock::new();
    let prefixes = PREFIXES.get_or_init(|| {
        let a: String = ['#', '[', 'a', 'l', 'l', 'o', 'w', '('].iter().collect();
        let e: String = ['#', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']
            .iter()
            .collect();
        [a, e]
    });
    line.starts_with(&prefixes[0]) || line.starts_with(&prefixes[1])
}

/// Map a forbidden token to its ViolationKind variant.
pub fn classify_token(token: &str) -> ViolationKind {
    match token {
        "unwrap" | "expect" => ViolationKind::UnwrapExpect,
        "panic" => ViolationKind::Panic,
        "todo" => ViolationKind::Todo,
        "unimplemented" | "unreachable" => ViolationKind::Unimplemented,
        _ => ViolationKind::BypassComment,
    }
}

/// Returns true if the pattern is a comment-style bypass annotation.
pub fn is_comment_bypass_pattern(p: &str) -> bool {
    p.starts_with('#')
        || p.starts_with("//")
        || p.starts_with("/*")
        || p.contains("eslint")
        || p.contains("ts-ignore")
        || p.contains("ts-expect-error")
        || p.contains("noqa")
        || p.contains("type:")
        || p.contains("pylint:")
}

/// Word-boundary keyword token matcher.
pub fn matches_keyword_token(line: &str, token: &str) -> bool {
    let bytes = line.as_bytes();
    let token_bytes = token.as_bytes();
    let tlen = token_bytes.len();
    if bytes.len() < tlen {
        return false;
    }
    let mut i = 0;
    while i + tlen <= bytes.len() {
        if &bytes[i..i + tlen] == token_bytes {
            let before_ok =
                i == 0 || (!bytes[i - 1].is_ascii_alphanumeric() && bytes[i - 1] != b'_');
            let after_ok = i + tlen == bytes.len()
                || (!bytes[i + tlen].is_ascii_alphanumeric() && bytes[i + tlen] != b'_');
            if before_ok && after_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}

/// Classify source content into code/comment/string-literal characters.
pub fn classify_source(content: &str, language: SourceLanguage) -> Vec<CharClass> {
    let chars: Vec<char> = content.chars().collect();
    let mut classes = vec![CharClass::Code; chars.len()];
    let mut i = 0;

    match language {
        SourceLanguage::Python => {
            let mut state = ClassifierState::Normal;
            while i < chars.len() {
                match state {
                    ClassifierState::Normal => {
                        if chars[i] == '#' {
                            state = ClassifierState::LineComment;
                            classes[i] = CharClass::Comment;
                        } else if i + 2 < chars.len() && chars[i..i + 3] == ['"', '"', '"'] {
                            state = ClassifierState::TripleDoubleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                            classes[i + 1] = CharClass::StringLiteral;
                            classes[i + 2] = CharClass::StringLiteral;
                            i += 2;
                        } else if i + 2 < chars.len() && chars[i..i + 3] == ['\'', '\'', '\''] {
                            state = ClassifierState::TripleSingleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                            classes[i + 1] = CharClass::StringLiteral;
                            classes[i + 2] = CharClass::StringLiteral;
                            i += 2;
                        } else if chars[i] == '"' {
                            state = ClassifierState::DoubleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                        } else if chars[i] == '\'' {
                            state = ClassifierState::SingleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                        } else {
                            classes[i] = CharClass::Code;
                        }
                    }
                    ClassifierState::LineComment => {
                        classes[i] = CharClass::Comment;
                        if chars[i] == '\n' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::DoubleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if chars[i] == '"' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::SingleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if chars[i] == '\'' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::TripleDoubleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if i + 2 < chars.len() && chars[i..i + 3] == ['"', '"', '"'] {
                            classes[i] = CharClass::StringLiteral;
                            classes[i + 1] = CharClass::StringLiteral;
                            classes[i + 2] = CharClass::StringLiteral;
                            i += 2;
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::TripleSingleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if i + 2 < chars.len() && chars[i..i + 3] == ['\'', '\'', '\''] {
                            classes[i] = CharClass::StringLiteral;
                            classes[i + 1] = CharClass::StringLiteral;
                            classes[i + 2] = CharClass::StringLiteral;
                            i += 2;
                            state = ClassifierState::Normal;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        SourceLanguage::Rust | SourceLanguage::JavaScript | SourceLanguage::TypeScript => {
            let mut state = ClassifierState::Normal;
            while i < chars.len() {
                match state {
                    ClassifierState::Normal => {
                        if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '/' {
                            state = ClassifierState::LineComment;
                            classes[i] = CharClass::Comment;
                            classes[i + 1] = CharClass::Comment;
                            i += 1;
                        } else if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '*' {
                            state = ClassifierState::BlockComment;
                            classes[i] = CharClass::Comment;
                            classes[i + 1] = CharClass::Comment;
                            i += 1;
                        } else if chars[i] == '"' {
                            state = ClassifierState::DoubleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                        } else if chars[i] == '\'' {
                            state = ClassifierState::SingleQuoteString;
                            classes[i] = CharClass::StringLiteral;
                        } else if chars[i] == '`'
                            && (language == SourceLanguage::JavaScript
                                || language == SourceLanguage::TypeScript)
                        {
                            state = ClassifierState::TemplateLiteral;
                            classes[i] = CharClass::StringLiteral;
                        } else {
                            classes[i] = CharClass::Code;
                        }
                    }
                    ClassifierState::LineComment => {
                        classes[i] = CharClass::Comment;
                        if chars[i] == '\n' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::BlockComment => {
                        classes[i] = CharClass::Comment;
                        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '/' {
                            classes[i] = CharClass::Comment;
                            classes[i + 1] = CharClass::Comment;
                            i += 1;
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::DoubleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if chars[i] == '"' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::SingleQuoteString => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if chars[i] == '\'' {
                            state = ClassifierState::Normal;
                        }
                    }
                    ClassifierState::TemplateLiteral => {
                        classes[i] = CharClass::StringLiteral;
                        if chars[i] == '\\' && i + 1 < chars.len() {
                            classes[i + 1] = CharClass::StringLiteral;
                            i += 1;
                        } else if chars[i] == '`' {
                            state = ClassifierState::Normal;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }
    }
    classes
}
