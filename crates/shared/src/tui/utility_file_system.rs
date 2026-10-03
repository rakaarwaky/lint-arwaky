// PURPOSE: utility_file_system — stateless filesystem utilities for TUI surfaces
// Pure functions only — no DI, no trait params, no contract imports.
use shared_common::FilePath;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Check whether a path points to a valid directory.
pub fn is_valid_directory(path: &FilePath) -> bool {
    Path::new(path.value()).is_dir()
}

/// Resolve the parent directory of a path.
pub fn parent_directory(path: &FilePath) -> Option<FilePath> {
    Path::new(path.value())
        .parent()
        .and_then(|p| FilePath::new(p.to_string_lossy().to_string()).ok())
}

/// Maximum time a clipboard helper is allowed to remain alive.
pub const CLIPBOARD_FALLBACK_TIMEOUT: Duration = Duration::from_secs(2);

/// Run the shell fallback with a hard deadline. `Child::wait` is deliberately
/// avoided: xclip and wl-copy can wait forever for a display server in SSH or
/// headless sessions. Dropping stdin before polling also gives well-behaved
/// helpers their EOF and prevents them waiting for more input.
fn run_fallback_with_timeout(script: &str, text: &str, timeout: Duration) -> bool {
    let mut child = match Command::new("sh")
        .arg("-c")
        .arg(script)
        .stdin(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return false,
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(text.as_bytes());
    }

    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return status.success(),
            Ok(None) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Ok(None) | Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return false;
            }
        }
    }
}

/// Copy text to the system clipboard.
/// Tries arboard first, then uses bounded xclip/wl-copy fallback work.
/// The caller should invoke this function on a worker thread; the action
/// handler does so so neither arboard nor a helper can block the TUI loop.
pub fn copy_text_to_clipboard(text: &str) -> bool {
    #[cfg(not(test))]
    {
        if let Ok(mut clipboard) = arboard::Clipboard::new()
            && clipboard.set_text(text).is_ok()
        {
            return true;
        }
    }

    run_fallback_with_timeout(
        "xclip -selection clipboard 2>/dev/null || wl-copy 2>/dev/null",
        text,
        CLIPBOARD_FALLBACK_TIMEOUT,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_is_killed_at_the_deadline() {
        let started = Instant::now();
        let copied = run_fallback_with_timeout("sleep 1", "test", Duration::from_millis(40));
        assert!(!copied);
        assert!(started.elapsed() < Duration::from_millis(500));
    }
}
