// PURPOSE: Section — the value object describing one parsed Markdown heading.
/// A markdown heading with its body and 1-based start line.
#[derive(Clone, Debug)]
pub struct Section {
    /// Heading depth: 1 for `#`, 2 for `##`, and so on.
    pub level: usize,
    /// Heading text without the leading `#` markers.
    pub title: String,
    /// Text between this heading and the next heading of the same or higher rank.
    pub body: String,
    /// 1-based line the heading starts on.
    pub line: usize,
}

impl Section {
    /// Build one parsed heading section.
    pub fn new(level: usize, title: String, body: String, line: usize) -> Self {
        Self {
            level,
            title,
            body,
            line,
        }
    }

    /// The heading depth, named for the level the AES contracts speak in.
    pub fn level(&self) -> usize {
        self.level
    }

    /// The heading text without its `#` markers.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The text this heading owns, up to the next same-or-higher-rank heading.
    pub fn body(&self) -> &str {
        &self.body
    }

    /// The 1-based line the heading starts on.
    pub fn line(&self) -> usize {
        self.line
    }
}
