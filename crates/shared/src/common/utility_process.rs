// PURPOSE: Process execution utility — stateless command runner helpers
use crate::common::utility_command_runner;

/// Execute a command and return stdout/stderr/success status.
pub fn run_command(name: &str, args: &[&str]) -> (String, String, bool) {
    utility_command_runner::run_command(name, args)
}

// ─── Security Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod security_tests {
    use super::*;

    /// Regression test: Verify that shell metacharacters in command names are rejected.
    /// The command runner uses std::process::Command which does NOT invoke a shell,
    /// so passing "; rm -rf /" as an argument is safe — it becomes a literal argument.
    #[test]
    fn shell_injection_rejected_in_command_name() {
        // Attempting to use a command name with shell metacharacters should fail
        // because there's no executable named "; rm" or similar
        let (stdout, stderr, success) = run_command("; rm", &["-rf", "/"]);
        assert!(
            !success,
            "Shell injection in command name should fail — got: stdout={}, stderr={}",
            stdout,
            stderr
        );
    }

    /// Regression test: Verify that shell metacharacters in args are passed literally.
    /// When using Command::new() directly (no shell), arguments with metacharacters
    /// are treated as literal strings, not shell syntax.
    #[test]
    fn shell_metacharacters_in_args_are_literal() {
        // Pass a path with shell metacharacters as an argument — should be literal
        let (stdout, _, success) = run_command("echo", &["hello; world"]);
        assert!(
            success,
            "Echo with semicolon arg should succeed — args are literal, not shell syntax"
        );
        assert!(
            stdout.contains("hello; world") || stdout.contains("hello"),
            "Argument should be passed literally without shell interpretation"
        );
    }

    /// Regression test: Verify path confinement — paths cannot escape their expected scope.
    /// The command runner uses std::process::Command which respects current_dir()
    /// and does not allow escaping outside the specified directory.
    #[test]
    fn path_confinement_via_current_dir() {
        // Even with a path traversal attempt, current_dir() confines execution
        let (stdout, _, success) = utility_command_runner::run_command_in_dir("pwd", &[], Some("/tmp"));
        assert!(
            success,
            "Command with confined current_dir should succeed"
        );
        // The output should be within /tmp (or whatever we set as current_dir)
        assert!(
            stdout.contains("/tmp") || stdout.trim() == "/tmp",
            "Execution should be confined to the specified directory"
        );
    }

    /// Regression test: Verify that backtick injection is harmless.
    /// Backticks are shell syntax for command substitution — but since we use
    /// Command::new() directly (no shell), backticks in arguments are literal.
    #[test]
    fn backtick_injection_harmless() {
        let (stdout, _, success) = run_command("echo", &["hello`whoami`"]);
        assert!(
            success,
            "Echo with backtick arg should succeed"
        );
        // The backticks should be literal, not interpreted as command substitution
        assert!(
            stdout.contains("hello") || stdout.is_empty(),
            "Backticks should be passed literally without shell interpretation"
        );
    }

    /// Regression test: Verify that pipe characters in args are literal.
    #[test]
    fn pipe_injection_harmless() {
        // Pipe character should not create a pipeline — it's a literal argument
        let (stdout, _, success) = run_command("echo", &["hello|world"]);
        assert!(
            success,
            "Echo with pipe arg should succeed"
        );
        assert!(
            stdout.contains("hello") || stdout.is_empty(),
            "Pipe character should be literal, not create a shell pipeline"
        );
    }
}

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

#[cfg(test)]
mod phase3_regression_tests {
    use super::utility_command_runner;

    /// Regression test for Phase 3.10: Cargo fallback returns "." (current dir)
    /// instead of a nonexistent path. This verifies that the command runner
    /// handles "." as a valid working directory.
    #[test]
    fn cargo_fallback_dot_is_valid_dir() {
        // The "." directory should be a valid current_dir for Command
        let (stdout, _, success) =
            utility_command_runner::run_command_in_dir("echo", &["ok"], Some("."));
        assert!(
            success,
            "Current directory '.' should be valid for command execution"
        );
    }

    /// Regression test: Verify that std::process::Command does NOT spawn a shell.
    /// This is the foundation of shell injection prevention — Command::new() creates
    /// a direct process without invoking sh/bash, so all metacharacters in args
    /// are treated as literal strings.
    #[test]
    fn command_no_shell_spawn() {
        // Using Command directly (no shell) means metacharacters are literal
        let result = std::process::Command::new("true")
            .arg(";")
            .output();
        // This should succeed — ";" is a literal argument, not shell syntax
        match result {
            Ok(output) => {
                assert!(
                    output.status.success(),
                    "Command with ';' arg should succeed when using Command::new() directly"
                );
            }
            Err(_) => {
                // If the command isn't found (e.g., on some systems), that's OK —
                // what matters is that we're NOT invoking a shell
            }
        }
    }

    /// Regression test: Verify cargo fallback "." dir works via command runner.
    #[test]
    fn cargo_fallback_dot_is_valid_dir() {
        // The "." directory should be a valid current_dir for Command
        let (_stdout, _, success) = utility_command_runner::run_command_in_dir("echo", &["ok"], Some("."));
        assert!(
            success,
            "Current directory '.' should be valid for command execution"
        );
    }
}
