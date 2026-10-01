// Unit tests — MCP tool command.
#[test]
fn mcp_tool_command_compiles() {
    let _ = std::any::type_name::<
        mcp_server_lint_arwaky::surface_mcp_tool_command::LintArwakyMcpServer,
    >();
}

// ── Issue #574: a long blocking tool call must not starve the async runtime ──
mod blocking_pool {
    use mcp_server_lint_arwaky::surface_mcp_tool_command::run_tool_blocking;
    use std::time::{Duration, Instant};

    /// Simulate one long-running `check` (collect_scan can block for seconds on
    /// a large project) running concurrently with a trivial call, and assert
    /// the trivial call's latency is not coupled to the long call's duration.
    ///
    /// The current-thread runtime is the harshest setup: with the pre-#574
    /// inline handler, the 600 ms blocking call would monopolize the runtime's
    /// only worker and the 250 ms timer below could not fire until it finished.
    /// With `spawn_blocking`, the timer fires while the scan is still running.
    #[tokio::test(flavor = "current_thread")]
    async fn long_tool_call_does_not_block_the_async_runtime() {
        let long = run_tool_blocking(move || {
            // Simulated collect_scan: synchronous, CPU-bound, slow.
            std::thread::sleep(Duration::from_millis(600));
            r#"{"status":"ok","exit_code":0}"#.to_string()
        });
        tokio::pin!(long);

        let started = Instant::now();
        let finished_early = tokio::select! {
            output = &mut long => Some(output),
            _ = tokio::time::sleep(Duration::from_millis(250)) => None,
        };
        assert!(
            finished_early.is_none(),
            "the long tool call must still be running when the trivial timer fires"
        );
        assert!(
            started.elapsed() < Duration::from_millis(500),
            "trivial call latency must not be coupled to the long call's duration"
        );

        // The long call still completes with its payload.
        let output = long.await;
        assert!(output.contains("\"exit_code\":0"));
    }

    /// A panicking handler surfaces a JSON error, not a runtime abort.
    #[tokio::test]
    async fn panicking_handler_returns_error_payload() {
        let output = run_tool_blocking(move || panic!("simulated handler panic")).await;
        assert!(output.contains("\"exit_code\":2"));
        assert!(output.contains("panicked"));
    }
}
