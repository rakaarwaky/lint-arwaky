// PURPOSE: WatchResponse — response payload for the watch aggregate

use shared_common::taxonomy_common_error::ExitCode;

pub enum WatchResponse {
    /// Terminal status of a completed run request.
    Run { exit_code: ExitCode },
    /// Answer to an `IsLintable` request.
    IsLintable { lintable: bool },
}
