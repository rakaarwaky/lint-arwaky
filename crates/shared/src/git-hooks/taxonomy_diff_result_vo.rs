// PURPOSE: Re-export GitDiffResultVO from file-watch for git-hooks module
//
// This file exists so dependents inside `git-hooks` can import the type via
// `git_hooks::taxonomy_diff_result_vo::GitDiffResultVO` without depending on
// the file-watch crate directly. The real definition lives in
// `file_watch::taxonomy_diff_result_vo` and is re-exported here.
pub use crate::file_watch::taxonomy_diff_result_vo::GitDiffResultVO;
