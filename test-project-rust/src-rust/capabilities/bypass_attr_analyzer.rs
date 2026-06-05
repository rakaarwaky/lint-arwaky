// AES014 — bypass-comment-violation
// This file contains #[allow(...)] attributes which are Rust's equivalent of
// noqa / type:ignore — they suppress linter warnings and mask design flaws.

use crate::taxonomy::removal_types::RemovalType;

#[allow(dead_code)]
#[allow(unused_variables)]
pub struct BypassAttrAnalyzer {
    pub value: String,
}

impl BypassAttrAnalyzer {
    #[allow(unused)]
    pub fn check_something(&self) -> bool {
        #[allow(clippy::needless_return)]
        let unused_var = "this is never used";
        return true;
    }

    #[allow(clippy::all)]
    pub fn complex_bypass(&self, items: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in items {
            result += i;
        }
        result
    }
}
