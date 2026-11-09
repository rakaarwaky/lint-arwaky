// PURPOSE: NamingFinding — the three fields one naming finding carries.
//
// The naming rules used to write the same defect three times: the message
// said what was wrong and what to do about it, the `why` said what was wrong
// again, and the `fix` said what to do about it again. A surface that printed
// all three showed the reader the same sentence two or three times.
//
// The finding therefore carries three distinct fields: the problem, the
// action, and the rule. This file holds the value; the text itself is built
// by the utility layer, which owns the width budget and the clause order.

/// The three fields one naming finding carries.
///
/// `message` is the problem, `fix` is the action, and `why` is the rule the
/// finding comes from. The three never restate one another, so a surface that
/// prints all three shows the defect exactly once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamingFinding {
    /// The problem: what is wrong, in one clause.
    pub message: String,
    /// The action: what the reader has to do about it.
    pub fix: String,
    /// The rule: why the product cares.
    pub why: String,
}

impl NamingFinding {
    /// A finding whose three fields are already rendered.
    ///
    /// The clause order of the constructor is the template's order: the
    /// problem, then the action, then the rule.
    pub fn new(
        message: impl Into<String>,
        fix: impl Into<String>,
        why: impl Into<String>,
    ) -> Self {
        Self {
            message: message.into(),
            fix: fix.into(),
            why: why.into(),
        }
    }
}
