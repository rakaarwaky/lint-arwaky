// PURPOSE: StdioClient — ICommandExecutorProtocol implementation via stdio (sync)
use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;

use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_executor_protocol::ICommandExecutorProtocol;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct StdioClient {
    timeout: Timeout,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICommandExecutorProtocol for StdioClient {
    fn execute_command(
        &self,
        command: PatternList,
        working_dir: FilePath,
        timeout: Option<Timeout>,
    ) -> anyhow::Result<ResponseData> {
        let timeout_val = match timeout {
            Some(d) => Duration::from_secs_f64(d.value()),
            None => Duration::from_secs_f64(self.timeout.value()),
        };
        let cmd_list: Vec<&str> = command.values.iter().map(|s| s.as_str()).collect();
        if cmd_list.is_empty() {
            anyhow::bail!("Empty command");
        }
        let mut cmd = Command::new(cmd_list[0]);
        if cmd_list.len() > 1 {
            cmd.args(&cmd_list[1..]);
        }
        cmd.current_dir(working_dir.value())
            .env("PYTHONUNBUFFERED", "1");

        let mut child = cmd
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to spawn command: {}", e))?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        // Drain stdout/stderr in threads so the child never blocks on a full
        // pipe buffer, regardless of output size.
        let stdout_handle = stdout.map(|mut h| {
            std::thread::spawn(move || {
                let mut buf = String::new();
                std::io::Read::read_to_string(&mut h, &mut buf).unwrap_or_default();
                buf
            })
        });
        let stderr_handle = stderr.map(|mut h| {
            std::thread::spawn(move || {
                let mut buf = String::new();
                std::io::Read::read_to_string(&mut h, &mut buf).unwrap_or_default();
                buf
            })
        });

        let start = std::time::Instant::now();
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) => {
                    if start.elapsed() >= timeout_val {
                        let _ = child.kill();
                        let _ = child.wait();
                        anyhow::bail!(
                            "Command timed out after {:.1}s: {}",
                            timeout_val.as_secs_f64(),
                            cmd_list.join(" ")
                        );
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(e) => {
                    let _ = child.kill();
                    return Err(anyhow::anyhow!("Failed to check command status: {}", e));
                }
            }
        };

        let stdout = match stdout_handle {
            Some(h) => h
                .join()
                .map_err(|_| anyhow::anyhow!("stdout reader thread failed"))?,
            None => String::new(),
        };
        let stderr = match stderr_handle {
            Some(h) => h
                .join()
                .map_err(|_| anyhow::anyhow!("stderr reader thread failed"))?,
            None => String::new(),
        };
        let mut meta_map = HashMap::new();
        meta_map.insert(
            "protocol".to_string(),
            serde_json::Value::String("Stdio".to_string()),
        );
        Ok(ResponseData {
            value: Some(serde_json::Value::Null),
            stdout,
            stderr,
            returncode: match status.code() {
                Some(c) => c as i64,
                None => -1,
            },
            metadata: meta_map,
        })
    }

    fn health_check(&self) -> anyhow::Result<ResponseData> {
        Ok(ResponseData::new())
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl StdioClient {
    pub fn new(timeout: Timeout) -> Self {
        Self { timeout }
    }
}
