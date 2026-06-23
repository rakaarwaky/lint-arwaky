// PURPOSE: GitRefVO — value object for git reference (branch, tag)
//
// `GitRef` is a thin string wrapper for git references (branch names, tag
// names, HEAD, etc.). It is generated with the `string_value_object!` macro
// so dependents pick up the standard `new`/`value`/`Default`/`Hash`/serde
// surface for free. Lives in its own file to avoid forcing every git-hooks
// consumer to pull in the rest of the common VO namespace.
use crate::string_value_object;

string_value_object!(GitRef);