// PURPOSE: Subprocess timeout helpers for the check/scan surface.
//
// Owns the timeout budget for self-invoked linter subprocesses and the
// reader-thread drain that keeps a killed child from hanging the caller.
use std::process::{Child, Output};
use std::time::{Duration, Instant};

/// Timeout budget for a self-invoked linter subprocess, overridable through
/// `LINT_ARWAKY_SUBPROCESS_TIMEOUT_SECS`.
pub fn subprocess_timeout() -> Duration {
    std::env::var("LINT_ARWAKY_SUBPROCESS_TIMEOUT_SECS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(60))
}

/// Drain a child's stdout/stderr concurrently while waiting for it, killing
/// and erroring out once `timeout` elapses.
pub fn wait_for_child(mut child: Child, timeout: Duration) -> Result<Output, String> {
    let stdout = child.stdout.take().ok_or("stdout pipe unavailable")?;
    let stderr = child.stderr.take().ok_or("stderr pipe unavailable")?;
    let stdout_reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let mut pipe = stdout;
        std::io::Read::read_to_end(&mut pipe, &mut bytes).map(|_| bytes)
    });
    let stderr_reader = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let mut pipe = stderr;
        std::io::Read::read_to_end(&mut pipe, &mut bytes).map(|_| bytes)
    });
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() < timeout => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                // Do NOT join the reader threads here: if the killed child
                // forked grandchildren that inherited the pipes, a join would
                // block until they exit — exactly the hang the timeout was
                // meant to prevent. Dropping the handles detaches the readers;
                // they finish at pipe EOF and cannot outlive the process.
                drop(stdout_reader);
                drop(stderr_reader);
                return Err(format!("timed out after {}s", timeout.as_secs_f64()));
            }
            Err(error) => return Err(format!("wait failed: {error}")),
        }
    };
    let stdout = stdout_reader
        .join()
        .map_err(|_| "stdout reader panicked".to_string())?
        .map_err(|error| error.to_string())?;
    let stderr = stderr_reader
        .join()
        .map_err(|_| "stderr reader panicked".to_string())?
        .map_err(|error| error.to_string())?;
    Ok(Output {
        status,
        stdout,
        stderr,
    })
}

#[cfg(test)]
mod subprocess_timeout_tests {
    use super::wait_for_child;
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    #[cfg(unix)]
    #[test]
    fn hanging_subprocess_is_killed_at_timeout() {
        let child = Command::new("sleep")
            .arg("5")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let started = Instant::now();
        let error = wait_for_child(child, Duration::from_millis(50)).unwrap_err();
        assert!(error.contains("timed out"));
        assert!(started.elapsed() < Duration::from_secs(2));
    }
}
